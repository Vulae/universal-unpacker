
pub mod read_ext;
pub mod pickle;

use std::num::ParseIntError;



// https://stackoverflow.com/questions/52987181#answer-52992629
pub fn decode_hex(hex_string: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex_string.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_string[i..i + 2], 16)
        })
        .collect()
}



// TODO: Is there any way to get better error messages for this?
#[macro_export]
macro_rules! hashmap {
    () => (
        std::collections::HashMap::new()
    );
    ($(($key:expr, $value:expr)),+ $(,)?) => ({
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    });
}


