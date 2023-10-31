use num::BigInt;

struct KeyPair {
    public_key_p: usize,
    public_key_g: usize,
    private_key: usize, 
}

fn main() {
    let range_start = 1;
    let range_end = 50;
    let time_start = std::time::Instant::now();

    let p = get_prime(range_start, range_end);
    let g = get_prime(range_start, range_end);
    println!("took {}ms since start | p: {}, g: {}", time_start.elapsed().as_millis(), p, g);

    // Alice and Bob or whatever, just 2 clients communicating
    let client_one = KeyPair {
        public_key_p: p,
        public_key_g: g,
        private_key: get_prime(range_start, range_end),
    };
    println!("Client 1 private key: {}", client_one.private_key);

    let client_two = KeyPair {
        public_key_p: p,
        public_key_g: g,
        private_key: get_prime(range_start, range_end),
    };
    println!("Client 2 private key: {}", client_two.private_key);

    // generate shared key
    let shared_key_one = generate_shared_key(&client_one);
    println!("Client 1 shared key: {}", shared_key_one);

    let shared_key_two = generate_shared_key(&client_two);
    println!("Client 2 shared key: {}", shared_key_two);

    // generate result key (should be the same for both clients)
    let result_key_one = generate_result_key(shared_key_two, &client_one);
    println!("Client 1 result key: {}", result_key_one);

    let result_key_two = generate_result_key(shared_key_one, &client_two);
    println!("Client 2 result key: {}", result_key_two);
}

fn generate_shared_key(client: &KeyPair) -> BigInt {
    return BigInt::from(num::pow(client.public_key_g, client.private_key) % client.public_key_p);
}

fn generate_result_key(shared_key: BigInt, client: &KeyPair) -> BigInt {
    return BigInt::from(num::pow(shared_key, client.private_key) % client.public_key_p);
}

// random and large prime number
fn get_prime(range_start: usize, range_end: usize) -> usize {
    let mut prime: usize = 0;
    loop {
        prime = rand::random::<usize>() % range_end + range_start;
        if is_prime_number(prime) {
            break;
        }
    }
    return prime;
}

fn is_prime_number(number: usize) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
