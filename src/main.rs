use clap::{Arg, ArgMatches, Command};
fn main() {
    let kairo: Command = Command::new("repo")
        .author("Willy Micieli")
        .version("0.0.0")
        .color(clap::ColorChoice::Always)
        .bin_name("kairo")
        .flatten_help(true)
        .about("Manage repositories")
        .arg(Arg::new("create").long("create").short('c').required(true))
        .arg(Arg::new("create").long("create").short('c').required(true));

    let matches: ArgMatches = kairo.get_matches();
    dbg!(matches);
}
