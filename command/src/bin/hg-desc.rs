use clap::{App, Arg};
use hg_command::utils;
use hg_command::version;
use hg_core::graph::VertexId;

fn main() {
    let args = App::new("hg-build")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Prints some basic statistics on the graph")
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .help("Use the specified directory instead of the current one")
                .default_value(".")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();
    let graph = utils::load_graph(path).expect(&format!["Couldn't load graph at '{}'", path]);

    println!("Path: {}", path);
    println!("Vertices: {}", graph.vertex_count());
    println!("Edges: {}", graph.edge_count());
    println!(
        "Min vertex id: {}",
        graph
            .vertices()
            .map(|VertexId(x)| x)
            .min()
            .map(|x| x.to_string())
            .unwrap_or_else(|| "-".to_string())
    );
    println!(
        "Max vertex id: {}",
        graph
            .vertices()
            .map(|VertexId(x)| x)
            .max()
            .map(|x| x.to_string())
            .unwrap_or_else(|| "-".to_string())
    );

    // FIXME compute more indicators (avg node degree, max node degree, min node degree, DAG yes/no, components, ...)
}
