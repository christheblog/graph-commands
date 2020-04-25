use clap::{App, Arg};
use hg_command::arg_utils;
use hg_command::graph_utils;
use hg_command::version;
use hg_core::graph::VertexId;

fn main() {
    let args = App::new("hg-delete")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Remove a vertex or an edge from a graph")
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
            Arg::with_name("vertex")
                .long("vertex")
                .short("v")
                .help("Removes a vertex id from a graph")
                .required(false)
                .multiple(true)
                .min_values(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("edge")
                .long("edge")
                .short("e")
                .help("Removes a directed edge")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();

    args.values_of("vertex")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| vids.map(|vid| VertexId(vid)).collect())
        .map(|vids| graph_utils::remove_vertices(path, vids));

    args.values_of("edge")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| vids.map(|vid| VertexId(vid)).collect())
        .map(|vids| arg_utils::as_vertex_tuple(vids).expect("Invalid number of vertices. Must be an even number"))
        .map(|vids| graph_utils::remove_edges(path, vids));
}
