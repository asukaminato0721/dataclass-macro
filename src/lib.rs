use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, Fields, Lit, Meta};

// 定义配置选项结构体
#[derive(Default)]
struct DataclassOptions {
    init: bool,
    repr: bool,
    eq: bool,
    order: bool,
    unsafe_hash: bool,
    frozen: bool,
    match_args: bool,
    kw_only: bool,
    slots: bool,
    weakref_slot: bool,
}

impl DataclassOptions {
    fn from_meta_list(meta_list: Punctuated<Meta, Comma>) -> Self {
        let mut options = DataclassOptions {
            init: true, // 默认值
            repr: true,
            eq: true,
            order: false,
            unsafe_hash: false,
            frozen: false,
            match_args: true,
            kw_only: false,
            slots: false,
            weakref_slot: false,
        };

        for meta in meta_list {
            match meta {
                Meta::NameValue(nv) => {
                    if let Some(ident) = nv.path.get_ident() {
                        let value = match nv.value {
                            Expr::Lit(expr_lit) => match expr_lit.lit {
                                Lit::Bool(lit_bool) => lit_bool.value(),
                                _ => panic!("Expected boolean value for option {}", ident),
                            },
                            _ => panic!("Expected literal value for option {}", ident),
                        };

                        match ident.to_string().as_str() {
                            "init" => options.init = value,
                            "repr" => options.repr = value,
                            "eq" => options.eq = value,
                            "order" => options.order = value,
                            "unsafe_hash" => options.unsafe_hash = value,
                            "kw_only" => options.kw_only = value,
                            "slots" => options.slots = value,
                            "frozen" => options.frozen = value,
                            "match_args" => options.match_args = value,
                            "weakref_slot" => options.weakref_slot = value,
                            _ => panic!("Unknown option: {}", ident),
                        }
                    }
                }
                _ => panic!("Expected name = value pair"),
            }
        }

        options
    }
}

fn has_serde_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let Ok(Meta::Path(path)) = attr.parse_args::<Meta>() {
            path.is_ident("serde")
        } else {
            false
        }
    })
}

#[proc_macro_attribute]
pub fn dataclass(args: TokenStream, input: TokenStream) -> TokenStream {
    let args =
        parse_macro_input!(args with syn::punctuated::Punctuated::<Meta, Comma>::parse_terminated);
    let mut input = parse_macro_input!(input as DeriveInput);

    let options = DataclassOptions::from_meta_list(args);

    // check if serde attribute is already present
    if !has_serde_attribute(&input.attrs) {
        // add serde derive attribute
        input.attrs.push(syn::parse_quote!(
            #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        ));
    }

    implement_dataclass(input, options)
}

fn implement_dataclass(input: DeriveInput, options: DataclassOptions) -> TokenStream {
    let struct_name = &input.ident;
    let attrs = &input.attrs;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Dataclass only works with named fields"),
        },
        _ => panic!("Dataclass only works with structs"),
    };

    let field_names: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let mut implementations = TokenStream2::new();

    // (init option)
    if options.init {
        let constructor = if options.kw_only {
            quote! {
                impl #struct_name {
                    pub fn new(#(#field_names: #field_types),*) -> Self {
                        Self {
                            #(#field_names,)*
                        }
                    }
                }
            }
        } else {
            quote! {
                impl #struct_name {
                    pub fn new(#(#field_names: #field_types),*) -> Self {
                        Self {
                            #(#field_names,)*
                        }
                    }
                }
            }
        };
        implementations.extend(constructor);
    }

    // Debug (repr option)
    if options.repr {
        let debug_impl = quote! {
            impl std::fmt::Debug for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#struct_name))
                        #(.field(stringify!(#field_names), &self.#field_names))*
                        .finish()
                }
            }
        };
        implementations.extend(debug_impl);
    }

    // (eq option)
    if options.eq {
        let eq_impl = quote! {
            impl PartialEq for #struct_name {
                fn eq(&self, other: &Self) -> bool {
                    #(self.#field_names == other.#field_names)&&*
                }
            }

            impl Eq for #struct_name {}
        };
        implementations.extend(eq_impl);
    }

    // (order option)
    if options.order {
        let ord_impl = quote! {
            impl PartialOrd for #struct_name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }

            impl Ord for #struct_name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    #(
                        if let std::cmp::Ordering::Equal = self.#field_names.cmp(&other.#field_names) {
                        } else {
                            return self.#field_names.cmp(&other.#field_names);
                        }
                    )*
                    std::cmp::Ordering::Equal
                }
            }
        };
        implementations.extend(ord_impl);
    }

    // Hash (unsafe_hash option)
    if options.unsafe_hash {
        let hash_impl = quote! {
            impl std::hash::Hash for #struct_name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    #(self.#field_names.hash(state);)*
                }
            }
        };
        implementations.extend(hash_impl);
    }

    // (frozen option)
    let struct_fields = if options.frozen {
        quote! {
            #(pub(crate) #field_names: #field_types,)*
        }
    } else {
        quote! {
            #(pub #field_names: #field_types,)*
        }
    };

    let expanded = quote! {
        #[derive(Clone)]
        #(#attrs)*
        pub struct #struct_name {
            #struct_fields
        }

        #implementations
    };

    TokenStream::from(expanded)
}
