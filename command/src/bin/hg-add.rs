use clap::{App, Arg};
use hg_command::utils;
use hg_command::version;
use hg_core::graph::VertexId;

fn main() {
    let args = App::new("hg-add")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Adds a vertex or an edge to a graph")
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
                .help("Adds a vertex id to a graph")
                .required(false)
                .multiple(true)
                .min_values(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("reverse")
                .long("reverse")
                .short("r")
                .help("Reverses created edges")
                .required(false)
                .max_values(1)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("edge")
                .long("edge")
                .short("e")
                .help("Adds a directed edge between 2 vertex ids")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("chain")
                .long("chain")
                .short("c")
                .help("Link all the provided vertices by a directed edge, effectvely creating a chain")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cycle")
                .long("cycle")
                .short("-O")
                .help("Creates a directed cycle with all vertices provided")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("star")
                .long("star")
                .short("X")
                .help("Creates a star pattern, using first vertex as center, and with all edges directed out of the center")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("clique")
                .long("clique")
                .short("C")
                .help("Creates a clique (ie all vertices will be connected to all other vertices)")
                .required(false)
                .min_values(2)
                .takes_value(true),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();
    let reverse_edges = args.is_present("reverse");

    args.values_of("vertex")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| vids.map(|vid| VertexId(vid)).collect())
        .map(|vids| utils::add_vertices(path, vids));

    args.values_of("edge")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| vids.map(|vid| VertexId(vid)).collect())
        .map(|vids| utils::as_vertex_tuple(vids).expect("Invalid number of vertices. Must be an even number"))
        .map(|vids| reverse_if_needed(reverse_edges, vids))
        .map(|vids| utils::add_edges(path, vids));

    args.values_of("chain")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| chain_from_vertices(vids.map(|vid| VertexId(vid)).collect()))
        .map(|vids| reverse_if_needed(reverse_edges, vids))
        .map(|vids| utils::add_edges(path, vids));

    args.values_of("cycle")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| cycle_from_vertices(vids.map(|vid| VertexId(vid)).collect()).expect("Invalid cycle"))
        .map(|vids| reverse_if_needed(reverse_edges, vids))
        .map(|vids| utils::add_edges(path, vids));

    args.values_of("star")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| star_from_vertices(vids.map(|vid| VertexId(vid)).collect()))
        .map(|vids| reverse_if_needed(reverse_edges, vids))
        .map(|vids| utils::add_edges(path, vids));

    args.values_of("clique")
        .map(|vids| vids.map(|v| v.parse::<u64>().expect("Invalid vertex id")))
        .map(|vids| clique_from_vertices(vids.map(|vid| VertexId(vid)).collect()))
        .map(|vids| utils::add_edges(path, vids));

}

fn chain_from_vertices(vertices: Vec<VertexId>) -> Vec<(VertexId, VertexId)> {
    let mut result = vec![];
    for i in 0..vertices.len() - 1 {
        result.push((vertices[i], vertices[i + 1]));
    }
    result
}

fn cycle_from_vertices(mut vertices: Vec<VertexId>) -> Option<Vec<(VertexId, VertexId)>> {
    vertices.first().map(|x| x.clone()).map(|vid| {
        vertices.push(vid);
        chain_from_vertices(vertices)
    })
}

fn star_from_vertices(vertices: Vec<VertexId>) -> Vec<(VertexId, VertexId)> {
    let mut result = vec![];
    for i in 1..vertices.len() {
        result.push((vertices[0], vertices[i]));
    }
    result
}

fn clique_from_vertices(vertices: Vec<VertexId>) -> Vec<(VertexId, VertexId)> {
    let mut result = vec![];
    for i in 0..vertices.len() {
        for j in 0..vertices.len() {
            if i!= j {
                result.push((vertices[i], vertices[j]));
            }
        }
    }
    result
}

fn reverse_if_needed(should_reverse: bool, vertices: Vec<(VertexId, VertexId)>) -> Vec<(VertexId, VertexId)> {
    if should_reverse {
        vertices.iter().map(|(src,dest)| (*dest, *src)).collect()
    } else {
        vertices
    }

}
