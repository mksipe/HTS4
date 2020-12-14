use which::which;
use colored::*;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("HTS4")
        .version("4.1")
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
            .help("Looks for executable programs on your system")
            .takes_value(false))
    .get_matches();
    if matches.is_present("services") {
        services();
    }
    if matches.is_present("executable") {
        executables();
    }
}

fn services() {
    println!("{}","Services".green());
    let list = vec![
        //list of services to search for
        //Preferrably ones that provide network services
        "sshd",         // Secure Shell daemon
        "cron",         // Linux task scheduler
        "avahi-daemon", // Apple bonjour service
        "cupsd",        // Apple Central Unix Printing Service
        "bluetoothd",   // Bluetooth daemon
        "mysqld",       // MySQL daemon
        "rsync",        // Remote sync service
        "named",        // Internet domain name Service (required by Bind9)
        "psql",         // PostgreSQL database CLI interaction
        "samba",        // SAMBA SMB share service
        "snmpd",        // Simple network management protocol service
        "ftp",          // File transfer Protocol
        "netcat",       // Netcat tool
        "apache2",      // Apache Web Service
        "nginx",        // Nginx Web Service
        "mariadb",      // Maria Database Server
        "ntpd",         // Network Time Protocol Daemon
        "pure-ftpd",    // Pure FTP daemon
        "postfix",      // Postfix mail server
        "smtpd",        // Simple Mail Transfer Protocol daemon
        "nscd",         // Name Service cache daemon\
        "lighttpd",     // Light HTTP Service
        "sendmail",     // Mail server daemon
        "vsftpd",       // Very Secure FTP daemon
    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let ret = match out {
            Ok(ref ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(error) => print!("{}", ""),
        };
    }
}

fn executables() {
    println!("{}","Executables".green());
    let list = vec![
        //list of executables to search for
        "g++",
    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let ret = match out {
            Ok(ref ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(error) => print!("{}", ""),
        };
    }
}