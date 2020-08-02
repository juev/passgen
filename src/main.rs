use colored::Colorize;
use rand::Rng;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(RustEmbed)]
#[folder = "diceware/"]
struct Asset;

#[derive(StructOpt)]
#[structopt(
    name = "passgen",
    version = env!("CARGO_PKG_VERSION"),
    about = "Password genrator (random vs diceware)"
)]
struct Opt {
    #[structopt(short, long, default_value = "24", help = "Passwords length")]
    length: isize,

    #[structopt(short, long, default_value = "7", help = "Number of passwords")]
    number: isize,

    #[structopt(short, long, help = "Using digits in passwords")]
    digit: bool,

    #[structopt(short, long, help = "Using symbols in passwords")]
    symbols: bool,

    #[structopt(subcommand, help = "Using diceware for passwords")]
    diceware: Option<DicewareEnum>,
}

#[derive(StructOpt)]
enum DicewareEnum {
    Diceware {
        #[structopt(
            short,
            long,
            help = "Using eff dictionary for passwords, by default used official diceware dictionary"
        )]
        eff: bool,

        #[structopt(short, long, default_value = "7", help = "Count of words in passwords")]
        words: isize,
        #[structopt(short, long, default_value = "7", help = "Number of passwords")]
        number: isize,
    },
}

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
    let opt = Opt::from_args();
    if let Some(diceware) = opt.diceware {
        match diceware {
            DicewareEnum::Diceware { eff, words, number } => {
                generate_diceware_passwords(words, number, eff)
            }
        }
    } else {
        let password_len = check_pass_len(opt.length);
        let password_count = check_pass_number(opt.number);
        let digit: bool = opt.digit;
        let symbols: bool = opt.symbols;
        generate_random_passwords(password_count, password_len, digit, symbols);
    }
}
