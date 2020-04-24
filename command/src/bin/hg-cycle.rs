use clap::{App, Arg, ArgGroup};
use hg_command::arg_utils;
use hg_command::utils;
use hg_command::version;
use hg_core::algorithm::cycle;
use hg_core::constraint::constraint::Constraint;
use hg_core::graph::VertexId;
use hg_core::iter::iter_cycle;
use hg_core::iter::iter_cycle::Cycle;
use hg_core::path;

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
        // Actions
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
            Arg::with_name("head")
                .long("head")
                .short("h")
                .help("Find the first cycle matching the constraints")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("take-n")
                .long("take-n")
                .short("n")
                .help("Find n cycles matching the constraints")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all")
                .long("all")
                .short("a")
                .help("Return all the cycles matching the constraints")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("shortest")
                .long("shortest")
                .short("s")
                .help("Return the shortest cycle of the graph matching the constraints")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("longest")
                .long("longest")
                .short("L")
                .help("Return the longest cycle of the graph matching the constraints")
                .required(false)
                .takes_value(false),
        )
        .group(
            ArgGroup::with_name("actions")
                .args(&["girth","count", "head", "take-n", "all", "shortest", "longest"])
                .required(true))
        // Constraints on the cycle
        .arg(
            Arg::with_name("min-length")
                .long("min-length")
                .short("m")
                .help("Return all the cycles from the graph with a length greater than or equal to min-length")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max-length")
                .long("max-length")
                .short("M")
                .help("Return all the cycles from the graph with a length less than or equal to max-length")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exact-length")
                .long("exact-length")
                .short("e")
                .help("Return all the cycles from the graph with the provided length")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("include-all")
                .long("include-all")
                .help("Return all the cycles from the graph that are including all the provided vertices")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("include-some-of")
                .long("include-some-of")
                .help("Return all the cycles from the graph that are including at least one of the provided vertices")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude-all")
                .long("exclude-all")
                .help("Return all the cycles from the graph that not including any of the provided vertices")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude-some-of")
                .long("exclude-some-of")
                .help("Return all the cycles from the graph that are not including at least one of the provided vertices")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("include-all-edges")
                .long("include-all-edges")
                .help("Return all the cycles from the graph that are including all the provided edges")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("include-some-edges")
                .long("include-some-edges")
                .help("Return all the cycles from the graph that are including at least one of the provided edges")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude-all-edges")
                .long("exclude-all-edges")
                .help("Return all the cycles from the graph that not including any of the provided edges")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude-some-edges")
                .long("exclude-some-edges")
                .help("Return all the cycles from the graph that are not including at least one of the provided edges")
                .required(false)
                .takes_value(true),
        )

        .get_matches();

    let path = args.value_of("path").unwrap();

    let graph = utils::load_graph(path).expect("Couldn't load graph");

    let girth = args.is_present("girth");
    // Action
    let count = args.is_present("count");
    let shortest = args.is_present("shortest");
    let longest = args.is_present("longest");
    let take_n = if args.is_present("head") {
        Some(1)
    } else {
        args.value_of("take-n")
            .and_then(|x| x.parse::<usize>().ok())
    };
    let take_all = args.is_present("all");
    // Constraints
    let include_all = args
        .values_of("include-all")
        .and_then(|ids| utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_include);
    let exclude_all = args
        .values_of("exclude-all")
        .and_then(|ids| utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_exclude);
    let include_all_edges = args
        .values_of("include-all-edges")
        .and_then(|ids| utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_include);
    let exclude_all_edges = args
        .values_of("exclude-all-edges")
        .and_then(|ids| utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_exclude);
    let min_length = args
        .value_of("min-length")
        .and_then(|x| x.parse::<usize>().ok())
        .map(arg_utils::build_constraint_min_length);
    let max_length = args
        .value_of("max-length")
        .and_then(|x| x.parse::<usize>().ok())
        .map(arg_utils::build_constraint_max_length);
    let exact_length = args
        .value_of("exact-length")
        .and_then(|x| x.parse::<usize>().ok())
        .map(arg_utils::build_constraint_exact_length);
    let min_score = args
        .value_of("min-score")
        .and_then(|x| x.parse::<i64>().ok())
        .map(arg_utils::build_constraint_min_score);
    let max_score = args
        .value_of("max-score")
        .and_then(|x| x.parse::<i64>().ok())
        .map(arg_utils::build_constraint_max_score);
    let exact_score = args
        .value_of("exact-score")
        .and_then(|x| x.parse::<i64>().ok())
        .map(arg_utils::build_constraint_exact_score);

    let constraints = build_all_constraints(
        include_all,
        exclude_all,
        include_all_edges,
        exclude_all_edges,
        min_length,
        max_length,
        exact_length,
        min_score,
        max_score,
        exact_score,
    );

    // Iterates on cycles, using the constraints to filter candidates
    let iterator = iter_cycle::cycle_iter(&graph).filter(|c| check(c, &constraints));

    if girth {
        println!("girth: {}", format_girth(cycle::girth(&graph)));
    } else if count {
        println!("count: {}", iterator.count());
    } else if shortest {
        println!(
            "shortest cycle: {}",
            format_cycle_opt(iterator.min_by_key(|c| c.len()).as_ref())
        );
    } else if longest {
        println!(
            "longest cycle: {}",
            format_cycle_opt(iterator.max_by_key(|c| c.len()).as_ref())
        );
    } else if let Some(n) = take_n {
        iterator
            .take(n)
            .for_each(|c| println!("{}", format_cycle(&c)));
    } else if take_all {
        iterator.for_each(|c| println!("{}", format_cycle(&c)));
    }
}

