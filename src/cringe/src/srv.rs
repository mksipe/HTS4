use colored::*;

pub fn main() {
    println!("{}","\nServices".green());
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
        "daphne",       // Django channels WebSocket Server
        "micro-httpd",  // Really small http server
        "nghttpd",      // Server, proxy and client implementing HTTP/2
        "qweborf",      // Share files using the HTTP protocol
        "mini_httpd",   // Small HTTP Server
        "tntnet",       // modular, multithreaded web application server for CPP
        "uvicorn",      // ASGI server implementation using uvloop and httptools
        "webfsd",       // lightweight HTTP server for static content
        "weborf",       // Fast and small webserver meant to be run without root privileges
        "yaws",         // High performance HTTP 1.1 webserver written in erlang
        "drupal",       // Open sourced content management system
        "iptables",     // Default firewall for linux (replaced bu nft)
        "smbd",         // Server Message block daemon
        "acron",        // Lanuguage support
        "acpi",         // Power management / laptop battery fan monitor
        "acpid",        // ACPI dispatch listening daemon
        "adsl",         // Internal switch control
        "alsa",         // Advanced Linux Sound Architechture
        "anacron",      // Task Scheduling tool
        "apmd",         // Laptop Power management
        "apmiser",      // Yet another laptop battery extender
        "arpwatch",     // Ethernet ip addressing pairing monitor
        "atd",          // Periodic command sceduler
        "autofds",      // Automount services
        "bluetooth",    // Core bluetooth service
        "bootparamd",   // boot server (zeroconf preferred)
        "canna",        // Japanese conversion engine
        "capi4linux",   // Basic CAPI subsystem
        "cpufreq",      // Probes and configures CPU frequency daemon modules
        "cpufreqd",     // CPUfreq daemon
        "crond",        // Cron daemon
        "cups-ipd",     // Allows CUPS to connect to a cups server
        "cups",         // Common unix printing service
        "cupsd",        // Cups printing daemon
        "cvs",          // Common versioning system
        "devfsd",       // System maintenience
        "dhcpd",        // DHCP server
        "diald",        // Dial up auto dialer
        "dkms",         // Autoinstall boot for DKMS
        "dm",           // Display manager
        "dnbc",         // Digital Network Bing Chrooter
        "drakxtools_http",   // Miniserve administration server
        "dund",         // Bluetooth Dialup networking
        "fam",          // File system alteration monitor
        "finger",       // Remote access of data
        "gpm",          // Console mode ouse driver
        "haldaemon",    // Hardware monitoring system
        "harddrake",    // Hardware detection service
        "heartbeat",    // High availability service
        "hidd",         // Bluetooth HTD server
        "hplip",        // HP Linux printing and imaging
        "hpoj",         // HP office jet drivers
        "ibod",         // IDSN MPPP bandwidth on demand
        "identd",       // Identification through TCP connection
        "imaps",        // Secure IMAP server
        "iplog",        // logs TCP, UDP, ICMP connections with hostname and remote host
        "ipop2",        // POP2 mail server
        "ipop3",        // POP3 mail server
        "ipsec",        // Encrypted authenticated communications
        "ipvsadmin",    // Linux kernel IP virtual server
        "irda",         // Infared device interface
        "keytable",     // keyboard map
        "kheader",      // boot services
        "lads",         // Login Anomaly Detection System
        "leafnode",     // NNTP service for X?intetd
        "lisa",         // LAN information server
        "lmsensors",    // Hardware health monitor
        "mailman",      // GNU Mailing list manager
        "mandi",        // Interactive firewall
        "mmdadm",       // Software raid tool
        "mdnsresponder", // Zeroconf DNS configuration tool
        "messagebus",    // Event monitoring service
        "mon",          // System monitoring daemon
        "netplugd",     // Network card daemon
        "network",      // Turns the network card on or powers the modem
        "nfs",          // Network FIle Share
        "nfsfs",        // The network file share server
        "nfslock",      // NFS file locking
        "nifd",         // howl client
        "nscd",         // password and group locking service
        "numlock",      // number lock light control
        "oki4daemon",   // compatibilty daemon for OKI4
        "pand",         // Bluetooth personal area networking
        "partmon",      // partition manager
        "pcmcia",       // personal computer memory card international association
        "pg_autovacum",  // Postgres maintenience
        "pop3s",        // secure pop3 server
        "portmap",      // RPC support
        "Postgresql",   // SQL database
        "pptp",         // Shutdown service for PPP
        "prelude",      // IDS
        "psacct",       // Process accounting
        "rawdevices",   // assigns rawdevices to blocks for use
        "rsync",        // remote synchronization
        "saned",        // network scanner service
        "shorwall",     // Firewall
        "smartd",       // self monitor service
        "smb",          // Samba network service
        "snmpd",        // Simple network management protocol
        "sound",        // sound system
        "squid",        // caching tool
        "subversion",   // concurrent versioning system
        "swat",         // Samba web administration tool
        "syslog",       // system logging
        "tmdns",        // multicast dns responder
        "upsd",         // NUT daemon and drivers
        "upsmon",       // UPS monitoring tool
        "vncserver",    // VNC server
        "winbind",      // samba name server
        "wine",         // Wine is not an emulator
        "wlan",         // control daemon
        "xfs",          // X font server
        "ypbind",       // Name server for sun's YP server


    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let _ret = match out {
            Ok(ref _ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
}