extern crate clap;
use clap::{App, Arg, SubCommand};

mod generator;
mod hasher;

fn main() {
  let matches = App::new("pwtools")
    .version("1.0")
    .author("Dan Lionis <lionis.dan@gmail.com>")
    .about("Password Tools")
    .subcommand(
      SubCommand::with_name("gen")
        .about("Generate a password")
        .arg(Arg::with_name("LENGTH").default_value("16").index(1)),
    ).subcommand(
      SubCommand::with_name("hash")
        .about("Hash a string")
        .arg(
          Arg::with_name("alg")
            .value_name("ALGORITHM")
            .possible_values(&["sha256", "md5", "bcrypt"])
            .required(true),
        ).arg(Arg::with_name("STRING").required(true)),
    ).get_matches();

  // gen subcommand
  if let Some(m) = matches.subcommand_matches("gen") {
    let length_str = m.value_of("LENGTH").unwrap();
    let length = length_str.parse::<u32>().unwrap();
    let pw = generator::generate_password(length);
    println!("{}", pw)
  }

  if let Some(m) = matches.subcommand_matches("hash") {
    let hash_alg = m.value_of("alg").unwrap();
    let input = m.value_of("STRING").unwrap();
    hasher::hash_password(&String::from(hash_alg), &String::from(input));
  }
}
