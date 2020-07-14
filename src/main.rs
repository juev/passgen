extern crate clipboard;

use argh::FromArgs;
use rand::Rng;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

#[derive(FromArgs)]
/// Get arguments from commane line
struct Arguments {
    /// pass length
    #[argh(option, short = 'l', default = "24")]
    length: usize,
}

fn main() {
    let up: Arguments = argh::from_env();
    let password_len: usize = up.length;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();
    let password: String = (0..password_len)
    .map(|_| {
        let idx = rng.gen_range(0, CHARSET.len());
        CHARSET[idx] as char
    })
    .collect();
    println!("{:?}", password);
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents(password.to_owned()).unwrap();
}
