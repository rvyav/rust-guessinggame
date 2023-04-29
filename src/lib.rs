use std::io;

pub fn convert_string_input_to_integer(mut input_range: String) -> i32 {
    io::stdin()
        .read_line(&mut input_range)
        .unwrap();
    let range: i32 = input_range.trim().parse().unwrap();
    range
}
