extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    use std::io;
    use std::io::prelude::*;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let key = line.unwrap();
        let key_bytes = key.trim().as_bytes();
        let mut hasher = Md5::new();

        let mut password: Vec<char> = Vec::new();
        for i in 0..std::u64::MAX {
            hasher.input(key_bytes);
            hasher.input(i.to_string().as_bytes());

            let md5 = hasher.result_str();
            if md5.starts_with("00000") {
                println!("{:?} -> {:?}", i, md5);
                password.push(md5.chars().nth(5).unwrap());

                if password.len() == 8 {
                    break;
                }
            }

            hasher.reset();
        }

        let pwd: String = password.into_iter().collect();
        println!("Password: {:?}", pwd);
    }
}
