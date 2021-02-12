use clap::{App, Arg, ArgGroup};
use gc_command::arg_utils;
use gc_command::graph_utils;
use gc_command::version;
use gc_core::algorithm::cycle;
use gc_core::constraint::constraint::Constraint;
use gc_core::directed_graph::DirectedGraph;
use gc_core::graph::VertexId;
use gc_core::iter::iter_cycle;
use gc_core::iter::iter_cycle::Cycle;
use gc_core::path;
use std::convert::TryInto;

fn main() {
    let args = App::new("gc-cycle")
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
                .help("Compute the length of the shortest cycle of the graph. Doesn't allow to specify constraints")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("hamiltonian")
                .long("hamiltonian")
                .help("Find a hamiltonian cycle of the graph. Doesn't allow to specify constraints")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count")
                .long("count")
                .short("c")
                .help("Count the number of cycles matching teh constraints")
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
                .args(&["girth","hamiltonian", "count", "head", "take-n", "all", "shortest", "longest"])
                .required(true))
        // Constraints on the cycle
        .arg(
            Arg::with_name("min-length")
                .long("min-length")
                .help("Return all the cycles from the graph with a length greater than or equal to min-length")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max-length")
                .long("max-length")
                .help("Return all the cycles from the graph with a length less than or equal to max-length")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exact-length")
                .long("exact-length")
                .help("Return all the cycles from the graph with the provided length")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("min-score")
                .long("min-score")
                .help("Return all the cycles from the graph with a score greater than or equal to min-score")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max-score")
                .long("max-score")
                .help("Return all the cycles from the graph with a length less than or equal to max-score")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exact-score")
                .long("exact-score")
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
            Arg::with_name("exclude-all")
                .long("exclude-all")
                .help("Return all the cycles from the graph that not including any of the provided vertices")
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
            Arg::with_name("exclude-all-edges")
                .long("exclude-all-edges")
                .help("Return all the cycles from the graph that not including any of the provided edges")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();

    let graph = graph_utils::load_graph(path).expect("Couldn't load graph");

    let girth = args.is_present("girth");
    let hamiltonian = args.is_present("hamiltonian");
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
        .and_then(|ids| arg_utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_include);
    let exclude_all = args
        .values_of("exclude-all")
        .and_then(|ids| arg_utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_exclude);
    let include_all_edges = args
        .values_of("include-all-edges")
        .and_then(|ids| arg_utils::parse_edge_list(ids.collect()))
        .map(arg_utils::build_constraint_include_edges);
    let exclude_all_edges = args
        .values_of("exclude-all-edges")
        .and_then(|ids| arg_utils::parse_edge_list(ids.collect()))
        .map(arg_utils::build_constraint_exclude_edges);
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
    let iterator = iter_cycle::cycle_iter(&graph).filter(|cycle| {
        check(
            &graph,
            cycle,
            &constraints,
            |_: &DirectedGraph, path: &path::Path| path.size().try_into().unwrap(),
        )
    });

    if girth {
        println!("girth: {}", format_girth(cycle::girth(&graph)));
    }  else if hamiltonian {
            println!("hamiltonian: {}", format_cycle_opt(cycle::hamiltonian(&graph).as_ref()));
    }  else if count {
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

fn check<F>(graph: &DirectedGraph, cycle: &Cycle, constraints: &Vec<Constraint>, scorefn: F) -> bool
where
    F: Fn(&DirectedGraph, &path::Path) -> i64,
{
    let path = cycle.as_path();
    let score = scorefn(graph, &path);
    let scored_path = path::ScoredPath { path, score };
    constraints.iter().all(|c| c.check_complete(&scored_path))
}

// Formatter

fn format_girth(g: Option<usize>) -> String {
    match g {
        Some(0) | None => "Infinity".to_string(),
        Some(n) => n.to_string(),
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
