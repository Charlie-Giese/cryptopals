extern crate base64;
use std::u8;
use self::base64::{encode};

fn main(){

    let hex = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Original Hex String: {}", hex);

    let b64 = hex_to_base64(hex);
    println!("Base64 String: {}", b64);

}

pub fn hex_to_base64(hex: String) -> String {

    // Make vector of bytes from octets
    let mut bytes = Vec::new(); //make new vector
    for i in 0..(hex.len()/2) { 
        let res = u8::from_str_radix(&hex   [2*i .. 2*i+2], 16); //this function converts string slice to integer for given base
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };

    encode(&bytes) // now convert from Vec<u8> to b64-encoded String
}  
