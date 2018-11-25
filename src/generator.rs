extern crate rand;

use self::rand::seq::SliceRandom;

const CHARSET_SYMBOLS: &[u8] = b"!\"*#^~+-.:,;$%&/()=?{[]}\\";
const CHARSET_SMALL_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const CHARSET_LARGE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// This function generates a password with the specified length
pub fn generate_password(length: u32) -> String {
  let charset = [
    CHARSET_SMALL_LETTERS,
    CHARSET_LARGE_LETTERS,
    CHARSET_SYMBOLS,
  ]
    .concat();

  // let charset = joined_charset
  //   .iter()
  //   .map(|x| *x as char)
  //   .collect::<Vec<char>>();

  let mut rng = rand::thread_rng();
  let v = (0..length)
    .map(|_| *charset.choose(&mut rng).unwrap() as char)
    .collect::<Vec<char>>();
  let password: String = v.into_iter().collect();
  password
}