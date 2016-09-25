extern crate time;

use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use clap::ArgMatches;

#[derive(PartialEq, Eq, Debug)]
pub enum Validation {
    Valid,
    DateParseError,
    AmountParseError
}

#[derive(PartialEq, Eq, Debug)]
pub struct Entry {
    pub date_string: String,
    pub amount_string: String
}

impl Entry {
    pub fn new<S>(date_string: S, amount_string: S) -> Self
        where S: Into<String> {
        Entry { date_string: date_string.into(),
                amount_string: amount_string.into()
              }
    }

    pub fn from_line<S>(line: S) -> Self
        where S: Into<String> {
        let line_str = line.into();
        let strs = line_str.split("|")
                           .into_iter()
                           .collect::<Vec<&str>>();
        Entry::new(strs[0], strs[1])
    }

    pub fn validate(&self) -> Validation {
        if !self.valid_date() {
            return Validation::DateParseError
        }
        if !self.valid_amount() {
            return Validation::AmountParseError
        }
        return Validation::Valid
    }

    fn valid_date(&self) -> bool {
        match time::strptime(&self.date_string, "%Y-%m-%d") {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn valid_amount(&self) -> bool {
        match f64::from_str(&self.amount_string) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn date(&self) -> time::Tm {
        time::strptime(&self.date_string, "%Y-%m-%d").unwrap()
    }

    pub fn amount(&self) -> f64 {
        f64::from_str(&self.amount_string).unwrap()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}\n", self.date_string, self.amount_string)
    }
}

pub fn filepath(matches: &ArgMatches, default_path: Option<PathBuf>) -> Option<PathBuf> {
    match matches.value_of("file") {
        Some(path) => Some(PathBuf::from(path)),
        None => {
            match default_path {
                Some(path) => Some(path.join(".budget")),
                None => None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use clap::{ App, Arg };
    use std::path::PathBuf;
    use super::{ Entry, Validation, filepath };

    #[test]
    fn validate_date_string() {
        let valid_date = "2016-09-01";
        let invalid_date = "9/1/16";
        let valid_amount = "1000";

        let valid_entry = Entry::new(valid_date, valid_amount);

        let invalid_entry = Entry::new(invalid_date, valid_amount);

        assert_eq!(valid_entry.validate(), Validation::Valid);
        assert_eq!(invalid_entry.validate(), Validation::DateParseError);
    }

    #[test]
    fn validate_amount_string() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";
        let invalid_amount = "hello";

        let valid_entry = Entry::new(valid_date, valid_amount);

        let invalid_entry = Entry::new(valid_date, invalid_amount);

        assert_eq!(valid_entry.validate(), Validation::Valid);
        assert_eq!(invalid_entry.validate(), Validation::AmountParseError);
    }

    #[test]
    fn entry_formats_with_newline() {
        let valid_date = "2016-09-01";
        let valid_amount = "1000";

        let valid_entry = Entry::new(valid_date, valid_amount);

        assert_eq!(format!("{}", valid_entry), "2016-09-01|1000\n");
    }

    #[test]
    fn entry_from_line() {
        let line = "2016-01-01|1000.00".to_string();
        let entry = Entry::from_line(line);

        assert_eq!(entry.validate(), Validation::Valid);
        assert_eq!(entry.date_string, "2016-01-01");
        assert_eq!(entry.amount_string, "1000.00");
    }

    #[test]
    fn returns_default_filepath() {
        let matches = App::new("test")
                          .arg(Arg::with_name("file")
                                   .short("f")
                                   .long("file")
                                   .takes_value(true))
                          .get_matches_from(vec!["test"]);

        let default_path = PathBuf::from("/home");
        let path = filepath(&matches, Some(default_path)).unwrap();

        assert_eq!(path, PathBuf::from("/home/.budget"));
    }

    #[test]
    fn returns_provided_filepath() {
        let matches = App::new("test")
                          .arg(Arg::with_name("file")
                                   .short("f")
                                   .long("file")
                                   .takes_value(true))
                          .get_matches_from(vec!["test", "-f", "/var/budget"]);

        let default_path = PathBuf::from("/home");
        let path = filepath(&matches, Some(default_path)).unwrap();

        assert_eq!(path, PathBuf::from("/var/budget"));
    }

}
