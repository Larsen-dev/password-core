// Use Implementations

use getrandom::fill;

// Public functions

pub fn generate_128key() -> [u8; 16] {
    let mut key_buffer = [0u8; 16];
    match fill(&mut key_buffer) {
        Ok(_) => key_buffer,
        Err(msg) => {
            panic!("Failed to generate key: {}", msg);
        }
    }
}
