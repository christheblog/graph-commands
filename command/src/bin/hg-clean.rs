use clap::{App, Arg};
use hg_command::utils;
use hg_command::version;

fn main() {
    let args = App::new("hg-clean")
        .version(version::VERSION)
        .author(version::AUTHOR)
        .about("Cleans a graph")
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
                .max_values(1)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .short("s")
                .help("Don't print anything on stdout in case of success")
                .required(false)
                .max_values(1)
                .takes_value(false),
        )
        .get_matches();

    let path = args.value_of("path").unwrap();
    let force = args.is_present("force");
    let silent = args.is_present("silent");

    if !force {
        let yes_no = utils::confirmation_yes_no(&format!("Are you sure you want to clean graph at '{}' ? (yes/no)", path));
        if !yes_no {
            println!("Aborting.");
            return ();
        }
    }

    if !silent {
        println!("Cleaning graph under '{}' ...", path);
    }
    utils::clean(path).expect(&format![
        "A problem occured. Path '{}' might not exist, or the graph is currently lock (check 'lock' file)",
        path
    ]);
    if !silent {
        println!("Done.");
    }
}
