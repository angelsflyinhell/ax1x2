use num::BigInt;

struct KeyPair {
    public_key_p: usize,
    public_key_g: usize,
    private_key: usize, 
}

fn main() {
    println!("Diffie-Hellman key exchange");
    let time_start = std::time::Instant::now();

    let p = get_prime();
    let g = get_prime();
    println!("p: {}, g: {}", p, g);

    // Alice and Bob or whatever, just 2 clients communicating
    let client_one = KeyPair {
        public_key_p: p,
        public_key_g: g,
        private_key: get_prime(),
    };
    println!("Client 1 private key: {}", client_one.private_key);

    let client_two = KeyPair {
        public_key_p: p,
        public_key_g: g,
        private_key: get_prime(),
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

    println!("took {}ms since start", time_start.elapsed().as_millis());
}

fn generate_shared_key(client: &KeyPair) -> BigInt {
    return bigint_pow(BigInt::from(client.public_key_g), client.private_key) % BigInt::from(client.public_key_p);
}

fn generate_result_key(shared_key: BigInt, client: &KeyPair) -> BigInt {
    return bigint_pow(shared_key, client.private_key) % BigInt::from(client.public_key_p);
}

// random and large prime number
fn get_prime() -> usize {
    let mut prime: usize;
    loop {
        prime = rand::random::<i8>() as usize;
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

/*
    quick pow function according to
    https://stackoverflow.com/questions/101439/the-most-efficient-way-to-implement-an-integer-based-power-function-powint-int
*/
fn bigint_pow(mut base: BigInt, mut exponent: usize) -> BigInt {
    let mut result = BigInt::from(1);
    for _ in 0..exponent {
        if exponent & 1 as usize == 1 {
            result *= base.clone();
        }
        exponent >>= 1;
        base *= base.clone();
    }
    return result;
}
