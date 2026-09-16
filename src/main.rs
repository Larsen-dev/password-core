use getrandom;

fn generate_aes128_private_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    getrandom::fill(&mut key).expect("OS random generator failed");
    key
}

fn encrypt_aes128_password() {}

fn main() {
    println!("Hello, world!");
}
