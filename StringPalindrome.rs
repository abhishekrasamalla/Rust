fn is_palindrome(s: &str) -> bool {
    let s = s.trim().to_lowercase(); // Convert to lowercase and remove leading/trailing whitespace
    let reversed = s.chars().rev().collect::<String>(); // Reverse the string
    s == reversed // Check if the original string is equal to the reversed string
}

fn main() {
    let input = "Coding is an easy way";
    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
