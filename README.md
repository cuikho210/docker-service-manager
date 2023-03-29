# DSM
## Requirements
- Rust (rustc, cargo, ...)

## Config
Adjust the `PATH` constant to point to your host directory at `src/main.rs`.  

## Install
Run the `install.sh` script.  
Optional argument is the directory where the binary will be installed. Default is `~/.local/bin`.  

Example:  
```bash
chmod +x ./install.sh
./install.sh
./install.sh ~/.bin
sudo ./install.sh /usr/bin
```

## Run
Restart the terminal session.  
Run `dsm` to get help.  