extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::crypto::md5::Md5;

enum HashAlg {
  Sha256,
  Md5,
  Bcrypt,
}

pub fn hash_password(algorithm: &String, input: &String) {
  match get_hash_alg(algorithm) {
    Some(HashAlg::Sha256) => {
      let mut hasher = Sha256::new();
      hasher.input_str(input.as_ref());
      println!("{}", hasher.result_str());
    }
    Some(HashAlg::Md5) => {
      let mut hasher = Md5::new();
      hasher.input_str(input.as_ref());
      println!("{}", hasher.result_str());
    }
    None => println!("Unknown hashing algorithm"),
    _ => {}
  }
}

fn get_hash_alg(input: &String) -> Option<HashAlg> {
  match input.as_ref() {
    "sha256" => Some(HashAlg::Sha256),
    "bcrypt" => Some(HashAlg::Bcrypt),
    "md5" => Some(HashAlg::Md5),
    _ => None,
  }
}
