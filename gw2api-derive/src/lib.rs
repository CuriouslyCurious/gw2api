use std::collections::HashMap;
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Ident, Span};
use syn::{self, parse_macro_input, DeriveInput, Attribute, Meta, Error};
use quote::{quote, ToTokens, TokenStreamExt};

// Ghetto uri splitting because it is good enough
fn split_uri(literal: syn::Lit) -> Result<(String, HashMap<String, String>)>  {
    let s = match &literal {
        syn::Lit::Str(s) => s.to_owned(),
        bad => return Err(Error::new_spanned(bad, "uri is not a string")),
    }.value();

    let mut queries = s.split('?');
    let mut map: HashMap<String, String> = HashMap::new();

    // First one is always the path, so if we have no queries we simply return an empty hashmap
    let path = match queries.next() {
        Some(s) => s,
        None => return Ok((s.to_string(), map)),
    };

    // TODO: Add error on missing template value
    // TODO: Add some more sanity checking on input (make sure it starts with a '/' and so on)
    for query in queries {
        let (parameter, value) = match query.split_once('=') {
            Some(s) => s,
            None => return Err(Error::new_spanned(literal, "uri is incorrectly formatted, missing a '='")),
        };
        map.insert(parameter.to_string(), value.to_string());
    }

    Ok((path.to_string(), map))
}

#[derive(PartialEq)]
enum EndpointTrait {
    Endpoint,
    ParamEndpoint,
    TwoParamEndpoint,
    MultiParamEndpoint,
}

impl ToTokens for EndpointTrait {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let t = match &self {
            EndpointTrait::Endpoint => "Endpoint",
            EndpointTrait::ParamEndpoint => "ParamEndpoint",
            EndpointTrait::TwoParamEndpoint => "TwoParamEndpoint",
            EndpointTrait::MultiParamEndpoint => "MultiParamEndpoint",
        };
        tokens.append(Ident::new(t, Span::call_site()));
    }
}

#[derive(Debug)]
struct Builder {
    pub name: Ident,
    pub path: String,
    pub queries: HashMap<String, String>,
    pub auth: bool,
    pub localised: bool,
}

type Result<T> = std::result::Result<T, Error>;

impl Default for Builder {
    fn default() -> Self {
        Self {
            name: Ident::new("a", Span::call_site()),
            path: String::default(),
            queries: HashMap::default(),
            auth: bool::default(),
            localised: bool::default(),
        }
    }
}

impl Builder {
    pub fn new(name: &Ident, attrs: &Vec<Attribute>) -> Result<Self> {
        let mut builder = Self::default();
        builder.name = name.to_owned();

        for attr in attrs {
            let nv = match attr.parse_meta()? {
                Meta::NameValue(nv) => nv,
                bad => return Err(Error::new_spanned(bad, "unknown attribute")),
            };

            if nv.path.is_ident("endpoint") {
                let (path, queries) = split_uri(nv.lit)?;
                builder.path = path;
                builder.queries = queries;
            } else if nv.path.is_ident("localised") {
                let localised = match nv.lit {
                    syn::Lit::Bool(b) => b.value(),
                    bad => return Err(Error::new_spanned(bad, "value is not boolean")),
                };
                builder.localised = localised;
            } else if nv.path.is_ident("auth") {
                let auth = match nv.lit {
                    syn::Lit::Bool(b) => b.value(),
                    bad => return Err(Error::new_spanned(bad, "value is not boolean")),
                };
                builder.auth = auth;
            }
        }

        if builder.path.len() == 0 && attrs.len() > 0 {
            return Err(Error::new_spanned(attrs.first(), "missing an 'endpoint' attribute"));
        }

        Ok(builder)
    }

    pub fn impl_methods(self: &Self, t: EndpointTrait) -> Result<TokenStream2> {
        let name = &self.name;
        // To not have to implement ToTokens we simply use separate variables
        // TODO: Add error on missing the correct number of parameters for a given Endpoint
        if !self.queries.is_empty() && t == EndpointTrait::Endpoint {
            return Err(Error::new_spanned(name, "regular endpoint cannot take queries"));
        } else if self.queries.is_empty() && t != EndpointTrait::Endpoint {
            return Err(Error::new_spanned(name, "no queries given for a parameter endpoint"));
        } else if self.queries.len() != 2 && t == EndpointTrait::TwoParamEndpoint {
            return Err(Error::new_spanned(name, "two queries required for two parameter endpoint"));
        } else if self.queries.len() <= 2 && t == EndpointTrait::MultiParamEndpoint {
            return Err(Error::new_spanned(name, "MultiParamEndpoint should take more than 2 parameters"));
        }

        let get_impl = self.impl_endpoint_method();

        let implemented = match t {
            EndpointTrait::Endpoint => {
                get_impl
            }
            _ => {
                let param_impl = self.impl_param_methods(&t);
                quote! {
                    #get_impl
                    #param_impl
                }
           }
        };

        Ok(implemented)
    }

