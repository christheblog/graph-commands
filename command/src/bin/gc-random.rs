use gc_core::graph::VertexId;
use gc_core::graph::Edge;
use clap::{App, Arg};
use gc_command::arg_utils;
use gc_command::graph_utils;
use gc_command::version;
use gc_core::directed_graph::DirectedGraph;
use itertools::Itertools;

fn main() {
    let args = App::new("gc-random")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Creates a random graph")
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
            Arg::with_name("force")
                .long("force")
                .short("f")
                .help("By-pass interactive confirmation")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("connected")
                .long("connected")
                .short("c")
                .help("Creates a connected graph")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("cycle")
                .long("cycle")
                .short("-O")
                .help("Add a cycle to the graph")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("vertex-count")
                .long("vertex-count")
                .short("-v")
                .help("Creates the graph with the given number of vertices")
                .required(true)
                .default_value("100")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("edge-count")
                .long("edge-count")
                .short("-e")
                .help("Target the provided number of edges")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();
    let force = args.is_present("force");
    let vertex_count =  args.value_of("vertex-count")
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap();
    let edge_count =  args.value_of("edge-count")
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(vertex_count * 3);


    let maybe_graph = creates_random_dag(vertex_count, edge_count);

    if let Some(random_graph) = maybe_graph {
        if !force {
            let yes_no = arg_utils::confirmation_yes_no(&format!(
                "Creating a random graph will clean existing graph at '{}' ? (yes/no)",
                path
            ));
            if !yes_no {
                println!("Aborting.");
                return ();
            }
        }

        // Cleaning current graph first
        graph_utils::clean(path).expect(&format![
            "A problem occured. Path '{}' might not exist, or the graph is currently lock (check 'lock' file)",
            &path
        ]);
        graph_utils::init(path).expect(&format![
            "A problem occured. Unable to create a new graph at '{}' (check directory structure and 'lock' file)",
            &path
        ]);
        graph_utils::save_graph_as_commands(path, &random_graph).expect(&format![
            "A problem occured. Unable to save graph at '{}' (check directory structure and 'lock' file)",
            &path
        ]);
    } else {
        println!("Couldn't generate graph.")
    }

}

// To create a DAG:
// Creates layers of Vertices
// A layer can only have vertices connected to a lower layer
// This prevents cycle to happen
fn creates_random_dag(vertex_count: usize, edge_count: usize) -> Option<DirectedGraph> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let layer = 10;
    let layer_size = (vertex_count / layer) as u64;
    let mut graph = DirectedGraph::new();
    while graph.edge_count() < edge_count {
        for vid in 1..vertex_count {
            let this = vid as u64;
            let other = rng.gen::<u64>();
            let (src, dst) = (this.min(other), this.max(other));
            if dst-src > layer_size {
                graph.add_edge(Edge(VertexId(src), VertexId(dst)));
            }
        }
    }

    Some(graph)
}
