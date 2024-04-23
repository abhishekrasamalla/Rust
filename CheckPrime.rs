fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    
    for i in 2..=n.sqrt() as u64 {
        if n % i == 0 {
            return false; // Found a divisor, so n is not prime
        }
    }
    
    true // No divisors found, so n is prime
}

fn main() {
    let num = 23;
    
    if is_prime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
