use clap::{App, Arg};
use hg_command::utils;
use hg_command::version;

fn main() {
    let args = App::new("hg-build")
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
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Verbose mode")
                .default_value("false")
                .required(false)
                .takes_value(false)
                .max_values(1),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();
    let verbose = args.is_present("verbose");

    let graph = utils::load_graph(path).expect("Couldn't load graph");
    if verbose {
        println!("Vertices: {}", graph.vertex_count());
        println!("Edges: {}", graph.edge_count());
    }

    utils::save_graph_as_commands(path, &graph)
        .expect("Couldn't save graph. Data maybe lost !");

    println!("Done.")
}
