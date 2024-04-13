use std::fmt::Display;
use syn::{
    GenericArgument, PathArguments, Type, TypeArray, TypeParen, TypeSlice,
};
use valico::json_schema::PrimitiveType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ParameterType {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array(Box<ParameterType>),
    Option(Box<ParameterType>),
    //Enum(Vec<String>), TODO:
    Object,
}

impl Default for ParameterType {
    fn default() -> Self {
        Self::Null
    }
}

impl Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            | ParameterType::Null => write!(f, "null"),
            | ParameterType::Boolean => write!(f, "boolean"),
            | ParameterType::Integer => write!(f, "integer"),
            | ParameterType::Number => write!(f, "number"),
            | ParameterType::String => write!(f, "string"),
            | ParameterType::Array(inner) => {
                write!(f, "array of {}", inner)
            },
            | ParameterType::Option(inner) => {
                write!(f, "option of {}", inner)
            },
            | ParameterType::Object => write!(f, "object"),
        }
    }
}

impl ParameterType {
    pub(crate) fn from_syn_type(ty: &Type) -> Self {
        match ty {
            | Type::Path(type_path) => {
                let path_segments = &type_path.path.segments;
                if let Some(first) = path_segments.first() {
                    if first.ident == "Option" {
                        if let PathArguments::AngleBracketed(args) =
                            first.arguments.clone()
                        {
                            if let Some(arg) = args.args.last() {
                                if let GenericArgument::Type(ty) = arg {
                                    return Self::Option(Box::new(
                                        ParameterType::from_syn_type(ty),
                                    ));
                                }
                            }
                        }
                    }

                    if first.ident == "Vec" {
                        if let PathArguments::AngleBracketed(args) =
                            first.arguments.clone()
                        {
                            if let Some(arg) = args.args.last() {
                                if let GenericArgument::Type(ty) = arg {
                                    return Self::Array(Box::new(
                                        ParameterType::from_syn_type(ty),
                                    ));
                                }
                            }
                        }
                    }
                }

                path_segments
                    .last()
                    .map_or(
                        Self::Object,
                        |last_segment| match last_segment
                            .ident
                            .to_string()
                            .as_str()
                        {
                            | "i8" | "i16" | "i32" | "i64" | "i128"
                            | "isize" | "u8" | "u16" | "u32" | "u64"
                            | "u128" | "usize" => Self::Integer,
                            | "f32" | "f64" => Self::Number,
                            | "bool" => Self::Boolean,
                            | "String" => Self::String,
                            | _ => Self::Object,
                        },
                    )
            },
            // Fixed array type like [T; N]
            | Type::Array(TypeArray {
                elem: element_type,
                ..
            }) => Self::Array(Box::new(Self::from_syn_type(
                element_type.as_ref(),
            ))),
            // Slice type like [T]
            | Type::Slice(slice_type) => {
                // [str]
                if Self::is_string_slice(slice_type) {
                    return Self::String;
                }

                // Other slices
                Self::Array(Box::new(Self::from_syn_type(
                    slice_type.elem.as_ref(),
                )))
            },
            // Parenthesized type like (T)
            | Type::Paren(TypeParen {
                elem: element_type,
                ..
            }) => Self::from_syn_type(element_type.as_ref()),
            // Pointer type like *const T or *mut T
            | Type::Ptr(pointer_type) => {
                Self::from_syn_type(pointer_type.elem.as_ref())
            },
            // Reference type like &T or &mut T
            | Type::Reference(reference_type) => {
                Self::from_syn_type(reference_type.elem.as_ref())
            },
            // Tuple type or other Object types
            | _ => Self::Object,
        }
    }

    fn is_string_slice(ty: &TypeSlice) -> bool {
        if let Type::Path(type_path) = ty.elem.as_ref() {
            let path_segments = &type_path.path.segments;
            if let Some(last) = path_segments.last() {
                if last.ident == "str" {
                    return true;
                }
            }
        }

        false
    }

    pub(crate) fn to_primitive_type(&self) -> PrimitiveType {
        match self {
            | ParameterType::Null => PrimitiveType::Null,
            | ParameterType::Boolean => PrimitiveType::Boolean,
            | ParameterType::Integer => PrimitiveType::Integer,
            | ParameterType::Number => PrimitiveType::Number,
            | ParameterType::String => PrimitiveType::String,
            | ParameterType::Array(_) => PrimitiveType::Array,
            | ParameterType::Option(inner) => inner.to_primitive_type(),
            | ParameterType::Object => PrimitiveType::Object,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("i8").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("i16").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("i32").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("i64").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("i128").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("isize").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("u8").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("u16").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("u32").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("u64").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("u128").unwrap()
            ),
            ParameterType::Integer
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("usize").unwrap()
            ),
            ParameterType::Integer
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("f32").unwrap()
            ),
            ParameterType::Number
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("f64").unwrap()
            ),
            ParameterType::Number
        );
    }

    #[test]
    fn boolean() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("bool").unwrap()
            ),
            ParameterType::Boolean
        );
    }

    #[test]
    fn string() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("String").unwrap()
            ),
            ParameterType::String
        );
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("&[str]").unwrap()
            ),
            ParameterType::String
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("Vec<i32>").unwrap()
            ),
            ParameterType::Array(Box::new(ParameterType::Integer))
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("&[i32]").unwrap()
            ),
            ParameterType::Array(Box::new(ParameterType::Integer))
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("Option<i32>").unwrap()
            ),
            ParameterType::Option(Box::new(ParameterType::Integer))
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("Option<bool>").unwrap()
            ),
            ParameterType::Option(Box::new(ParameterType::Boolean))
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("Option<String>").unwrap()
            ),
            ParameterType::Option(Box::new(ParameterType::String))
        );
    }

    #[test]
    fn object() {
        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("JsonValueType").unwrap()
            ),
            ParameterType::Object,
        );

        assert_eq!(
            ParameterType::from_syn_type(
                &syn::parse_str::<Type>("(u32, bool)").unwrap()
            ),
            ParameterType::Object,
        );
    }
}
