fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();    }
    
    let first_string = strings[0];
    let mut longest_prefix = String::new();
    
    'outer: for (i, ch) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(i) {
                if c != ch {
                    break 'outer; 
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(ch);    }
    
    longest_prefix
}

fn main() {
    let strings = vec!["flower", "flow", "flight"];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}
