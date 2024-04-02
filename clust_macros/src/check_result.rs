use syn::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ReturnType {
    Value,
    Result,
}

pub(crate) fn get_return_type(ty: &Type) -> ReturnType {
    if is_result_type(ty) {
        ReturnType::Result
    } else {
        ReturnType::Value
    }
}

fn is_result_type(ty: &Type) -> bool {
    match ty {
        | Type::Path(type_path) => {
            let path_segments = &type_path.path.segments;
            path_segments.last().map_or(false, |last_segment| {
                if last_segment.ident == "Result" {
                    match &last_segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => args.args.len() == 2,
                        _ => false,
                    }
                } else if path_segments.len() >= 2 {
                    path_segments
                        .iter()
                        .rev()
                        .nth(1)
                        .map_or(false, |second_last_segment| {
                            second_last_segment.ident == "result"
                                && match &last_segment.arguments {
                                syn::PathArguments::AngleBracketed(args) => args.args.len() == 2,
                                _ => false,
                            }
                        })
                } else {
                    false
                }
            })
        },
        | _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<i32, String>;

    #[test]
    fn test_get_return_type() {
        let ty = syn::parse_str::<Type>("Result<i32, String>").unwrap();
        assert_eq!(get_return_type(&ty), ReturnType::Result);

        let ty =
            syn::parse_str::<Type>("std::result::Result<i32, String>").unwrap();
        assert_eq!(get_return_type(&ty), ReturnType::Result);

        let ty = syn::parse_str::<Type>("i32").unwrap();
        assert_eq!(get_return_type(&ty), ReturnType::Value);

        let ty: Type = syn::parse_str::<Type>("TestResult").unwrap();
        assert_eq!(get_return_type(&ty), ReturnType::Value);
    }
}
