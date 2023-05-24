# UPSH

UPSH is a shell program that aims to integrate all the required functionalities of shell. It will have options to turn on and off certain features via config file.

## How to build and run?
To run the following command in the main project folder to build it
```bash
./build.sh
```
The executable will be kept inside a bin folder. You can copy paste it to appropriate location. For use by all users in wheel/sudoer group, use the following command in the bin folder
```bash
sudo mkdir -p /opt/upsh
sudo cp upsh /opt/upsh/
sudo ln -s /opt/upsh/upsh /usr/bin/upsh
```
To run the program, just launch it as a command in a terminal as follows
```bash
upsh
```

