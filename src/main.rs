use which::which;
use colored::*;


fn main() {
    services();
}

fn services() {
    let list = vec![
        //list of services to search for    \
        "openssh-server",
        "sshd",
        "ssh",

    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let ret = match out {
            Ok(ref ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(error) => print!("{}", ""),
        };
    }
}