    fn impl_endpoint_method(&self) -> TokenStream2 {
        let name = &self.name;
        let path = &self.path;
        let auth = &self.auth;
        let localised = &self.localised;

        let doc_string = format!(" Retrieves [`{}`] from the official API.", name);
        quote! {
            impl gw2api_core::endpoint::Endpoint for #name {
                #[doc = #doc_string]
                fn get(client: &gw2api_core::client::Client) -> gw2api_core::client::Result<Self> {
                    let request_builder = gw2api_core::client::RequestBuilder::new(#path)
                        .authenticated(#auth)
                        .localised(#localised);
                    client.request::<Self>(request_builder)
                }
            }
        }
    }

    fn impl_param_methods(&self, t: &EndpointTrait) -> TokenStream2 {
        let name = &self.name;
        let path = &self.path;
        let auth = &self.auth;
        let localised = &self.localised;

        // TODO: Restructure everything below to better handle trait specifics
        let mut parameters = String::new();
        for (parameter, _) in self.queries.iter() {
            parameters.push_str(&format!("{}: String, ", parameter));
        }
        // let parameters = parameters.strip_suffix(", ").unwrap();

        let mut ids_path = path.clone();
        for (key, _) in self.queries.iter() {
            let str = format!("?{}=\t", key);
            ids_path.push_str(&str);
        }
        let doc_string = format!(" Retrieves [`{}`] from the official API returning a [`Vec`] containing only the matching ids. Will be empty upon all specified ids not existing in the API.", name);

        let by_ids = quote! {
            #[doc = #doc_string]
            fn get_ids(client: &gw2api_core::client::Client, ids: &Vec<&'static str>)-> gw2api_core::client::Result<Self> {
                let request_builder = gw2api_core::client::RequestBuilder::new(#ids_path)
                    .authenticated(#auth)
                    .localised(#localised);
                client.request::<Self>(request_builder)
            }
        };

        let all_path = format!("{}?ids=all", path);
        let doc_string = format!(" Retrieves all [`{}`] from the official API returning a [`Vec`] containing all the available objects.", name);
        let all = quote! {
            #[doc = #doc_string]
            fn get_all(client: &gw2api_core::client::Client) -> gw2api_core::client::Result<Self> {
                let request_builder = gw2api_core::client::RequestBuilder::new(#all_path)
                    .authenticated(#auth)
                    .localised(#localised);
                client.request::<Self>(request_builder)
            }
        };
        quote! {
            impl gw2api_core::endpoint::#t for #name {
                #all
                #by_ids
            }
        }
    }
}

fn parse_input_ast(input_ast: &DeriveInput, t: EndpointTrait) -> std::result::Result<TokenStream2, TokenStream> {
    // TODO: Ensure there is some input or error
    let name = &input_ast.ident;
    let attrs = &input_ast.attrs;

    let builder = match Builder::new(name, attrs) {
        Ok(a) => a,
        Err(stream) => return Err(TokenStream::from(stream.into_compile_error())),
    };

    let implemented =  match builder.impl_methods(t) {
        Ok(m) => m,
        Err(stream) => return Err(TokenStream::from(stream.into_compile_error())),
    };


    Ok(implemented)
}

/// TODO: Implement bounded caching of results
#[proc_macro_derive(Endpoint, attributes(endpoint, localised, auth))]
pub fn endpoint(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let output = match parse_input_ast(&input_ast, EndpointTrait::Endpoint) {
        Ok(s) => s,
        Err(e) => return e,
    };
    TokenStream::from(output)
}

#[proc_macro_derive(ParamEndpoint, attributes(endpoint, localised, auth))]
pub fn param_endpoint(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let output = match parse_input_ast(&input_ast, EndpointTrait::ParamEndpoint) {
        Ok(s) => s,
        Err(e) => return e,
    };
    TokenStream::from(output)
}

#[proc_macro_derive(TwoParamEndpoint, attributes(endpoint, localised, auth))]
pub fn two_param_endpoint(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let output = match parse_input_ast(&input_ast, EndpointTrait::TwoParamEndpoint) {
        Ok(s) => s,
        Err(e) => return e,
    };
    TokenStream::from(output)

}#[proc_macro_derive(MultiParamEndpoint, attributes(endpoint, localised, auth))]
pub fn multi_param_endpoint(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let output = match parse_input_ast(&input_ast, EndpointTrait::MultiParamEndpoint) {
        Ok(s) => s,
        Err(e) => return e,
    };
    TokenStream::from(output)
}
