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

    println!("{}", get_string(args.string, args.encode));
}

/// Base64 encodes or decodes the input string.
///
/// # Arguments
///
/// * `input`: String to encode/decode.
/// * `encode`: If true, the string will be encoded. If false, the string will be decoded.
///
/// returns: String The encoded/decoded string.
///
/// # Examples
///
/// ```
/// let encoded_string = get_string(String::from("Hello World!"), true); // SGVsbG8gV29ybGQh
///
/// let decoded_string = get_string(String::from("SGVsbG8gV29ybGQh"), false); // Hello World!
/// ```
fn get_string(input: String, encode: bool) -> String {
    return if encode {
        general_purpose::STANDARD.encode(input)
    } else {
        match general_purpose::STANDARD.decode(input) {
            Ok(r) => {
                String::from_utf8(r).unwrap()
            }
            Err(e) => {
                e.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = String::from("Hello World!");
        let expected = String::from("SGVsbG8gV29ybGQh");
        let actual = get_string(input, true);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_decode() {
        let input = String::from("SGVsbG8gV29ybGQh");
        let expected = String::from("Hello World!");
        let actual = get_string(input, false);

        assert_eq!(expected, actual);
    }
}