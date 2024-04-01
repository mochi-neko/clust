/// Implements [`serde::Serialize`] and [`serde::Deserialize`] for an enum with corresponding string variants.
///
/// ## Arguments
/// - `$enum_name`: The name of the enum.
/// - `$($variant:ident => $str:expr),*`: The variants of the enum and their corresponding string representations.
macro_rules! impl_enum_string_serialization {
    ($enum_name:ident, $($variant:ident => $str:expr),*) => {
        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $enum_name::$variant => serializer.serialize_str($str),
                    )*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                    type Value = $enum_name;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        formatter.write_str(concat!("a string representing a ", stringify!($enum_name)))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$enum_name, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            $(
                                $str => Ok($enum_name::$variant),
                            )*
                            _ => Err(serde::de::Error::custom("invalid value for enum")),
                        }
                    }
                }

                deserializer.deserialize_str(EnumVisitor)
            }
        }
    };
}

pub(crate) use impl_enum_string_serialization;

/// Implements [`serde::Serialize`], [`serde::Deserialize`] and [`From`]
/// for an enum with corresponding struct variants by indicating the tag field.
///
/// ## Arguments
/// - `$enum_name`: The name of the enum.
/// - `$tag_field`: The name of the [`String`] field that contains the tag that indicates the variant.
/// - `$( $variant:ident($struct:ident, $tag:expr) ),*`: The variants of the enum and their corresponding structs and tags.
macro_rules! impl_enum_struct_serialization {
    ($enum_name:ident, $tag_field:ident, $( $variant:ident($struct:ident, $tag:expr) ),* ) => {
        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $enum_name::$variant(ref inner) => inner.serialize(serializer),
                    )*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;

                let tag = value.get(stringify!($tag_field)).and_then(serde_json::Value::as_str)
                    .ok_or_else(|| serde::de::Error::missing_field(stringify!($tag_field)))?;

                match tag {
                    $(
                        $tag => serde_json::from_value(value.clone())
                            .map($enum_name::$variant)
                            .map_err(serde::de::Error::custom),
                    )*
                    _ => Err(serde::de::Error::custom(format!("unknown tag: {}", tag))),
                }
            }
        }

        $(
            impl From<$struct> for $enum_name {
                fn from(item: $struct) -> Self {
                    $enum_name::$variant(item)
                }
            }
        )*
    };
}

pub(crate) use impl_enum_struct_serialization;

/// Implements [`serde::Serialize`] and [`serde::Deserialize`] for an enum with corresponding boolean variants.
///
/// ## Arguments
/// - `$enum_name`: The name of the enum.
/// - `$true_variant`: The name of the variant that corresponds to `true`.
/// - `$false_variant`: The name of the variant that corresponds to `false`.
macro_rules! impl_enum_bool_serialization {
    ($enum_name:ident, $true_variant:ident, $false_variant:ident) => {
        impl serde::Serialize for $enum_name {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    | $enum_name::$true_variant => {
                        serializer.serialize_bool(true)
                    },
                    | $enum_name::$false_variant => {
                        serializer.serialize_bool(false)
                    },
                }
            }
        }

        struct BoolVisitor;

        impl<'de> serde::de::Visitor<'de> for BoolVisitor {
            type Value = $enum_name;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a boolean")
            }

            fn visit_bool<E>(
                self,
                value: bool,
            ) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(if value {
                    $enum_name::$true_variant
                } else {
                    $enum_name::$false_variant
                })
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_bool(BoolVisitor)
            }
        }
    };
}

pub(crate) use impl_enum_bool_serialization;

