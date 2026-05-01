#[cfg(test)]
mod tests {
    use error_stack_macros2::Error;

    #[test]
    fn unit_variant_works() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("unit variant")]
            Unit,
        }

        assert_eq!(EnumType::Unit.to_string(), "unit variant");
    }

    #[test]
    fn named_field_variant_works_without_interpolation() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("named field variant")]
            NamedFields {
                _length: usize,
                _is_ascii: bool,
                _inner: String,
            },
        }

        let test_val = EnumType::NamedFields {
            _length: 5,
            _is_ascii: true,
            _inner: String::from("hello"),
        };
        assert_eq!(test_val.to_string(), "named field variant");
    }

    #[test]
    fn named_field_variant_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("named field variant: {inner:?} has {length} characters")]
            NamedFields {
                length: usize,
                _is_ascii: bool,
                inner: String,
            },
        }

        let test_val = EnumType::NamedFields {
            length: 5,
            _is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field variant: \"hello\" has 5 characters"
        );
    }

    #[test]
    fn named_field_variant_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFields {
                length: usize,
                is_ascii: bool,
                inner: String,
            },
        }

        let test_val = EnumType::NamedFields {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );
    }

    #[test]
    fn tuple_variant_works_without_interpolation() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("tuple variant")]
            Tuple(isize, isize, isize),
        }

        let test_val = EnumType::Tuple(5, 10, 15);
        assert_eq!(test_val.to_string(), "tuple variant");
    }

    #[test]
    fn tuple_variant_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("tuple variant: point with y value")]
            Tuple(isize, isize, isize),
        }

        let test_val = EnumType::Tuple(5, 10, 15);
        assert_eq!(
            test_val.to_string(),
            "tuple variant: point with y value 10"
        );
    }

    #[test]
    fn tuple_variant_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display(
                "tuple variant: point units in front of the origin, and with x and y coords"
            )]
            Tuple(isize, isize, isize),
        }

        let test_val = EnumType::Tuple(5, 10, 15);
        assert_eq!(
            test_val.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }
}
