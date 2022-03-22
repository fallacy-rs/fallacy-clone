//! Derive macro test cases.

#[cfg(feature = "derive")]
mod tests {
    use fallacy_clone::TryClone;

    #[test]
    fn test_struct_with_named_field() {
        #[derive(PartialEq, Debug, TryClone)]
        struct Named {
            a: i32,
            b: i64,
        }

        let s = Named { a: 10, b: 20 };
        assert_eq!(s, s.try_clone().unwrap());
    }

    #[test]
    fn test_struct_with_unnamed_field() {
        #[derive(PartialEq, Debug, TryClone)]
        struct Unnamed(i32, i64);

        let s = Unnamed(10, 20);
        assert_eq!(s, s.try_clone().unwrap());
    }

    #[test]
    fn test_struct_unit() {
        #[derive(PartialEq, Debug, TryClone)]
        struct Unit;

        let s = Unit;
        assert_eq!(s, s.try_clone().unwrap());
    }

    #[test]
    fn test_struct_with_generic() {
        #[derive(PartialEq, Debug, TryClone)]
        struct Named<'a, T, const N: bool>
        where
            T: TryClone,
        {
            a: i32,
            b: T,
            c: &'a i64,
        }

        let c = 20;
        let s = Named::<'_, _, true> {
            a: 10,
            b: "hello",
            c: &c,
        };

        assert_eq!(s, s.try_clone().unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(PartialEq, Debug, TryClone)]
        enum Enum {
            Named { a: i32, b: i64 },
            Unnamed(i32, i64),
            Unit,
        }

        let s = Enum::Named { a: 10, b: 20 };
        assert_eq!(s, s.try_clone().unwrap());

        let s = Enum::Unnamed(10, 20);
        assert_eq!(s, s.try_clone().unwrap());

        let s = Enum::Unit;
        assert_eq!(s, s.try_clone().unwrap());
    }

    #[test]
    fn test_enum_with_generic() {
        #[derive(PartialEq, Debug, TryClone)]
        enum Enum<'a, T, const N: bool>
        where
            T: TryClone,
        {
            Named(T),
            Unnamed(i32, &'a i64),
            Unit,
        }

        let s = Enum::<_, false>::Named("hello");
        assert_eq!(s, s.try_clone().unwrap());

        let s = Enum::<'_, i32, true>::Unnamed(10, &20);
        assert_eq!(s, s.try_clone().unwrap());

        let s = Enum::<'_, i64, true>::Unit;
        assert_eq!(s, s.try_clone().unwrap());
    }
}
