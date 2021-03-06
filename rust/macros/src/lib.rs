#[macro_export]
macro_rules! hashmap {
    ( $( $x:expr => $y:expr ),* $(,)? ) => {
        {
            let mut temp_hash_map = HashMap::new();
            $(
            temp_hash_map.insert($x, $y);
            )*
            temp_hash_map
        }
    };
}
