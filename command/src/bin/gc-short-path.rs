use clap::{App, Arg};
use gc_command::arg_utils;
use gc_command::graph_utils;
use gc_command::version;
use gc_core::directed_graph::DirectedGraph;
use gc_core::graph::VertexId;
use gc_core::path::ScoredPath;

fn main() {
    let args = App::new("gc-short-path")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Builds a graph from the list of commands")
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
    let graph = graph_utils::load_graph(path).expect("Couldn't load graph");

    match shortest_path(&graph, start_vertex, end_vertex) {
        Some(ScoredPath {
            score,
            path: shortest,
        }) => {
            println!(
                "Shortest path from vertex {} to vertex {} with total cost of {}",
                start_vertex.0, end_vertex.0, score
            );
            for vertex in shortest.to_vertex_list() {
                println!("{}", vertex.0);
            }
        }
        None => println!(
            "Vertex {} is not reachable from vertex {}.",
            end_vertex.0, start_vertex.0
        ),
    }
}

fn shortest_path(graph: &DirectedGraph, start: VertexId, end: VertexId) -> Option<ScoredPath> {
    use gc_core::search::a_star;
    a_star::shortest_path(
        graph,
        a_star::one_weighted_edge,
        a_star::zero_heuristic,
        start,
        end,
    )
}
