use clap::{App, Arg};

fn main() {
    let matches = App::new("Log Log Akita")
        .arg(Arg::new("include")
                .about("include pattern") // Displayed when showing help info
                .takes_value(true) // MUST be set to true in order to be an "option" argument
                .short('i') // This argument is triggered with "-i"
                .long("include") // This argument is triggered with "--input"
                .multiple_occurrences(true)) // Set to true if you wish to allow multiple occurrences)
        .get_matches();

    matches.values_of("include")
        .map(|includes| includes.map(|v| PatternFilter::new(FilterMode::Includes, v.to_owned())));
}