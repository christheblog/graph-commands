use clap::{App, Arg};
use hg_command::graph_utils;
use hg_command::version;
use hg_core::algorithm::topo_sort;


fn main() {
    let args = App::new("hg-topo-sort")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Compute a topological order of a directed graph")
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

    let graph = graph_utils::load_graph(path).expect("Couldn't load graph");

    match topo_sort::topological_sort(&graph) {
        Some(vertices) => {
            print!("Topological order: ");
            for v in vertices {
                print!("{} ", v.0);
            }
            println!()
        }
        None => println!("Graph is not a DAG."),
    }
}
