fn is_prime(num: u32) -> bool {
    let limit = (num as f64).sqrt() as u32;
    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn print_primes_up_to(num: u32) {
    for i in 2..=num {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn main() {
    let input = 20;
    print_primes_up_to(input);
}
