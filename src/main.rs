const CHAR_COUNT: usize = 3;
const FILL_WIDTH: usize = 10;

fn main() {
    // Read input value from stdin
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Transform the value as you see fit (in this case we just reverse the string)
    let new_value: String = match input.len() {
        // Only transform if the character count is greater than 3
        len if len > 3 => {
            format!(
                "{}{}",
                input.chars().take(CHAR_COUNT).collect::<String>(),
                "*".repeat(FILL_WIDTH - CHAR_COUNT)
            )
        }
        _ => input,
    };

    // Write transformed value to stdout (simply print)
    println!("{}", new_value);
}
