/*
Count a string appearances
 */
use std::io::stdin;

pub(crate) fn count_string() {
    const STRING: &str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    println!("Default text is {}", STRING);

    println!("Please input a string:");
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    input_string = input_string.trim().parse().unwrap();
    println!("Your text is: {}", input_string);
    let input_text_length = input_string.len();
    let mut i = 0;
    let mut count = 0;
    while i < STRING.len() - input_text_length {
        let candidate = &STRING[i..i + input_text_length];
        if candidate == input_string {
            count = count + 1;
        }
        i = i + 1;
    }
    println!("Your text appears {} times", count);
}