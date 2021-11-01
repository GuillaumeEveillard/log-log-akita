mod engine;

fn main() {
    let arg_matches = clap::App::new("Log Log Akita")
        .arg(clap::Arg::new("include")
            .about("include pattern")
            .takes_value(true)
            .short('i')
            .long("include")
            .multiple_occurrences(true))
        .arg(clap::Arg::new("exclude")
            .about("exclude pattern")
            .takes_value(true)
            .short('e')
            .long("exclude")
            .multiple_occurrences(true))
        .arg(clap::Arg::new("file")
            .about("fiel or folder")
            .takes_value(true)
            .short('f')
            .long("file")
            .multiple_occurrences(true))
        .get_matches();

    let mut filters : Vec<Box<dyn engine::Filter>> = Vec::new();

    filters.extend(match arg_matches.values_of("include") {
        None => Vec::new(),
        Some(includes) => includes.map(|v| Box::new(engine::PatternFilter::new(engine::FilterMode::Includes, v)) as Box<dyn engine::Filter>).collect()
    });

    filters.extend(match arg_matches.values_of("exclude") {
        None => Vec::new(),
        Some(excludes) => excludes.map(|v| Box::new(engine::PatternFilter::new(engine::FilterMode::Excludes, v)) as Box<dyn engine::Filter>).collect()
    });

    let files = match arg_matches.values_of("file") {
        None => Vec::new(),
        Some(ff) => ff.map(|v| std::path::PathBuf::from(v)).collect()
    };

    println!("{:?}", filters);

    let engine = engine::Engine::new(files, filters);

    engine.all_lines().iter().for_each(|l| println!("{}", l));
   

}