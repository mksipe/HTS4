#!/usr/bin/env sh

if [ $(/usr/bin/id -u) -ne 0 ] ; then
    printf "You must be root.\n"
    exit
elif [ $(/usr/bin/id -u) -eq 0 ]; then
    if [ $(which cargo > /dev/null ; echo $?) -ne 0 ]; then
        printf "Cargo is not installed.\n"
    elif [ $(which cargo > /dev/null; echo $?) -eq 0 ]; then
        printf "Making installation.\n"
        printf "Compiling binaries...\n" 
        cargo build --release
        cp target/debug/hts4 /usr/bin/ > /dev/null
       	if [ $(which hts4 > /dev/null; echo $?) -eq 0 ]; then
		printf "`hts4 -V`, has been installed.\n"
	elif [ $(which hts4 > /dev/null; echo $?) -ne 0 ]; then
		printf "HTS4 could not be installed.\n"
	fi
    fi
fi
