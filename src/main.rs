use ferris_says::say;
use std::io;

fn main() {
    let stdout = io::stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = io::BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
