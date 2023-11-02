# HTS4

#### [Documentation](https://mksipe.github.io/mksipe/hts4/)

#### Rationale

The HTS project was initially developed in one of my older repositories, shell-script-lib, outdated and used too many resources, and took too long to execute. This is the fourth official revision of this project to make finding specific tools, services, and other possibly unwanted services on your system. This project is not very practical in the real world, as if somebody were to change an executable name from this project's cringelib list, the program would not be able to pick it up. However, this was initially developed in 2017 and was hardcoded in shell, which took up many resources to run on a virtual machine with limited resources. So it was written in rust to help mitigate this issue. The original project followed the same setup as it would not be practical to use this tool in actual sysadmin scenarios. However, for CTF competitions, such as CyberPatriot, where executables are installed and not modified and usually are against policy to have on their devices, it would be reasonable to use this tool instead of manually searching for them. 

#### Status
|OS|Status|
|:-:|:-:|
|Ubuntu Latest |[![Rust - Linux](https://github.com/mksipe/HTS4/actions/workflows/linrust.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/linrust.yml)|
|Ubuntu 18.04  |[![Rust - Linux 18.04](https://github.com/mksipe/HTS4/actions/workflows/lin1804.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/lin1804.yml)|
|Ubuntu 20.04  |[![Rust - Linux 20.04](https://github.com/mksipe/HTS4/actions/workflows/lin2004.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/lin2004.yml)|
|Windows Latest|[![Rust - Windows](https://github.com/mksipe/HTS4/actions/workflows/winrust.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/winrust.yml)|
|Windows 2016  |[![Rust - Windows2016](https://github.com/mksipe/HTS4/actions/workflows/win2016.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/win2016.yml)|
|Windows 2019  |[![Rust - Windows 2019](https://github.com/mksipe/HTS4/actions/workflows/win2019.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/win2019.yml)|
|Windows 2022  |[![Rust - Windows2022](https://github.com/mksipe/HTS4/actions/workflows/win2022.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/win2022.yml)|
|Mac Latest|[![Rust - Mac](https://github.com/mksipe/HTS4/actions/workflows/macrust.yml/badge.svg)](https://github.com/mksipe/HTS4/actions/workflows/macrust.yml)|


## Dependencies

Cargo: `<package manager> install cargo -y`

--- 

## Usage

### Standalone

These set of instructions are to run this program once with cargo, then can be safely removed. 

#### Installation

> This is for the most recent changes. They may work, they may not. The most stable versions are in releases.

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
