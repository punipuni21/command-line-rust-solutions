use chrono::{Datelike, Local, NaiveDate};
use clap::{Arg, Command};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("calr")
        .version("0.1.0")
        .author("ryo")
        .about("Rust cal")
        .get_matches();

    let today = Local::today();

    Ok(Config {
        month: None,
        year: today.year(),
        today: today.naive_local(),
    })
}

fn parse_year(year: &str) -> MyResult<i32> {
    unimplemented!();
}

fn parse_month(month: &str) -> MyResult<u32> {
    unimplemented!();
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    unimplemented!();
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{parse_int, parse_month, parse_year};
    use chrono::NaiveDate;
    #[test]
    fn test_parse_int() {
        // Parse positive int as usize
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);
        // Parse negative int as i32
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);
        // Fail on a string
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }
    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);
        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);
        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"0\" not in the range 1 through 9999"
        );
        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"10000\" not in the range 1 through 9999"
        );
        let res = parse_year("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }
    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);
        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);
        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);
        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );
        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );
        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }
    // #[test]
    // fn test_format_month() {
    //     let today = NaiveDate::from_ymd(0, 1, 1);
    //     let leap_february = vec![
    //         "   February 2020      ",
    //         "Su Mo Tu We Th Fr Sa  ",
    //         "                   1  ",
    //         " 2  3  4  5  6  7  8  ",
    //         " 9 10 11 12 13 14 15  ",
    //         "16 17 18 19 20 21 22  ",
    //         "23 24 25 26 27 28 29  ",
    //         "                      ",
    //     ];
    //     assert_eq!(format_month(2020, 2, true, today), leap_february);
    //     let may = vec![
    //         "        May           ",
    //         "Su Mo Tu We Th Fr Sa  ",
    //         "                1  2  ",
    //         " 3  4  5  6  7  8  9  ",
    //         "10 11 12 13 14 15 16  ",
    //         "17 18 19 20 21 22 23  ",
    //         "24 25 26 27 28 29 30  ",
    //         "31                    ",
    //     ];
    //     assert_eq!(format_month(2020, 5, false, today), may);
    //     let april_hl = vec![
    //         "     April 2021       ",
    //         "Su Mo Tu We Th Fr Sa  ",
    //         "             1  2  3  ",
    //         " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
    //         "11 12 13 14 15 16 17  ",
    //         "18 19 20 21 22 23 24  ",
    //         "25 26 27 28 29 30     ",
    //         "                      ",
    //     ];
    //     let today = NaiveDate::from_ymd(2021, 4, 7);
    //     assert_eq!(format_month(2021, 4, true, today), april_hl);
    // }
    // #[test]
    // fn test_last_day_in_month() {
    //     assert_eq!(last_day_in_month(2020, 1), NaiveDate::from_ymd(2020, 1, 31));
    //     assert_eq!(last_day_in_month(2020, 2), NaiveDate::from_ymd(2020, 2, 29));
    //     assert_eq!(last_day_in_month(2020, 4), NaiveDate::from_ymd(2020, 4, 30));
    // }
}
