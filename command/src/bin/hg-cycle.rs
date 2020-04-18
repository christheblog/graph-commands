use clap::{App, Arg};
use hg_command::utils;
use hg_command::version;
use hg_core::graph::VertexId;
use hg_core::algorithm::cycle;
use hg_core::iter::iter_cycle;
use hg_core::iter::iter_cycle::Cycle;


fn main() {
    let args = App::new("hg-cycle")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Identify cycles")
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .help("Use the specified directory instead of the current one")
                .default_value(".")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("girth")
                .long("girth")
                .short("g")
                .help("Compute the length of the shortest cycle of the graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count")
                .long("count")
                .short("c")
                .help("Count the number of cycles form the graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("shortest")
                .long("shortest")
                .short("s")
                .help("Return the shortest cycle of the graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("longest")
                .long("longest")
                .short("L")
                .help("Return the longest cycle of the graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("take-one")
                .long("take-one")
                .short("o")
                .help("Compute the length of the shortest cycle of a graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("take-n")
                .long("take-n")
                .short("n")
                .help("Find at most n cycles")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("take-all")
                .long("take-all")
                .short("a")
                .help("Return all the cycles from the graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("min-length")
                .long("min-length")
                .short("m")
                .help("Compute the length of the shortest cycle of a graph")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max-length")
                .long("max-length")
                .short("M")
                .help("Compute the length of the longest cycle of a graph")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();

    let graph = utils::load_graph(path).expect("Couldn't load graph");

    let girth = args.is_present("girth");
    let count = args.is_present("count");
    let shortest = args.is_present("shortest");
    let longest = args.is_present("longest");
    let take_one = args.is_present("take-one");
    let take_n = args.value_of("take-n")
        .and_then(|x| x.parse::<usize>().ok());
    let take_all = args.is_present("take-all");
    let min_length = args.value_of("min-length")
        .and_then(|x| x.parse::<usize>().ok());
    let max_length = args.value_of("max-length")
        .and_then(|x| x.parse::<usize>().ok());

    if girth {
        println!("girth: {}", format_girth(cycle::girth(&graph)));
    } else if count {
        println!("count: {}", cycle::count(&graph));
    } else if shortest {
        println!("shortest cycle: {}", format_cycle_opt(cycle::shortest(&graph).as_ref()));
    } else if longest {
        println!("longest cycle: {}", format_cycle_opt(cycle::longest(&graph).as_ref()));
    } else if take_one {
        println!("{}", format_cycle_opt(cycle::first(&graph).as_ref()));
    } else if let Some(n) = take_n {
        let cycles = cycle::take(&graph, n);
        cycles.iter().for_each(|c| println!("{}", format_cycle(c)));
    } else if take_all {
        let cycles = cycle::take_all(&graph);
        cycles.iter().for_each(|c| println!("{}", format_cycle(c)));
    } else if min_length.is_some() {
        iter_cycle::cycle_iter(&graph)
            .filter(|c| c.len() >= min_length.unwrap())
            .for_each(|c| println!("{}", format_cycle(&c)));
    } else if max_length.is_some() {
        iter_cycle::cycle_iter(&graph)
            .filter(|c| c.len() <= max_length.unwrap())
            .for_each(|c| println!("{}", format_cycle(&c)));
    }
}

fn format_girth(g: Option<usize>) -> String {
    match g {
        Some(n) => n.to_string(),
        None => "Infinity".to_string()
    }
}

fn format_cycle_length(g: Option<usize>) -> String {
    match g {
        Some(n) => n.to_string(),
        None => "N/A".to_string()
    }
}

fn format_cycle(cycle: &Cycle) -> String {
    format!["{:?}", cycle.iter().map(|VertexId(vid)| vid).collect::<Vec<&u64>>()]
}

fn format_cycle_opt(cycle: Option<&Cycle>) -> String {
    match cycle {
        Some(c) => format_cycle(c),
        None => "N/A".to_string()
    }
}
