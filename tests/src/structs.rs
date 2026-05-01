#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};

    use error_stack_macros2::Error;

    #[test]
    fn unit_struct_works() {
        #[derive(Debug, Error)]
        #[display("unit struct")]
        struct UnitStructType;

        assert_eq!(UnitStructType.to_string(), "unit struct");
    }

    #[test]
    fn named_field_struct_works_without_interpolation() {
        #[derive(Debug, Error)]
        #[display("named field struct")]
        struct NamedFieldStructType {
            _length: usize,
            _is_ascii: bool,
            _inner: String,
        }

        let test_val = NamedFieldStructType {
            _length: 5,
            _is_ascii: true,
            _inner: String::from("hello"),
        };
        assert_eq!(test_val.to_string(), "named field struct");
    }

    #[test]
    fn named_field_struct_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        #[display("named field struct: has characters")]
        struct NamedFieldStructType {
            length: usize,
            _is_ascii: bool,
            inner: String,
        }

        let test_val = NamedFieldStructType {
            length: 5,
            _is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field struct: \"hello\" has 5 characters"
        );
    }

    #[test]
    fn named_field_struct_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        #[display("named field struct: has characters and is ascii")]
        struct NamedFieldStructType {
            length: usize,
            is_ascii: bool,
            inner: String,
        }

        let test_val = NamedFieldStructType {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field struct: \"hello\" has 5 characters and is ascii=true"
        );
    }

    #[test]
    fn named_field_struct_works_with_type_parameters() {
        #[derive(Debug, Error)]
        #[display("T, U")]
        struct NamedFieldStructType<T: Display, U: Debug = Vec<u8>> {
            t: T,
            u: U,
        }

        let test_val = NamedFieldStructType {
            t: String::from("string"),
            u: vec![192, 168, 0, 254],
        };
        assert_eq!(test_val.to_string(), "T = string, U = [192, 168, 0, 254]");
    }

    #[test]
    fn named_field_struct_works_with_lifetime_parameters() {
        #[derive(Debug, Error)]
        #[display("string_a, string_b, slice")]
        struct NamedFieldStructType<'a, 'b: 'a> {
            string_a: &'a str,
            string_b: &'b str,
            slice: &'b [u8],
        }

        let test_val = NamedFieldStructType {
            string_a: "string a",
            string_b: "string b",
            slice: &[192, 168, 0, 254],
        };
        assert_eq!(
            test_val.to_string(),
            "string_a = string a, string_b = string b, slice = [192, 168, 0, 254]"
        );
    }

    #[test]
    fn named_field_struct_works_with_const_parameters() {
        #[derive(Debug, Error)]
        #[display("inner")]
        struct NamedFieldStructType<const LENGTH: usize, const BYTE: u8 = 172> {
            inner: u8,
        }

        let test_val = NamedFieldStructType::<8> { inner: 8 };
        assert_eq!(test_val.to_string(), "inner = 8");
    }

    // TODO: move #[expect(redundant_lifetimes)] to type when fixed
    #[test]
    #[expect(
        redundant_lifetimes,
        reason = "this test requires a where clause with both lifetime and trait bounds"
    )]
    fn named_field_struct_works_with_where_clause() {
        const STRING: &str = "t ref";

        #[derive(Debug, Error)]
        #[display("t_ref")]
        struct NamedFieldStructType<'a, T>
        where
            'a: 'static,
            T: Debug,
        {
            t_ref: &'a T,
        }

        let test_val = NamedFieldStructType { t_ref: &STRING };
        assert_eq!(test_val.to_string(), "t_ref = \"t ref\"");
    }

    #[test]
    #[expect(
        dead_code,
        reason = "this test requires `TupleStructType`'s fields to exist, even though they won't be read"
    )]
    fn tuple_struct_works_without_interpolation() {
        #[derive(Debug, Error)]
        #[display("tuple struct")]
        struct TupleStructType(isize, isize, isize);

        let test_val = TupleStructType(5, 10, 15);
        assert_eq!(test_val.to_string(), "tuple struct");
    }

    #[test]
    #[expect(
        dead_code,
        reason = "this test requires `TupleStructType`to have multiple fields, even though not all of them will be read"
    )]
    fn tuple_struct_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        #[display("tuple struct: point with y value")]
        struct TupleStructType(isize, isize, isize);

        let test_val = TupleStructType(5, 10, 15);
        assert_eq!(test_val.to_string(), "tuple struct: point with y value 10");
    }

    #[test]
    fn tuple_struct_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        #[display(
            "tuple struct: point units in front of the origin, and with x and y coords"
        )]
        struct TupleStructType(isize, isize, isize);

        let test_val = TupleStructType(5, 10, 15);
        assert_eq!(
            test_val.to_string(),
            "tuple struct: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }
}
