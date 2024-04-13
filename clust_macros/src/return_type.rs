use syn::Type;

/// Return type of function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ReturnType {
    /// Returns a value.
    Value,
    /// Returns a Result<T, E>.
    Result,
    /// Returns nothing.
    None,
}

impl ReturnType {
    pub(crate) fn from_syn(ty: &syn::ReturnType) -> ReturnType {
        match ty {
            | syn::ReturnType::Default => ReturnType::None,
            | syn::ReturnType::Type(_, ty) => match ty.as_ref() {
                | Type::Path(type_path) => {
                    return if Self::is_result_at_first_segment(&type_path)
                        || Self::is_result_by_full_path(&type_path)
                    {
                        ReturnType::Result
                    } else {
                        ReturnType::Value
                    }
                },
                | _ => ReturnType::Value,
            },
        }
    }

    // Result<T, E>
    fn is_result_at_first_segment(type_path: &syn::TypePath) -> bool {
        if let Some(first) = type_path
            .path
            .segments
            .first()
        {
            first.ident == "Result"
        } else {
            false
        }
    }

    // std::result::Result<T, E>
    fn is_result_by_full_path(type_path: &syn::TypePath) -> bool {
        let mut segments = type_path.path.segments.iter();
        if let Some(first) = segments.next() {
            if first.ident == "std" {
                if let Some(second) = segments.next() {
                    if second.ident == "result" {
                        if let Some(third) = segments.next() {
                            if third.ident == "Result" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Token;

    type TestResult = Result<i32, String>;

    #[test]
    fn test_get_return_type() {
        assert_eq!(
            ReturnType::from_syn(&syn::ReturnType::Default),
            ReturnType::None
        );

        let ty = syn::parse_str::<Type>("Result<i32, String>").unwrap();
        let r_array = syn::parse_str::<Token![->]>("->").unwrap();
        let return_type = syn::ReturnType::Type(r_array, Box::new(ty));
        assert_eq!(
            ReturnType::from_syn(&return_type),
            ReturnType::Result
        );

        let ty =
            syn::parse_str::<Type>("std::result::Result<i32, String>").unwrap();
        let return_type = syn::ReturnType::Type(r_array, Box::new(ty));
        assert_eq!(
            ReturnType::from_syn(&return_type),
            ReturnType::Result
        );

        let ty = syn::parse_str::<Type>("i32").unwrap();
        let return_type = syn::ReturnType::Type(r_array, Box::new(ty));
        assert_eq!(
            ReturnType::from_syn(&return_type),
            ReturnType::Value
        );

        let ty: Type = syn::parse_str::<Type>("TestResult").unwrap();
        let return_type = syn::ReturnType::Type(r_array, Box::new(ty));
        assert_eq!(
            ReturnType::from_syn(&return_type),
            ReturnType::Value
        );
    }
}
