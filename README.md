# HTS4

#### Rationale

The HTS project was initially developed in one of my older repositories, shell-script-lib, outdated and used too many resources, and took too long to execute. This is the fourth official revision of this project to make finding specific tools, services, and other possibly unwanted services on your system. This project is not very practical in the real world, as if somebody were to change an executable name from this project's cringelib list, the program would not be able to pick it up. However, this was initially developed in 2017 and was hardcoded in shell, which took up many resources to run on a virtual machine with limited resources. So it was written in rust to help mitigate this issue. The original project followed the same setup as it would not be practical to use this tool in actual sysadmin scenarios. However, for CTF competitions, such as CyberPatriot, where executables are installed and not modified and usually are against policy to have on their devices, it would be reasonable to use this tool instead of manually searching for them. 


#### Status
|OS|Status|
|-|-|
|Linux|[![Rust - Linux](https://github.com/mksipe/HTS4/actions/workflows/linrust.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/linrust.yml)|
|Windows|[![Rust - Windows](https://github.com/mksipe/HTS4/actions/workflows/winrust.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/winrust.yml)|
|Mac|[![Rust - Mac](https://github.com/mksipe/HTS4/actions/workflows/macrust.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/macrust.yml)|

## Dependencies

Cargo: `<package manager> install cargo -y`

--- 

## Usage

### Standalone

These set of instructions are to run this program once with cargo, then can be safely removed. 

#### Installation

1. `git clone https://github.com/mksipe/HTS4`
2. `cd HTS4`
3. `cargo run -- -h`
> after usage
4. `cd ..`
5. `rm -r HTS4`


### Built-in

These set of instructions are to run this program periodically through the command line.

#### Installation

> You must be an administrator to be able to add public executables on Linux.

1. `git clone https://github.com/mksipe/HTS4`
2. `cd HTS4`
3. `sudo chmod 700 Install.sh` 
4. `sudo ./Install.sh`
5. `cd .. `
6. `rm HTS4 -r  `


##### Side note

The final output should show the version of the currently installed software. If you want to update the software, you need to run the same script again with the new source files. 
