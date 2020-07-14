use clap::{App, Arg};
use colored::Colorize;
use rand::Rng;
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "diceware/"]
struct Asset;

fn check_pass_len(len: isize) -> isize {
    if len < 1 {
        println!(
            "{}: Cannot use negative number as length, used default value.",
            "WARN".yellow()
        );
        return 24;
    }
    len
}

fn check_pass_number(num: isize) -> isize {
    if num < 1 || num > 20 {
        println!(
            "{}: Cannot use pass number outside 1 and 20, used default value.",
            "WARN".yellow()
        );
        return 7;
    }
    num
}

fn generate_random_passwords(
    password_count: isize,
    password_len: isize,
    digit: bool,
    symbols: bool,
) {
    let mut charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz"
        .to_string();

    if digit {
        charset.push_str("0123456789");
    };
    if symbols {
        charset.push_str(")(*&^%$#@!~");
    };

    let mut rng = rand::thread_rng();
    let mut password: String;
    for _ in 0..password_count {
        password = (0..password_len)
            .map(|_| {
                let idx = rng.gen_range(0, charset.len());
                (charset.as_bytes())[idx] as char
            })
            .collect();
        println!("{}", password.bold());
    }
}

fn generate_diceware_passwords(word_count: isize, number: isize, eff: bool) {
    let filename = if eff {
        "eff_large_wordlist.txt"
    } else {
        "diceware.wordlist.txt"
    };
    let dictionary = Asset::get(filename).unwrap();
    let mut content: Vec<&str> = std::str::from_utf8(dictionary.as_ref())
        .unwrap_or("")
        .split('\n')
        .collect();
    content.pop();
    let mut map = HashMap::new();
    for i in content {
        let tmp: Vec<&str> = i.split('\t').collect();
        map.insert(tmp[0], tmp[1]);
    }

    for _ in 0..number {
        let mut password: Vec<String> = vec![];
        for _ in 0..word_count {
            let mut rng = rand::thread_rng();
            let mut index: String = "".to_string();
            for _ in 0..5 {
                index.push_str(&rng.gen_range(1, 7).to_string()[..]);
            }
            let word = map.get(&(&index[..])).unwrap();
            password.push(word.to_string());
        }
        println!("{}", password.join("-"));
    }
}

fn main() {
    let matches = App::new("passgen")
        .version("1.0.0")
        .about("Password genrator (random vs diceware)")
        .arg(
            Arg::with_name("LENGTH")
                .short('l')
                .long("length")
                .about("Passwords length")
                .default_value("24"),
        )
        .arg(
            Arg::with_name("NUMBER")
                .short('n')
                .long("number")
                .about("Number of passwords")
                .default_value("7"),
        )
        .arg(
            Arg::with_name("digit")
                .short('d')
                .long("digit")
                .about("Using digits in passwords"),
        )
        .arg(
            Arg::with_name("symbols")
                .short('s')
                .long("symbols")
                .about("Using symbols in passwords"),
        )
        .subcommand(
            App::new("diceware")
                .about("Using diceware for passwords")
                .arg(Arg::with_name("eff").short('e').long("eff").about(
                    "Using eff dictionary for passwords, by default used official diceware dictionary",
                ))
                .arg(
                    Arg::with_name("words")
                        .short('w')
                        .long("words")
                        .default_value("7")
                        .about("Count of words in passwords"),
                )
                .arg(
                    Arg::with_name("NUMBER")
                        .short('n')
                        .long("number")
                        .about("Number of passwords")
                        .default_value("7"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("diceware") => {
            let words_count = matches
                .subcommand_matches("diceware")
                .unwrap()
                .value_of("words")
                .unwrap()
                .parse()
                .unwrap();
            let number = matches
                .subcommand_matches("diceware")
                .unwrap()
                .value_of("NUMBER")
                .unwrap()
                .parse()
                .unwrap();
            let eff = matches
                .subcommand_matches("diceware")
                .unwrap()
                .is_present("eff");
            generate_diceware_passwords(words_count, number, eff);
        }
        None => {
            let password_len = check_pass_len(matches.value_of("LENGTH").unwrap().parse().unwrap());
            let password_count =
                check_pass_number(matches.value_of("NUMBER").unwrap().parse().unwrap());
            let digit: bool = matches.is_present("digit");
            let symbols: bool = matches.is_present("symbols");
            generate_random_passwords(password_count, password_len, digit, symbols);
        }
        _ => println!("Whats are fucking your doing?"),
    }
}
