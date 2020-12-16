use clap::{Arg, App};

fn main() {
    let matches = App::new("HTS4")
        .version("4.2.1")
        .author("Mason Sipe <m-sipe@protonmaill.com>")
        .about("Searches for executables that are installed on your system. 
This is free software, and you are welcome to redistribute it under certain conditions.")
        .arg(Arg::with_name("services")
            .short("s")
            .long("services")
            .value_name("Services")
            .help("Looks for services on your system.")
            .takes_value(false))
        .arg(Arg::with_name("executable")
            .short("e")
            .long("executable")
            .value_name("Executables")
            .help("Searches for executable programs on your system")
            .takes_value(false))
        .arg(Arg::with_name("find-all")
            .short("a")
            .long("find-all")
            .value_name("Find-All")
            .help("Searches for all listed tools, services, etc.")
            .takes_value(false))
        .arg(Arg::with_name("Games")
            .short("g")
            .long("games")
            .value_name("Games")
            .help("Searches for game binaries on your system")
            .takes_value(false))
    .get_matches();
    if matches.is_present("services") {
        cringe::srv::main();
    }
    if matches.is_present("executable") {
        cringe::exe::main();
    }
    if matches.is_present("games") {
        cringe::gme::main();
    }
    if matches.is_present("find-all") {
        cringe::exe::main();
        cringe::srv::main();
        cringe::gme::main();
    }
}



