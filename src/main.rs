// I strongly do not recomend you to use my code since it's learning project which
// means I can be wrong with my AES implementation, data transition, etc.

// Use Implementation

mod aes;
mod key_generation;

// Constants

const ASCII: [&str; 129] = [
    "\0", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\x09", "\x0a", "\x0b",
    "\x0c", "\x0d", "\x0e", "\x0f", "\x10", "\x11", "\x12", "\x13", "\x14", "\x15", "\x16", "\x17",
    "\x18", "\x19", "\x1a", "\x1b", "\x1c", "\x1d", "\x1e", "\x1f", " ", "!", "\"", "#", "$", "%",
    "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1", "2", "3", "4", "5", "6", "7", "8",
    "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C", "D", "B", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]",
    "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
    "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~", "\x7f",
]; // For future cipher text representation

fn main() -> Result<(), ()> {
    let mut input_buffer = String::new();
    match std::io::stdin().read_line(&mut input_buffer) {
        Ok(_) => (),
        Err(_) => {
            panic!("Failed to get user's input.")
        }
    };

    let key = key_generation::generate_128key();
    let input_as_bytes = input_buffer.as_bytes();
    let output_ebc_test = aes::ebc_encode(input_as_bytes, &key);
    let output_cbc_test = aes::cbc_encode(&[0u8; 16], input_as_bytes, &key);

    Ok(())
}
