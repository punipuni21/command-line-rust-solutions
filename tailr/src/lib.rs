use crate::TakeValue::*;
use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust tail")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress headers"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    Ok(Config {
        files,
        lines: PlusZero,
        bytes: None,
        quiet: false,
    })
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    unimplemented!()
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{parse_num, TakeValue::*};
    use pretty_assertions::assert_eq;

    // #[test]
    // fn test_count_lines_bytes() {
    //     let res = count_lines_bytes("tests/inputs/one.txt");
    //     assert!(res.is_ok());
    //     let (lines, bytes) = res.unwrap();
    //     assert_eq!(lines, 1);
    //     assert_eq!(bytes, 24);

    //     let res = count_lines_bytes("tests/inputs/twelve.txt");
    //     assert!(res.is_ok());
    //     let (lines, bytes) = res.unwrap();
    //     assert_eq!(lines, 12);
    //     assert_eq!(bytes, 63);
    // }

    // #[test]
    // fn test_get_start_index() {
    //     // +0 from an empty file (0 lines/bytes) returns None
    //     assert_eq!(get_start_index(&PlusZero, 0), None);

    //     // +0 from a nonempty file returns an index that
    //     // is one less than the number of lines/bytes
    //     assert_eq!(get_start_index(&PlusZero, 1), Some(0));

    //     // Taking 0 lines/bytes returns None
    //     assert_eq!(get_start_index(&TakeNum(0), 1), None);

    //     // Taking any lines/bytes from an empty file returns None
    //     assert_eq!(get_start_index(&TakeNum(1), 0), None);

    //     // Taking more lines/bytes than is available returns None
    //     assert_eq!(get_start_index(&TakeNum(2), 1), None);

    //     // When starting line/byte is less than total lines/bytes,
    //     // return one less than starting number
    //     assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
    //     assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
    //     assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

    //     // When starting line/byte is negative and less than total,
    //     // return total - start
    //     assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
    //     assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
    //     assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

    //     // When the starting line/byte is negative and more than the total,
    //     // return 0 to print the whole file
    //     assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    // }

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num((i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num("3.14".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
