use colored::*;

pub fn main() {
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
            Ok(ref _ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
}