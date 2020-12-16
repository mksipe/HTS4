use colored::*;

pub fn main() {
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
        // Sectools list
        "ophcrack-cli",
        "nessuscli",
        "snort",
        "tcpdump",
        "scapy",
        "hydra",
        "medusa",
        "radare2",
        "rainbowcrack",
        "brutus",
        "ike-scan",
        "truecrypt",
        "openssl",
        "tor",
        "stunnel",
        "transmission",
        "drupal",
        "ollydbg",
        "gdb",
        "helix",
        "autopsy",
        "wapiti",
        "telnet",
        "socat",
        "nemesis",
        "ettercap",
        "dsniff",
        "ntop",
        "ngrep",
        "etherape",
        "inssider",
        "kismac",
        "canvas",
        "netsparker",
        "webgoat",
        "splunk",
        "nagios",
        "argus",
        "nexpose",
        "qulysguard",
        "mbsa",
        "retina",
        "nipper",
        "fiddler",
        "sslstrip",
        "ratproxy",
        "websecurify",
        "dirbuster",
        "grendel-scan",

    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let _ret = match out {
            Ok(ref _ret) => println!("[{}] {:?}", "+".red(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
}