/// Implements [`serde::Serialize`], [`serde::Deserialize`] and [`From`] for an enum with corresponding string or array of any struct variants.
///
/// ## Arguments
/// - `$enum_name`: The name of the enum.
/// - `$single_variant`: The name of the variant that corresponds to a single element.
/// - `$single_type`: The type of the single element. This must be a [`String`].
/// - `$array_variant`: The name of the variant that corresponds to an array of elements.
/// - `$array_type`: The type of the array elements.
macro_rules! impl_enum_with_string_or_array_serialization {
    ($enum_name:ident, $single_variant:ident($single_type:ty), $array_variant:ident($array_type:ty)) => {
        impl serde::Serialize for $enum_name {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                match self {
                    | $enum_name::$single_variant(ref single) => {
                        serde::Serialize::serialize(single, serializer)
                    },
                    | $enum_name::$array_variant(ref array) => {
                        let mut seq =
                            serializer.serialize_seq(Some(array.len()))?;
                        for element in array {
                            serde::ser::SerializeSeq::serialize_element(
                                &mut seq, element,
                            )?;
                        }
                        serde::ser::SerializeSeq::end(seq)
                    },
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                    type Value = $enum_name;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        formatter.write_str(
                            "a single element or an array of elements",
                        )
                    }

                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($enum_name::$single_variant(
                            value.to_string(),
                        ))
                    }

                    fn visit_seq<A>(
                        self,
                        mut seq: A,
                    ) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let mut array = Vec::new();
                        while let Some(element) = seq.next_element()? {
                            array.push(element);
                        }
                        Ok($enum_name::$array_variant(array))
                    }
                }

                deserializer.deserialize_any(EnumVisitor)
            }
        }

        impl From<$single_type> for $enum_name {
            fn from(item: $single_type) -> Self {
                $enum_name::$single_variant(item)
            }
        }

        impl From<Vec<$array_type>> for $enum_name {
            fn from(array: Vec<$array_type>) -> Self {
                $enum_name::$array_variant(array)
            }
        }
    };
}

pub(crate) use impl_enum_with_string_or_array_serialization;

/// Implements [`std::fmt::Display`] for a type that can be serialized.
///
/// ## Arguments
/// - `$t`: The type.
macro_rules! impl_display_for_serialize {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                let json = serde_json::to_string_pretty(self)
                    .map_err(|_| std::fmt::Error)?;
                write!(f, "{}", json)
            }
        }
    };
}

pub(crate) use impl_display_for_serialize;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_impl_enum_string_serialization() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum TestEnum {
            A, // "a"
            B, // "b"
            C, // "c"
        }

        impl_enum_string_serialization!(TestEnum, A => "a", B => "b", C => "c");

        let test = TestEnum::A;
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "\"a\"");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);

        let test = TestEnum::B;
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "\"b\"");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);

        let test = TestEnum::C;
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "\"c\"");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);
    }

    #[test]
    fn test_impl_enum_struct_serialization() {
        #[derive(Debug, Clone, PartialEq)]
        enum TestEnum {
            A(TestStructA),
            B(TestStructB),
            C(TestStructC),
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct TestStructA {
            tag: String,
            value: u32,
        }

        impl TestStructA {
            fn new(value: u32) -> Self {
                Self {
                    tag: "a".to_string(),
                    value,
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        struct TestStructB {
            tag: String,
            value: u32,
        }

        impl TestStructB {
            #[allow(dead_code)]
            fn new(value: u32) -> Self {
                Self {
                    tag: "b".to_string(),
                    value,
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        struct TestStructC {
            tag: String,
            value: u32,
            other_value: u32,
        }

        impl TestStructC {
            #[allow(dead_code)]
            fn new(
                value: u32,
                other_value: u32,
            ) -> Self {
                Self {
                    tag: "c".to_string(),
                    value,
                    other_value,
                }
            }
        }

        impl_enum_struct_serialization!(
            TestEnum,
            tag,
            A(TestStructA, "a"),
            B(TestStructB, "b"),
            C(TestStructC, "c")
        );

        let test = TestEnum::A(TestStructA::new(42));
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(
            serialized,
            "{\"tag\":\"a\",\"value\":42}"
        );

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);
    }

    #[test]
    fn test_impl_enum_bool_serialization() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum TestEnum {
            A, // true
            B, // false
        }

        impl_enum_bool_serialization!(TestEnum, A, B);

        let test = TestEnum::A;
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "true");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);

        let test = TestEnum::B;
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "false");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);
    }

    #[test]
    fn test_impl_enum_with_string_or_array_serialization() {
        #[derive(Debug, Clone, PartialEq)]
        enum TestEnum {
            Single(String),
            Array(Vec<TestStruct>),
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct TestStruct {
            value: u32,
        }

        impl_enum_with_string_or_array_serialization!(
            TestEnum,
            Single(String),
            Array(TestStruct)
        );

        let test = TestEnum::Single("42".to_string());
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "\"42\"");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);

        let test = TestEnum::Array(vec![TestStruct {
            value: 42,
        }]);
        let serialized = serde_json::to_string(&test).unwrap();
        assert_eq!(serialized, "[{\"value\":42}]");

        let deserialized: TestEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, test);
    }

    #[test]
    fn test_impl_display_for_serialize() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct TestStruct {
            value: u32,
        }

        impl_display_for_serialize!(TestStruct);

        let test = TestStruct {
            value: 42,
        };

        assert_eq!(
            test.to_string(),
            "{\n  \"value\": 42\n}"
        );
    }
}
