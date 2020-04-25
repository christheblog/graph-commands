use clap::{App, Arg};
use hg_command::graph_utils;
use hg_command::arg_utils;
use hg_command::version;
use hg_core::directed_graph::DirectedGraph;
use hg_core::graph::VertexId;
use hg_core::constraint::constraint::Constraint;
use hg_core::path::ScoredPath;

fn main() {
    let args = App::new("hg-csp")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Constrained short-path")
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
            Arg::with_name("start")
                .long("start")
                .short("s")
                .help("Starting node")
                .required(true)
                .takes_value(true)
                .max_values(1),
        )
        .arg(
            Arg::with_name("end")
                .long("end")
                .short("e")
                .help("End node")
                .required(true)
                .takes_value(true)
                .max_values(1),
        )
        .arg(
            Arg::with_name("include")
                .long("include")
                .help("Must include the following nodes")
                .required(false)
                .takes_value(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("exclude")
                .long("exclude")
                .help("Must exclude the following nodes")
                .required(false)
                .takes_value(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("ordered")
                .long("ordered")
                .help("Vertices must appear in the provided order")
                .required(false)
                .takes_value(true)
                .min_values(2),
        )
        .arg(
            Arg::with_name("include-cycle")
                .long("include-cycle")
                .help("Must include at least a cycle")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no-cycle")
                .long("no-cycle")
                .help("Must not include any cycle")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("min-length")
                .long("min-length")
                .help("Minimum number of vertices to be included")
                .required(false)
                .takes_value(true)
                .min_values(1)
                .max_values(1),
        )
        .arg(
            Arg::with_name("max-length")
                .long("max-length")
                .help("Maximum number of vertices to be included")
                .required(false)
                .takes_value(true)
                .min_values(1)
                .max_values(1),
        )
        .arg(
            Arg::with_name("exact-length")
                .long("exact-length")
                .help("Exact number of vertices to be included")
                .required(false)
                .takes_value(true)
                .min_values(1)
                .max_values(1),
        )
        .arg(
            Arg::with_name("min-score")
                .long("min-score")
                .help("Must have at least the minimum score")
                .required(false)
                .takes_value(true)
                .min_values(1)
                .max_values(1),
        )
        .arg(
            Arg::with_name("max-score")
                .long("max-score")
                .help("Must have at least the maximum score")
                .required(false)
                .takes_value(true)
                .min_values(1)
                .max_values(1),
        )
        .arg(
            Arg::with_name("exact-score")
                .long("exact-score")
                .help("Exact expected score")
                .required(false)
                .takes_value(true)
                .min_values(1)
                .max_values(1),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();

    let start_vertex = args
        .value_of("start")
        .and_then(arg_utils::parse_vertex_id)
        .map(|id| VertexId(id))
        .unwrap();
    let end_vertex = args
        .value_of("end")
        .and_then(arg_utils::parse_vertex_id)
        .map(|id| VertexId(id))
        .unwrap();

    let include = args
        .values_of("include")
        .and_then(|ids| arg_utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_include);

    let exclude = args
        .values_of("exclude")
        .and_then(|ids| arg_utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_exclude);

    let ordered = args
        .values_of("ordered")
        .and_then(|ids| arg_utils::parse_vertex_id_list(ids.collect()))
        .map(arg_utils::build_constraint_ordered);

    let include_cycle = arg_utils::option_of(args.is_present("include-cycle"),
        || arg_utils::build_constraint_include_cycle());

    let no_cycle = arg_utils::option_of(args.is_present("no-cycle"),
        || arg_utils::build_constraint_no_cycle());

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


    let graph = graph_utils::load_graph(path).expect("Couldn't load graph");

    let constraints = build_all_constraints(
        include,
        exclude,
        ordered,
        include_cycle,
        no_cycle,
        min_length,
        max_length,
        exact_length,
        min_score,
        max_score,
        exact_score,
    );
    match shortest_path_with_constraints(&graph, start_vertex, end_vertex, constraints) {
        Some(ScoredPath {
            score,
            path: shortest,
        }) => {
            println!(
                "Constrained shortest path from vertex {} to vertex {} with total cost of {}.",
                start_vertex.0, end_vertex.0, score
            );
            for vertex in shortest.to_vertex_list() {
                println!("{}", vertex.0);
            }
        }
        None => println!(
            "Vertex {} is not reachable from vertex {} within the given constraints.",
            end_vertex.0, start_vertex.0
        ),
    }
}

fn shortest_path_with_constraints(
    graph: &DirectedGraph,
    start: VertexId,
    end: VertexId,
    constraints: Vec<Constraint>,
) -> Option<ScoredPath> {
    use hg_core::search::a_star;;

    println!("Constraint that will be applied to search are: ");
    for c in &constraints {
        println!("{:?}", c);
    }
    // Searching for the shortest constrained path
    a_star::constrained_shortest_path(
        graph,
        a_star::one_weighted_edge,
        a_star::zero_heuristic,
        start,
        end,
        constraints,
    )
}

// Constraints

fn build_all_constraints(
    constraint_include: Option<Vec<Constraint>>,
    constraint_exclude: Option<Vec<Constraint>>,
    constraint_ordered: Option<Constraint>,
    constraint_include_cycle: Option<Constraint>,
    constraint_no_cycle: Option<Constraint>,
    constraint_min_length: Option<Constraint>,
    constraint_max_length: Option<Constraint>,
    constraint_exact_length: Option<Vec<Constraint>>,
    constraint_min_score: Option<Constraint>,
    constraint_max_score: Option<Constraint>,
    constraint_exact_score: Option<Vec<Constraint>>,
) -> Vec<Constraint> {
    // Adding all constraints together
    let mut constraints = vec![];
    for c in constraint_include.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_exclude.unwrap_or(vec![]) {
        constraints.push(c);
    }
    for c in constraint_ordered {
        constraints.push(c);
    }
    for c in constraint_include_cycle {
        constraints.push(c);
    }
    for c in constraint_no_cycle {
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
