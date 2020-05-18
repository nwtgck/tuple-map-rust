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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
