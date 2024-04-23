fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let original = "hello";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
