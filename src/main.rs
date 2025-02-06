use rand::Rng;
use std::io::{self, Write};
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::env;
use std::process::{Command};
use tokio::time::{sleep, Duration};
use colored::*;

fn generate_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

async fn wait(seconds: u64) {
    let mut remaining_seconds = seconds;
    
    while remaining_seconds > 0 {
        let time_remaining = format_seconds(remaining_seconds);
        print!("{}", format!("\rWaiting for {}", time_remaining).yellow());
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await;
        remaining_seconds -= 1;
    }
    println!("{}", "\rRunning Next Scan...".yellow());
}

fn format_seconds(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn create_file(name: &str) -> std::io::Result<File> {
    let file_path = get_file_path(name)?;

    if file_path.exists() {
        println!("{}", "File already exists.".red());
    } else {
        let file = File::create(&file_path)?;
        return Ok(file);
    }

    Ok(File::open(file_path)?)
}

fn get_file_path(name: &str) -> std::io::Result<PathBuf> {
    let mut path = env::current_dir()?;
    path.push("map_data");
    path.push(name);
    Ok(path)
}

fn map_target(ip: &str, port: &str, scan_type: Option<&str>, output_prefix: &str) {
    let scan_type_str = match scan_type {
        Some("-sS") => "SYN",
        Some("-sF") => "FIN",
        Some("-sN") => "NULL",
        Some("-sT") => "CONNECT",
        None => "REGULAR",
        Some(_) => "",
    };

    let file_name = format!("{PREFIX}_{TYPE}", PREFIX=output_prefix, TYPE=scan_type_str);
    match create_file(&file_name) {
        Ok(mut file) => {
            let mut command = Command::new("sudo");

            command.arg("nmap");
            command.arg(ip);
            if let Some(f) = scan_type {
                command.arg(f);
            }
            if !port.trim().is_empty() {
                command.arg("-p").arg(port);
            }

            let output = command.output().expect(&"Failed to execute command".red());

            if output.status.success() {
                file.write_all(&output.stdout).expect(&"Failed to write output to file".red());
                println!("\nnmap {} {} {}",
                    ip, 
                    scan_type.unwrap_or(""), 
                    if port.trim().is_empty() {
                        "".to_string() 
                    } else { 
                        format!("-p {}", port) 
                    });

                println!("{}", format!("Output Stored In: {}.txt", file_name).green());
            } else {
                eprintln!("{}", format!("Nmap command failed with error: {}", String::from_utf8_lossy(&output.stderr)).red());
            }
        }
        Err(e) => eprintln!("{}", format!("Error creating file: {}", e).red()),
    }
}

fn main() {
    let mut map_data_dir = env::current_dir().unwrap();
    map_data_dir.push("map_data");
    if !map_data_dir.exists() {
        create_dir_all(&map_data_dir).expect(&"Failed to create map_data directory.".red());
    }

    let mut target_ip = String::new();
    print!("\nTarget IP: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut target_ip).expect(&"Failed to read input.".red());

    let mut target_port = String::new();
    print!("Target Port (Enter for ALL): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut target_port).expect(&"Failed to read input.".red());

    let mut file_prefix = String::new();
    print!("Output File Prefix: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut file_prefix).expect(&"Failed to read input.".red());

    let mut mode_choice = String::new();
    print!("Choose Mode (Stealth / Fast): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mode_choice).expect(&"Failed to read input.".red());

    println!("{}", format!("Output Directory: {}", map_data_dir.display()).green());

    let mode_choice = mode_choice.trim().to_lowercase();
    let (min_wait_time, max_wait_time) = if mode_choice == "fast" {
        (0, 300)
    } else {
        (1800, 7200)
    };

    let target_ip = target_ip.trim();
    let target_port = target_port.trim();
    let file_prefix = file_prefix.trim();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let scan_flags = [None, Some("-sS"), Some("-sF"), Some("-sN"), Some("-sT")];

    for i in 0..=4 {
        let flag = scan_flags.get(i % scan_flags.len()).copied().flatten();

        map_target(target_ip, target_port, flag, file_prefix);

        let wait_time = generate_number(min_wait_time, max_wait_time);
        rt.block_on(wait(wait_time.try_into().unwrap()));
    }
}