// Constraints

fn build_all_constraints(
    constraint_include_all: Option<Vec<Constraint>>,
    constraint_exclude_all: Option<Vec<Constraint>>,
    constraint_include_all_edges: Option<Vec<Constraint>>,
    constraint_exclude_all_edges: Option<Vec<Constraint>>,
    constraint_min_length: Option<Constraint>,
    constraint_max_length: Option<Constraint>,
    constraint_exact_length: Option<Vec<Constraint>>,
    constraint_min_score: Option<Constraint>,
    constraint_max_score: Option<Constraint>,
    constraint_exact_score: Option<Vec<Constraint>>,
) -> Vec<Constraint> {
    // Adding all constraints together
    let mut constraints = vec![];
    for c in constraint_include_all.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_exclude_all.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_include_all_edges.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_exclude_all_edges.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_min_length {
        constraints.push(c);
    }
    for c in constraint_max_length {
        constraints.push(c);
    }
    for c in constraint_exact_length.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_min_score {
        constraints.push(c);
    }
    for c in constraint_max_score {
        constraints.push(c);
    }
    for c in constraint_exact_score.unwrap_or(vec![]) {
        constraints.push(c);
    }
    constraints
}

fn check(cycle: &Cycle, constraints: &Vec<Constraint>) -> bool {
    let scored_path = path::ScoredPath {
        path: cycle.as_path(),
        score: 0,
    };
    constraints.iter().all(|c| c.check_complete(&scored_path))
}

// Formatter

fn format_girth(g: Option<usize>) -> String {
    match g {
        Some(0) | None => "Infinity".to_string(),
        Some(n) => n.to_string(),
    }
}

fn format_cycle_length(g: Option<usize>) -> String {
    match g {
        Some(n) => n.to_string(),
        None => "N/A".to_string(),
    }
}

fn format_cycle(cycle: &Cycle) -> String {
    format![
        "{:?}",
        cycle.iter().map(|VertexId(vid)| vid).collect::<Vec<&u64>>()
    ]
}

fn format_cycle_opt(cycle: Option<&Cycle>) -> String {
    match cycle {
        Some(c) => format_cycle(c),
        None => "N/A".to_string(),
    }
}
