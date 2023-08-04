use arboard::Clipboard;
use clap::Parser;
use guid_create::GUID;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Generates a guid.
struct Args {

    /// Generates an empty guid (00000000-0000-0000-0000-000000000000).
    #[clap(short, long, action)]
    empty: bool
}

fn main() {
    let args = Args::parse();
    let mut clipboard = Clipboard::new().unwrap();
    let mut guid = String::from("00000000-0000-0000-0000-000000000000");

    if !args.empty {
        guid = GUID::rand().to_string();
    }

    clipboard.set_text(&guid).unwrap();
    println!("{} copied to clipboard.", guid);
}
