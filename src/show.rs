use std::fmt;
use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader };
use std::path::Path;
use std::str::FromStr;
use clap::ArgMatches;

use base::Entry;

#[derive(PartialEq, Eq, Debug)]
struct Delta<'a> {
    start: &'a Entry,
    end: &'a Entry
}

impl<'a> Delta<'a> {
    fn new(start: &'a Entry, end: &'a Entry) -> Self {
        Delta { start: start, end: end }
    }

    fn delta(&self) -> f64 {
        self.end.amount() - self.start.amount()
    }
}

impl<'a> fmt::Display for Delta<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}: {} -> {} | {}", self.start.date_string,
                                             self.end.date_string,
                                             self.start.amount_string,
                                             self.end.amount_string,
                                             self.delta())
    }
}

pub fn run_show(data_path: &Path, matches: &ArgMatches) -> bool {
    if let Some(submatches) = matches.subcommand_matches("show") {
        if let Some(entries) =  read_file(data_path) {
            for delta in delta_by_line(filter_entries(&entries, &submatches)) {
                println!("{}", delta);
            }
            return true
        }
    }
    false
}

fn read_file(file_path: &Path) -> Option<Vec<Entry>> {
    match OpenOptions::new()
                      .read(true)
                      .open(file_path) {
        Ok(f) => {
            let reader = BufReader::new(f);
            Some(reader.lines()
                       .filter_map(|l|
                            match l {
                                Ok(line) => Some(Entry::from_line(line)),
                                Err(_) => None
                            }
                        )
                       .collect::<Vec<Entry>>())
        }
        Err(_) => None
    }
}

fn filter_entries<'a>(entries: &'a [Entry], submatches: &ArgMatches) -> &'a [Entry] {
    if let Some(interval) = submatches.value_of("num") {
        match usize::from_str(interval) {
            Ok(n) => {
                entries.split_at(entries.len().checked_sub(n).unwrap()).1
            }
            Err(_) => {
                entries
            }
        }
    } else {
        entries
    }
}

fn delta_by_line<'a>(entries: &'a [Entry]) -> Vec<Delta<'a>> {
    entries.windows(2)
           .map(|es| Delta::new(&es[0], &es[1]))
           .collect::<Vec<Delta<'a>>>()

}

#[cfg(test)]
mod test {
    use std::path::Path;
    use clap::{ Arg, App };
    use base::Entry;
    use super::{ Delta,
                 read_file,
                 filter_entries,
                 delta_by_line
               };

    #[test]
    fn builds_entries_from_file() {
        // 2016-01-01|1000.00
        // 2016-02-01|2000.00
        let test_file = Path::new("./test_data/build_entries_from_file");
        let entries = read_file(&test_file).unwrap();

        let expected = vec![Entry::new("2016-01-01", "1000.00"),
                            Entry::new("2016-02-01", "2000.00")
                           ];

        assert_eq!(entries, expected);
    }

    #[test]
    fn returns_the_last_n_entries() {
        let entries = vec![Entry::new("2016-09-01", "1000"),
                           Entry::new("2016-10-01", "1200"),
                           Entry::new("2016-11-01", "1100"),
                           Entry::new("2016-12-01", "1300")
                          ];

        let matches = App::new("test")
                          .arg(Arg::with_name("num")
                                   .short("n")
                                   .long("number")
                                   .takes_value(true))
                          .get_matches_from(vec!["test", "-n", "2"]);

        let filtered = filter_entries(&entries, &matches);

        assert_eq!(filtered, entries.split_at(2).1);
    }

    #[test]
    fn delta_calculates_difference_between_entries() {
        let entry_1 = Entry::new("2016-10-01", "1200");
        let entry_2 = Entry::new("2016-11-01", "1100");

        let delta = Delta::new(&entry_1, &entry_2);

        assert_eq!(delta.delta(), -100.0)
    }

    #[test]
    fn delta_formats_for_display() {
        let e1 = Entry::new("2016-01-01", "1000.00");
        let e2 = Entry::new("2016-02-01", "2000.00");
        let delta = Delta::new(&e1, &e2);

        assert_eq!(format!("{}", delta),
                   "2016-01-01 -> 2016-02-01: 1000.00 -> 2000.00 | 1000");
    }

    #[test]
    fn returns_differences_for_each_entry() {
        let entries = vec![Entry::new("2016-09-01", "1000"),
                           Entry::new("2016-10-01", "1200"),
                           Entry::new("2016-11-01", "1100"),
                           Entry::new("2016-12-01", "1300")
                          ];

        let differences = delta_by_line(&entries).iter()
                                                 .map(|d| d.delta())
                                                 .collect::<Vec<f64>>();

        assert_eq!(differences, vec![200.0, -100.0, 200.0]);
    }
}
