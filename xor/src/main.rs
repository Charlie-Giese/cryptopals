extern crate hex;
use std::u8;
use self::hex::{encode};
mod read_lines;

fn main(){

    let hex_1 = String::from("1c0111001f010100061a024b53535009181c");
    //println!("String 1: {}", hex_1);

    let hex_2 = String::from("686974207468652062756c6c277320657965");
    //println!("String 2: {}", hex_2);

    let output = hex_xor(hex_1, hex_2);
    //println!("XOR Result: {}", output);

    let encrypt_string = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    xor_decrypt(encrypt_string)

    //if let Ok(lines) = read_lines::read_lines("./hex_codes.txt") {
    //    // Consumes the iterator, returns an (Optional) String
    //    for line in lines {
    //        if let Ok(ip) = line {
    //            println!("{}", ip);
    //            xor_decrypt(ip);
    //       }
    //    }
    //}

}

pub fn b_xor(a : &Vec<u8>, b : &Vec<u8>) -> Vec<u8>{

    let output: Vec<_> = a.iter().zip(b).map(|(x, y)| x ^ y).collect();
    return output;

} 

pub fn xor_decrypt(hex : String){

    let bytes = hex_decode(hex);
    let byte_len = bytes.len();

    let mut ascii_test_chars : Vec<u8> = Vec::new();
    for i in 97..123{
        ascii_test_chars.push(i);
    }
    ascii_test_chars.push(32);

    let mut eval : Vec<i32> = Vec::new();

    for val in 0..256{

        let key : i32 = val;
        let candidate_key = key.to_ne_bytes();
        let mut key_stream : Vec<u8> = Vec::new();
        for _l in 0 .. byte_len{
            for byte in candidate_key{
                key_stream.push(byte)   
            }
        }   
        let candidate_message = b_xor(&bytes, & key_stream);
        println!("Candidate Message: {:?}", candidate_message);
        let candidate_message_hex = encode(&candidate_message);
        println!("Hex: {:?}", candidate_message_hex);
        let mut count = 0;

        for letter in &ascii_test_chars{
            for guess in &candidate_message{
                if letter == guess{
                    count += 1
                }
            }
        }
        println!("Candidate Score: {}", count);
        eval.push(count);
    }

    let index_of_max: usize = eval
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .expect("Error");

    let max_score = eval.iter().max();

    let true_key = index_of_max.to_ne_bytes();
    let mut true_key_stream : Vec<u8> = Vec::new();
    for _l in 0 .. byte_len{
        for byte in true_key{
            true_key_stream.push(byte)   
        }
    }  
    let true_message = b_xor(&bytes, & true_key_stream);

    let hex_message = encode(&true_message);
    println!("Recovered Hex String: {:?}", true_message);
    println!("Score: {:?}", max_score);


}

pub fn hex_decode(hex : String) -> Vec<u8>{

    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex   [2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    }

    return bytes
}

pub fn hex_xor(hex_1 : String, hex_2 : String) -> String{

    let mut bytes_1 = Vec::new(); //make new vector
    let mut bytes_2 = Vec::new();
    for i in 0..(hex_1.len()/2) { 
        let res_1 = u8::from_str_radix(&hex_1   [2*i .. 2*i+2], 16);
        let res_2 = u8::from_str_radix(&hex_2   [2*i .. 2*i+2], 16); //this function converts string slice to integer for given base
        match res_1 {
            Ok(v) => bytes_1.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
        match res_2 {
            Ok(v) => bytes_2.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };

    let output: Vec<_> = bytes_1.iter().zip(bytes_2).map(|(x, y)| x ^ y).collect();


    encode(&output)

}
 