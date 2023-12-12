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

    println!("{}", get_guid(args.empty));
}


/// Generates a guid.
///
/// # Arguments
///
/// * `empty`: Whether or not the guid should be empty.
///
/// returns: String The generated guid.
///
/// # Examples
///
/// ```
/// let non_empty_guid = get_guid(false); // Guid is not 00000000-0000-0000-0000-000000000000.
///
/// let empty_guid = get_guid(true); // Guid is 00000000-0000-0000-0000-000000000000.
/// ```
fn get_guid(empty: bool) -> String {
    return if empty {
        String::from("00000000-0000-0000-0000-000000000000")
    } else {
        GUID::rand().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let expected = String::from("00000000-0000-0000-0000-000000000000");
        let actual = get_guid(true);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_guid() {
        let expected = String::from("00000000-0000-0000-0000-000000000000");
        let actual = get_guid(false);

        assert_ne!(expected, actual);
    }
}