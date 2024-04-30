extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{AttrStyle, Expr, ItemFn, Meta};
use valico::json_schema::PrimitiveType;

use proc_macro::TokenStream;
use std::collections::BTreeMap;

use crate::parameter_type::ParameterType;
use crate::return_type::ReturnType;

#[derive(Debug, Clone)]
struct DocComments {
    description: Option<String>,
    parameters: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
struct ParameterWithNoDescription {
    name: String,
    _type: ParameterType,
}

#[derive(Debug, Clone)]
struct Parameter {
    name: String,
    _type: ParameterType,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct ToolInformation {
    name: String,
    description: Option<String>,
    parameters: Vec<Parameter>,
}

impl ToolInformation {
    fn build_json_schema(&self) -> serde_json::Value {
        let mut builder = valico::json_schema::Builder::new();

        builder.type_(PrimitiveType::Object);

        if let Some(description) = &self.description {
            builder.desc(&description.clone());
        }

        builder.properties(|properties| {
            for parameter in &self.parameters {
                properties.insert(&parameter.name, |property| {
                    property.type_(
                        parameter
                            ._type
                            .to_primitive_type(),
                    );

                    if let Some(description) = &parameter.description {
                        property.desc(&description.clone());
                    }

                    // "items" for array
                    if let ParameterType::Array(item_type) =
                        parameter._type.clone()
                    {
                        property.items_schema(|items| {
                            items.type_(item_type.to_primitive_type());
                        });
                    }
                });
            }
        });

        builder.required(
            self.parameters
                .iter()
                .filter(|parameter| !parameter._type.optional())
                .map(|parameter| parameter.name.clone())
                .collect(),
        );

        builder.into_json()
    }
}

fn get_doc_comments(func: &ItemFn) -> Vec<String> {
    func.attrs
        .iter()
        .filter_map(|attr| match attr.style {
            | AttrStyle::Outer => {
                if attr
                    .meta
                    .path()
                    .is_ident("doc")
                {
                    match attr.meta.clone() {
                        | Meta::NameValue(meta) => {
                            if let Expr::Lit(lit) = meta.value {
                                Some(
                                    lit.lit
                                        .to_token_stream()
                                        .to_string()
                                        .replace("r\"", "")
                                        .replace("\"", "")
                                        .trim_start_matches(" ")
                                        .to_string(),
                                )
                            } else {
                                None
                            }
                        },
                        | _ => None,
                    }
                } else {
                    None
                }
            },
            | _ => None,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocBlockState {
    Description,
    ParametersHeader,
    Parameters,
    Otherwise,
}

impl DocBlockState {
    fn check_block(
        self,
        doc: &str,
    ) -> Self {
        match self {
            | Self::Description => {
                if doc.starts_with("# Arguments")
                    || doc.starts_with("## Arguments")
                    || doc.starts_with("# Parameters")
                    || doc.starts_with("## Parameters")
                {
                    Self::ParametersHeader
                } else {
                    Self::Description
                }
            },
            | Self::Parameters | Self::ParametersHeader => {
                if doc.starts_with("- ") || doc.starts_with("* ") {
                    Self::Parameters
                } else {
                    Self::Otherwise
                }
            },
            | Self::Otherwise => Self::Otherwise,
        }
    }
}

fn parse_doc_comments(docs: Vec<String>) -> DocComments {
    let mut description = String::new();
    let mut parameters = BTreeMap::new();

    let mut state = DocBlockState::Description;

    for doc in docs {
        state = state.check_block(&doc);

        match state {
            | DocBlockState::Description => {
                // Append description
                description.push_str(&doc);
            },
            | DocBlockState::ParametersHeader => continue,
            | DocBlockState::Parameters => {
                // Parse parameters
                let body = doc
                    .trim_start_matches("- ")
                    .trim_start_matches("* ");
                let (parameter_name, parameter_desc) = body.split_at(
                    body.find(" - ")
                        .expect("Parameter description must be in the format `'<name>' - <description>`"),
                );
                let parameter_name = parameter_name.replace("`", "");
                let parameter_desc = parameter_desc.trim_start_matches(" - ");

                parameters.insert(
                    parameter_name.clone(),
                    parameter_desc.to_string(),
                );
            },
            | DocBlockState::Otherwise => break,
        }
    }

    let description = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    DocComments {
        description,
        parameters,
    }
}

fn get_parameter_types(func: &ItemFn) -> Vec<ParameterWithNoDescription> {
    func.sig.inputs.iter().map(|input| {
        match input {
            | syn::FnArg::Typed(pat) => {
                match pat.pat.as_ref() {
                    | syn::Pat::Ident(ident) => {
                        ParameterWithNoDescription {
                            name: ident.ident.to_string(),
                            _type: ParameterType::from_syn_type(&pat.ty),
                        }
                    }
                    | _ => panic!("Tool trait requires named fields"),
                }
            }
            | _ => panic!("Tool trait can only be derived for functions with named fields."),
        }
    }).collect()
}

fn get_tool_information(func: &ItemFn) -> ToolInformation {
    let doc_comments = get_doc_comments(&func);
    let doc_comments = parse_doc_comments(doc_comments);
    let parameter_types = get_parameter_types(&func);

    let parameters = parameter_types
        .iter()
        .map(|parameter| {
            // Add parameter description if it has been found in doc comments
            if let Some((_, parameter_description)) = doc_comments
                .parameters
                .iter()
                .find(|(name, _)| *name == &parameter.name)
            {
                Parameter {
                    name: parameter.name.clone(),
                    _type: parameter._type.clone(),
                    description: Some(parameter_description.clone()),
                }
            } else {
                Parameter {
                    name: parameter.name.clone(),
                    _type: parameter._type.clone(),
                    description: None,
                }
            }
        })
        .collect();

    ToolInformation {
        name: func.sig.ident.to_string(),
        description: doc_comments.description,
        parameters,
    }
}

fn quote_definition(info: &ToolInformation) -> proc_macro2::TokenStream {
    let name = info.name.clone();
    let description = info.description.clone();
    let input_schema = info
        .build_json_schema()
        .to_string();

    if let Some(description) = description {
        quote! {
            fn definition(&self) -> clust::messages::ToolDefinition {
                clust::messages::ToolDefinition::new(
                    #name,
                    Some(#description),
                    serde_json::from_str(&#input_schema).expect("Failed to parse JSON schema of tool definition"),
                )
            }
        }
    } else {
        quote! {
            fn definition(&self) -> clust::messages::ToolDefinition {
                clust::messages::ToolDefinition::new(
                    #name,
                    None,
                    serde_json::json!(#input_schema),
                )
            }
        }
    }
}

fn quote_invoke_parameters(
    info: &ToolInformation
) -> Vec<proc_macro2::TokenStream> {
    info
        .parameters
        .iter()
        .map(|parameter| {
            let name = parameter.name.clone();
            if !parameter._type.optional() {
                quote! {
                    serde_json::from_value(
                        tool_use
                            .input
                            .get(#name)
                            .ok_or_else(|| clust::messages::ToolCallError::ParameterNotFound(#name.to_string()))?
                            .clone()
                    )
                    .map_err(|_| clust::messages::ToolCallError::ParameterParseFailed(#name.to_string()))?
                }
            } else {
                quote! {
                    serde_json::from_value(
                        tool_use
                            .input
                            .get(#name)
                            .unwrap_or(&serde_json::Value::Null)
                            .clone()
                    )
                    .map_err(|_| clust::messages::ToolCallError::ParameterParseFailed(#name.to_string()))?
                }
            }
        })
        .collect()
}

fn quote_tool_call() -> proc_macro2::TokenStream {
    quote! {
        fn call(&self, tool_use: clust::messages::ToolUse)
        -> std::result::Result<clust::messages::ToolResult, clust::messages::ToolCallError>
    }
}

fn quote_async_tool_call() -> proc_macro2::TokenStream {
    quote! {
        async fn call(&self, tool_use: clust::messages::ToolUse)
        -> std::result::Result<clust::messages::ToolResult, clust::messages::ToolCallError>
    }
}

fn quote_check_name(info: &ToolInformation) -> proc_macro2::TokenStream {
    let name = info.name.clone();

    quote! {
        if tool_use.name != #name {
            return Err(clust::messages::ToolCallError::ToolNameMismatch);
        }
    }
}

fn quote_call_with_no_value(
    func: &ItemFn,
    info: &ToolInformation,
) -> proc_macro2::TokenStream {
    let function = func.sig.ident.clone();
    let impl_invoke_parameters = quote_invoke_parameters(info);

    quote! {
        #function(
            #(
                #impl_invoke_parameters
            ),*
        );
    }
}

fn quote_call_with_value(
    func: &ItemFn,
    info: &ToolInformation,
) -> proc_macro2::TokenStream {
    let function = func.sig.ident.clone();
    let impl_invoke_parameters = quote_invoke_parameters(info);

    quote! {
        let result = #function(
            #(
                #impl_invoke_parameters
            ),*
        );
    }
}

fn quote_call_with_no_value_async(
    func: &ItemFn,
    info: &ToolInformation,
) -> proc_macro2::TokenStream {
    let function = func.sig.ident.clone();
    let impl_invoke_parameters = quote_invoke_parameters(info);

    quote! {
        #function(
            #(
                #impl_invoke_parameters
            ),*
        ).await;
    }
}

fn quote_call_with_value_async(
    func: &ItemFn,
    info: &ToolInformation,
) -> proc_macro2::TokenStream {
    let function = func.sig.ident.clone();
    let impl_invoke_parameters = quote_invoke_parameters(info);

    quote! {
        let result = #function(
            #(
                #impl_invoke_parameters
            ),*
        ).await;
    }
}

fn quote_return_no_value() -> proc_macro2::TokenStream {
    quote! {
        Ok(clust::messages::ToolResult::success_without_content(
            tool_use.id,
        ))
    }
}

fn quote_return_value() -> proc_macro2::TokenStream {
    quote! {
        Ok(clust::messages::ToolResult::success(
            tool_use.id,
            Some(format!("{}", result)),
        ))
    }
}

fn quote_return_value_with_result() -> proc_macro2::TokenStream {
    quote! {
        match result {
            | Ok(value) => {
                Ok(clust::messages::ToolResult::success(
                    tool_use.id,
                    Some(format!("{}", value)),
                ))
            },
            | Err(error) => {
                Ok(clust::messages::ToolResult::error(
                    tool_use.id,
                    Some(format!("{}", error)),
                ))
            },
        }
    }
}

fn quote_call(
    func: &ItemFn,
    info: &ToolInformation,
    return_type: ReturnType,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let impl_tool_call = if !is_async {
        quote_tool_call()
    } else {
        quote_async_tool_call()
    };
    let impl_check_name = quote_check_name(info);
    let impl_call = match return_type {
        | ReturnType::None => {
            if !is_async {
                quote_call_with_no_value(func, info)
            } else {
                quote_call_with_no_value_async(func, info)
            }
        },
        | ReturnType::Value | ReturnType::Result => {
            if !is_async {
                quote_call_with_value(func, info)
            } else {
                quote_call_with_value_async(func, info)
            }
        },
    };
    let impl_return_value = match return_type {
        | ReturnType::None => quote_return_no_value(),
        | ReturnType::Value => quote_return_value(),
        | ReturnType::Result => quote_return_value_with_result(),
    };

    quote! {
        #impl_tool_call {
            #impl_check_name
            #impl_call
            #impl_return_value
        }
    }
}

fn quote_impl_tool(struct_name: &Ident) -> proc_macro2::TokenStream {
    let struct_name = struct_name.clone();

    quote! {
        impl clust::messages::Tool for #struct_name
    }
}

fn quote_impl_async_tool(struct_name: &Ident) -> proc_macro2::TokenStream {
    let struct_name = struct_name.clone();

    quote! {
        impl clust::messages::AsyncTool for #struct_name
    }
}

fn impl_tool_for_function(
    func: &ItemFn,
    info: ToolInformation,
    return_type: ReturnType,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let struct_name = Ident::new(
        &format!("ClustTool_{}", info.name),
        Span::call_site(),
    );

    let impl_impl_tool = if !is_async {
        quote_impl_tool(&struct_name)
    } else {
        quote_impl_async_tool(&struct_name)
    };
    let impl_definition = quote_definition(&info);
    let impl_call = quote_call(func, &info, return_type, is_async);

    quote! {
        // Original function
        #func

        // Generated tool struct
        pub struct #struct_name;

        // Implement Tool or AsyncTool trait for generated tool struct
        #impl_impl_tool {
            #impl_definition
            #impl_call
        }
    }
}

pub(crate) fn impl_tool(func: &ItemFn) -> TokenStream {
    let tool_information = get_tool_information(func);
    let is_async = func.sig.asyncness.is_some();
    let return_type = ReturnType::from_syn(&func.sig.output);

    impl_tool_for_function(
        func,
        tool_information,
        return_type,
        is_async,
    )
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_doc_comments() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            fn test_function(arg1: i32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let doc_comments = get_doc_comments(&item_func);

        assert_eq!(doc_comments.len(), 4);
        assert_eq!(
            doc_comments[0],
            "A function for testing."
        );
        assert_eq!(doc_comments[1], "");
        assert_eq!(doc_comments[2], "## Arguments");
        assert_eq!(
            doc_comments[3],
            "- `arg1` - First argument."
        );
    }

    #[test]
    fn test_get_doc_comments_with_multi_args() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            /// - `arg2` - Second argument.
            fn test_function(arg1: i32, arg2: u32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let doc_comments = get_doc_comments(&item_func);

        assert_eq!(doc_comments.len(), 5);
        assert_eq!(
            doc_comments[0],
            "A function for testing."
        );
        assert_eq!(doc_comments[1], "");
        assert_eq!(doc_comments[2], "## Arguments");
        assert_eq!(
            doc_comments[3],
            "- `arg1` - First argument."
        );
        assert_eq!(
            doc_comments[4],
            "- `arg2` - Second argument."
        );
    }

    #[test]
    fn test_parse_doc_comments() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            fn test_function(arg1: i32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let doc_comments = get_doc_comments(&item_func);
        let doc_comments = parse_doc_comments(doc_comments);

        assert_eq!(
            doc_comments.description,
            Some("A function for testing.".to_string())
        );
        assert_eq!(doc_comments.parameters.len(), 1);
        assert_eq!(
            doc_comments
                .parameters
                .get("arg1")
                .unwrap(),
            "First argument."
        );
    }

    #[test]
    fn test_parse_doc_comments_with_multi_args() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            /// - `arg2` - Second argument.
            fn test_function(arg1: i32, arg2: u32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let doc_comments = get_doc_comments(&item_func);
        let doc_comments = parse_doc_comments(doc_comments);

        assert_eq!(
            doc_comments.description,
            Some("A function for testing.".to_string())
        );
        assert_eq!(doc_comments.parameters.len(), 2);
        assert_eq!(
            doc_comments
                .parameters
                .get("arg1")
                .unwrap(),
            "First argument."
        );
        assert_eq!(
            doc_comments
                .parameters
                .get("arg2")
                .unwrap(),
            "Second argument."
        );
    }

    #[test]
    fn test_get_tool_information() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            fn test_function(arg1: i32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let tool_information = get_tool_information(&item_func);

        assert_eq!(tool_information.name, "test_function");
        assert_eq!(
            tool_information.description,
            Some("A function for testing.".to_string())
        );
        assert_eq!(
            tool_information
                .parameters
                .len(),
            1
        );
        assert_eq!(
            tool_information
                .parameters
                .get(0)
                .unwrap()
                .name,
            "arg1"
        );
        assert_eq!(
            tool_information
                .parameters
                .get(0)
                .unwrap()
                ._type,
            ParameterType::Integer,
        );
        assert_eq!(
            tool_information
                .parameters
                .get(0)
                .unwrap()
                .description,
            Some("First argument.".to_string())
        );
    }

    #[test]
    fn test_get_tool_information_with_multi_args() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            /// - `arg2` - Second argument.
            fn test_function(arg1: i32, arg2: u32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let tool_information = get_tool_information(&item_func);

        assert_eq!(tool_information.name, "test_function");
        assert_eq!(
            tool_information.description,
            Some("A function for testing.".to_string())
        );
        assert_eq!(
            tool_information
                .parameters
                .len(),
            2
        );
        assert_eq!(
            tool_information
                .parameters
                .get(0)
                .unwrap()
                .name,
            "arg1"
        );
        assert_eq!(
            tool_information
                .parameters
                .get(0)
                .unwrap()
                ._type,
            ParameterType::Integer,
        );
        assert_eq!(
            tool_information
                .parameters
                .get(0)
                .unwrap()
                .description,
            Some("First argument.".to_string())
        );
        assert_eq!(
            tool_information
                .parameters
                .get(1)
                .unwrap()
                .name,
            "arg2"
        );
        assert_eq!(
            tool_information
                .parameters
                .get(1)
                .unwrap()
                ._type,
            ParameterType::Integer,
        );
        assert_eq!(
            tool_information
                .parameters
                .get(1)
                .unwrap()
                .description,
            Some("Second argument.".to_string())
        );
    }

    #[test]
    fn test_build_json_schema() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            fn test_function(arg1: i32) -> i32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let tool_information = get_tool_information(&item_func);
        let schema = tool_information.build_json_schema();

        assert_eq!(
            serde_json::to_string_pretty(&schema).unwrap(),
            r#"{
  "description": "A function for testing.",
  "properties": {
    "arg1": {
      "description": "First argument.",
      "type": "integer"
    }
  },
  "required": [
    "arg1"
  ],
  "type": "object"
}"#
        );
    }

    #[test]
    fn test_build_json_schema_with_multi_args() {
        let input = quote! {
            /// A function for testing.
            ///
            /// ## Arguments
            /// - `arg1` - First argument.
            /// - `arg2` - Second argument.
            /// - `arg3` - Third argument.
            fn test_function(
                arg1: f32,
                arg2: Option<String>,
                arg3: Vec<bool>)
            -> f32 {
                arg1
            }
        };

        let item_func = syn::parse_str::<ItemFn>(&input.to_string()).unwrap();
        let tool_information = get_tool_information(&item_func);
        let schema = tool_information.build_json_schema();

        assert_eq!(
            serde_json::to_string_pretty(&schema).unwrap(),
            r#"{
  "description": "A function for testing.",
  "properties": {
    "arg1": {
      "description": "First argument.",
      "type": "number"
    },
    "arg2": {
      "description": "Second argument.",
      "type": "string"
    },
    "arg3": {
      "description": "Third argument.",
      "items": {
        "type": "boolean"
      },
      "type": "array"
    }
  },
  "required": [
    "arg1",
    "arg3"
  ],
  "type": "object"
}"#
        );
    }
}
