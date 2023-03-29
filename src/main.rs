use std::{env, process, fs, path::Path};

const PATH: &str = "/home/cuikho210/Documents/services/";

fn get_command() -> Result<Vec<String>, ()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        Ok(args)
    } else {
        Err(())
    }
}

fn init(args: Vec<String>) {
    if args[1] == "list" || args[1] == "ls" { list_services(); }
    else if args[1] == "rename" { rename_service(args); }
    else if args[1] == "up" { run_service("up", args); }
    else if args[1] == "down" { run_service("down", args); }
    else if args[1] == "start" { run_service("start", args); }
    else if args[1] == "stop" { run_service("stop", args); }
    else if args[1] == "ps" { list_running_services(); }
    else { show_help(); }
}

fn run_command(docker_compose_command: &str, service_name: &String) {
    let file_path = PATH.to_owned() + service_name + "/docker-compose.yml";

    if !Path::new(&file_path).exists() {
        eprintln!("{} service is incorrect!", service_name);
        process::exit(1);
    }

    println!("Running command {} for {}", docker_compose_command, file_path);

    let mut command = process::Command::new("/usr/bin/docker");

    command
        .arg("compose")
        .arg("-f")
        .arg(file_path)
        .arg(docker_compose_command);

    if docker_compose_command == "up" {
        command.arg("-d");
    }

    let output = command.output().expect("Error!");

    println!("{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr))
}

fn run_service(command: &str, args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Too few arguments!");
        process::exit(1);
    }

    for (index, arg) in args.iter().enumerate() {
        if index > 1 {
            run_command(command, arg);
        }
    }
}

fn rename_service(args: Vec<String>) {
    if args.len() < 4 {
        eprintln!("Too few arguments!");
        process::exit(1);
    }

    let old_path: String = PATH.to_owned() + &args[2];
    let new_path: String = PATH.to_owned() + &args[3];
    fs::rename(old_path, new_path).unwrap();
    println!("Rename successfully!");
}

fn list_services() {
    for file in fs::read_dir(PATH).unwrap() {
        println!("{}", file.unwrap().file_name().to_str().unwrap());
    }
}

fn list_running_services() {
    let output = process::Command::new("/usr/bin/docker")
        .arg("ps")
        .output()
        .expect("Error!");

    println!("{}", String::from_utf8_lossy(&output.stdout))
}

fn show_help() {
    println!("\n----------[ Docker Service Manager ]----------\n");
    println!("  start ...<service name>         Start a service");
    println!("  stop ...<service name>          Stop a service");
    println!("  up ...<service name>            Up a service");
    println!("  down ...<service name>          Down a service");
    println!("  list (or ls)                    List services");
    println!("  rename <old name> <new name>    Rename a service");
    println!("\n    Example: dsm up nginx mariadb mongodb");
}

fn main() {
    match get_command() {
        Ok(args) => {
            init(args);
        }
        Err(_err) => {
            show_help();
            process::exit(1);
        }
    }
}
