#[macro_export]
macro_rules! tuple_map {
    ( ($( $x:expr ),*), $name:ident, $y:expr ) => {
        ($(
            {let $name = $x; $y},
        )*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        let a = tuple_map!((10, "hello", 1.23), x, {
            format!("{:?}", x)
        });
        assert_eq!(a, ("10".to_string(), "\"hello\"".to_string(), "1.23".to_string()));
    }

    #[test]
    fn test2() {
        fn f1(vec: &mut Vec<i32>) -> u32 {
            vec.push(1);
            10
        }

        fn f2(vec: &mut Vec<i32>) -> &'static str {
            vec.push(2);
            "hello"
        }

        fn f3(vec: &mut Vec<i32>) -> f32 {
            vec.push(3);
            1.2
        }

        let mut vec: Vec<i32> = Vec::new();
        let a = tuple_map!((f1, f2, f3), f, {
            f(&mut vec)
        });
        assert_eq!(a, (10, "hello", 1.2));
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
