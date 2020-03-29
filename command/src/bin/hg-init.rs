use clap::{App, Arg};
use hg_command::utils;
use hg_command::version;

fn main() {
    let args = App::new("hg-init")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Creates an empty graph")
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .help("Use the specified directory instead of the current one")
                .default_value(".")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    // Reading arguments
    let path = args.value_of("path").or_else(|| Some(".")).unwrap();
    utils::init(path).expect("Couldn't create graph directory structure");
}
