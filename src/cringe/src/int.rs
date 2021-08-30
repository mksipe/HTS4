use colored::*;

pub fn main() {
    println!("{}","Interpreters".green());
    let list = vec![
        "python",
        "python2.7",
        "python3",
        "perl",
        "ruby",
        "rustc",
        "cargo",
        "java",
        "javac",
        "gcc",
        "clang",
        "cc",
        "mxmlc",
        "make",
        "cmake",
        "mono",
        "pc",
        "php5",
        "php7",
        "bash",
        "ash",
        "csh",
        "sh",
        "dash",
        "nu",
        "ksh",
        "tcsh",
        "cs",
        "eshell",
        "fish",
        "pwsh",
        "rc",
        "sash",
        "scsh",
    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let _ret = match out {
            Ok(ref _ret) => println!("[{}] {:?}", "+".cyan(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
}