use which::which;
use colored::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("HTS4")
        .version("4.1.1")
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
        let _ret = match out {
            Ok(_ref ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
} //nice

fn executables() {
    println!("{}","Executables".green());
    let list = vec![
        //list of executables to search for
        // Kali tool list
        "ace",
        "amap",
        "apt2",
        "arp-scan",
        "automater",
        "bing-ip2hosts",
        "braa",
        "casefile",
        "cdpsnarf",
        "cisco-torch",
        "copy-router-config.pl",
        "dmitry",
        "dnmap_client",
        "dnsenum",
        "dnsmap",
        "dnsrecon",
        "dnstracer",
        "dnswalk",
        "dotdotpwn.pl",
        "enum4linux",
        "enumiax",
        "eyewitness",
        "python-faraday",
        "fierce",
        "firewalk",
        "fragroute",
        "fragrouter",
        "ghost-phisher",
        "golismero",
        "goofile",
        "hping3",
        "ident-user-enum",
        "inspy",
        "intrace",
        "ismtp",
        "lbd",
        "maltego",
        "masscan",
        "metagoofil",
        "miranda",
        "nbtscan-unixwiz",
        "nikto",
        "nmap",
        "ntop",
        "usufy",
        "p0f",
        "parsero",
        "recon-ng",
        "setoolkit",
        "smbmap",
        "smtp-user-enum",
        "snmp-check",
        "sparta",
        "sslcaudit",
        "sslsplit",
        "sslstrip",
        "sslyze",
        "sublist3r",
        "address6",
        "theharvester",
        "tlssled",
        "twofi",
        "unicornscan",
        "urlcrazy",
        "wireshark",
        "wol-e",
        "xplico",
        "bbqsql",
        "bed",
        "cisco-auditing-tool",
        "cisco-global-exploiter",
        "cisco-ocs",
        "doona",
        "hexorbase",
        "jsql",
        "lynis",
        "ohrwurum",
        "gvm-start",
        "oscanner",
        "powerfuzzer",
        "sfuzz",
        "sidguesser",
        "siparmyknife",
        "sqlmap",
        "sqlninja",
        "sqlsus",
        "tnscmd10g",
        "unix-privsc-check",
        "yersinia",
        "armitage",
        "backdoor-factory",
        "beef-xss",
        "commix",
        "crackle",
        "crunch",
        "searchsploit",
        "jboss-linux",
        "linux-exploit-suggester",
        "msfconsole",
        "msfpc",
        "routersploit",
        "shellnoob",
        "aircrack-ng",
        "asleap",
        "besside-ng",
        "bluelog",
        "bluemaho",
        "bluepot",
        "blueranger",
        "bully",
        "cowpatty",
        "eapmd5pass",
        "easside-ng",
        "fern-wifi-cracker",
        "freeradius-wpe",
        "giskismet",
        "gqrx",
        "gr-scan",
        "hostapd-wpe",
        "ivstools",
        "kal",
        "zbgoodfind",
        "kismet",
        "makeivs-ng",
        "mdk3",
        "mfcuk",
        "mfoc",
        "mfterm",
        "multimon-ng",
        "packetforge-ng",
        "pixiewps",
        "pyrit",
        "reaver",
        "redfang",
        "rlsdr-scanner",
        "spooftooph",
        "tkiptun-ng",
        "wesside-ng",
        "wifi-honey",
        "wifiphisher",
        "wifiarp",
        "wifite",
        "wpaclean",
        "binwalk",
        "bulk-extractor",
        "capstone",
        "chntpw",
        "cuckoo",
        "dc3dd",
        "ddrescue",
        "dff",
        "distorm3",
        "dumpzilla",
        "extundelete",
        "foremost",
        "galleta",
        "guymager",
        "iphone-backup-analyzer",
        "pdf-parser",
        "pfid",
        "pdgmail",
        "peepdf",
        "regripper",
        "volatility",
        "apache-users",
        "arachni",
        "blindelephant",
        "burpsuite",
        "cutycapt",
        "daavtest",
        "deblaze",
        "dirb",
        "fimap",
        "funkload",
        "gobuster",
        "grabber",
        "hurl",
        "joomscan",
        "padbuster",
        "paros",
        "plecost",
        "proxystrike",
        "skipfish",
        "ua-tester",
        "uniscan",
        "w3af",
        "webscarab",
        "webshag",
        "webslayer",
        "websploit",
        "wfuzz",
        "whatweb",
        "wpscan",
        "xsser",
        "zaproxy",
        "dhcpig",
        "iaxflood",
        "inundator",
        "inviteflood",
        "flow6",
        "mdk3",
        "rtpflood",
        "slowhttptest",
        "t50",
        "termineter",
        "thc-ssl-dos",

    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let _ret = match out {
            Ok(ref _ret) => println!("[{}] {:?}", "+".red(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
}