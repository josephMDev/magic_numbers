use magic_num::magic_number;

fn generate_bits(bits: u8) -> u8 {
    // create mutable unsigned 8bit number
    let mut num = 0;
    // iterate over the bits supplied
    for i in 0..bits {
        // generate random num either 0 or 1, left shift to i position
        num = num | (magic_number() << i) as u8;
    }
    num
}


fn generate_bits_with_range(upper: u8) -> u8 {
    // bad time complexity
    loop {
        let num = generate_bits(8); // generate random 8 bit num
        if num <= upper { // check if num is within range
            return num; //return if num is within range
        }
    }
}

fn main() {

    // 0- 255 => 0000 0000 -> 1111 1111

    // 0 - 231 => 0000 0000 -> 1110 0111

    /* 
    Using a function magic_number() that returns a random unsigned byte 1 or 0, write a Rust
    (https://www.rust-lang.org/) application that will generate and print two random unsigned
    numbers: one between 0 and 255 and a second between 0 and 231.
    You must use the magic_number() function found in crate magic-num
    (https://github.com/cfilipescu/magic-num). Once complete upload your code to github.com or
    gitlab.com and share the link prior to the interview scheduled time.
     */
    let random_u255: u8 = generate_bits(8);
    println!("Random number between 0 and 255: {}", random_u255);

    let random_u231: u8 = generate_bits_with_range(231);
    println!("Random number between 0 and 231: {}", random_u231);
}
