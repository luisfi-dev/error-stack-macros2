#[cfg(test)]
mod tests {
    use error_stack_macros2::Error;

    #[test]
    fn empty_enum_works_without_display_attr() {
        #[derive(Debug, Error)]
        enum _EmptyEnumType {}
    }

    #[test]
    fn empty_enum_works_with_display_attr() {
        #[derive(Debug, Error)]
        #[display("this display attr is unnecessary")]
        enum _EmptyEnumType {}
    }

    #[test]
    #[expect(
        dead_code,
        reason = "this test requires `EnumType::Tuple`'s fields to exist, even though they won't be read"
    )]
    fn enum_works_with_display_attr_default() {
        #[derive(Debug, Error)]
        #[display("enum type")]
        enum EnumType {
            Unit,

            NamedFields {
                _length: usize,
                _is_ascii: bool,
                _inner: String,
            },

            Tuple(isize, isize, isize),
        }

        let unit = EnumType::Unit;
        assert_eq!(unit.to_string(), "enum type");

        let named_field = EnumType::NamedFields {
            _length: 5,
            _is_ascii: true,
            _inner: String::from("hello"),
        };
        assert_eq!(named_field.to_string(), "enum type");

        let tuple = EnumType::Tuple(5, 10, 15);
        assert_eq!(tuple.to_string(), "enum type");
    }

    #[test]
    fn enum_works_with_display_attr_default_and_some_variants() {
        #[derive(Debug, Error)]
        #[display("enum type")]
        enum EnumType {
            Unit,

            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFields {
                length: usize,
                is_ascii: bool,
                inner: String,
            },

            #[display(
                "tuple variant: point units in front of the origin, and with x and y coords"
            )]
            Tuple(isize, isize, isize),
        }

        let unit = EnumType::Unit;
        assert_eq!(unit.to_string(), "enum type");

        let named_field = EnumType::NamedFields {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            named_field.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );

        let tuple = EnumType::Tuple(5, 10, 15);
        assert_eq!(
            tuple.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }

    #[test]
    fn enum_works_with_display_attr_default_and_all_variants() {
        #[derive(Debug, Error)]
        #[display("enum type")]
        enum EnumType {
            #[display("unit variant")]
            Unit,

            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFields {
                length: usize,
                is_ascii: bool,
                inner: String,
            },

            #[display(
                "tuple variant: point units in front of the origin, and with x and y coords"
            )]
            Tuple(isize, isize, isize),
        }

        let unit = EnumType::Unit;
        assert_eq!(unit.to_string(), "unit variant");

        let named_field = EnumType::NamedFields {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            named_field.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );

        let tuple = EnumType::Tuple(5, 10, 15);
        assert_eq!(
            tuple.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }

    #[test]
    fn enum_works_with_display_attr_all_variants() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("unit variant")]
            Unit,

            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFields {
                length: usize,
                is_ascii: bool,
                inner: String,
            },

            #[display(
                "tuple variant: point units in front of the origin, and with x and y coords"
            )]
            Tuple(isize, isize, isize),
        }

        let unit = EnumType::Unit;
        assert_eq!(unit.to_string(), "unit variant");

        let named_field = EnumType::NamedFields {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            named_field.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );

        let tuple = EnumType::Tuple(5, 10, 15);
        assert_eq!(
            tuple.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }
}
