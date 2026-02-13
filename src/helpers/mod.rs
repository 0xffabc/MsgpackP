#[macro_export]
macro_rules! array {
    (($($value:expr),*)) => {{
        let mut buffer = Vec::new();

        let packet = vec![$($value),*];

        packet.write_to(&mut buffer).unwrap();

        buffer
    }};
}

#[macro_export]
macro_rules! map {
    ($( $key:expr => $value:expr),* ) => {{
        let mut buffer = Vec::new();

        let packet = vec![$($key),*];

        packet.write_to(&mut buffer).unwrap();

        buffer
    }};
}
