use base64::{Engine as _, engine::general_purpose};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Encodes or Decodes the input string. Must use either 'encode' or 'decode' flag.
struct Args {

    /// String to encode/decode.
    string: String,

    /// Decodes base64 encoded string
    #[clap(short, long, action)]
    decode: bool,

    /// Encodes string with base64
    #[clap(short, long, action)]
    encode: bool
}

fn main() {
    let args = Args::parse();

    if !args.encode && !args.decode {
        eprintln!("Must use either --encode or --decode flag.");
        return;
    }

    if args.encode {
        println!("{}", general_purpose::STANDARD.encode(args.string));
    } else if args.decode {
        match general_purpose::STANDARD.decode(args.string) {
            Ok(r) => {
                println!("{}", String::from_utf8(r).unwrap());
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}