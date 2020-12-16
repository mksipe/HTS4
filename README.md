# HTS4

#### Rationale

The HTS project was initially developed in one of my older repositories, shell-script-lib, outdated and used too many resources, and took too long to execute. This is the fourth official revision of this project to make finding specific tools, services, and other possibly unwanted services on your system.


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