// Constants used for transformation
const CHAR_COUNT: usize = 3;
const FILL_WIDTH: usize = 10;
const CHAR_MASK: &str = "*";

fn main() {
    // Read input value from stdin
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Transform the value with the fixed RepliByte RedacterTransformer logic
    let new_value: String = match input.len() {
        // Only transform if the character count is greater than 3
        len if len > 3 => {
            format!(
                "{}{}",
                input.chars().take(CHAR_COUNT).collect::<String>(),
                CHAR_MASK.repeat(FILL_WIDTH - CHAR_COUNT)
            )
        }
        _ => input,
    };

    // Write transformed value to stdout (simply print)
    println!("{}", new_value);
}
