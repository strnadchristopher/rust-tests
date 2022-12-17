use std::time::{SystemTime, UNIX_EPOCH};


fn main(){
    let start_time = get_current_time();
    

    test();

    // Get current time in milliseconds
    let end_time = get_current_time();

    // Subtract start time from end time to get total time
    let total_time = end_time - start_time;
    // Print total_time from milliseconds to seconds
    println!("Total time: {} seconds", total_time as f64 / 1000.0);
}

fn test(){
    let nth_prime: usize = 100000;
    // find our upper bound
    let (_lo, hi) = slow_primes::estimate_nth_prime(nth_prime.try_into().unwrap());

    // find the primes up to this upper bound
    let sieve = slow_primes::Primes::sieve(hi as usize);

    // (.nth is zero indexed.)
    match sieve.primes().nth(nth_prime - 1) {
        // Print 'The ' nth_prime ' prime is ' p
        Some(p) => println!("The {}th prime is {}", nth_prime, p),
        None => unreachable!(),
    }
}

fn get_current_time() -> u128{
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    return current_time;
}