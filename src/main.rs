use std::process::{Command, exit};
use std::fs::{read_to_string, write};
use dirs;

const MODES: [&str; 3] = ["On", "Keyboard", "Off"];

fn main() {
    let current_mode = read_last_mode();
    let next_mode = get_next_mode(current_mode.as_str());
    mode_switch(next_mode);
}

fn mode_switch(mode: &str) {
    println!("Changing mode to: {}", mode);
    execute_mode_switch(mode);
    save_mode(mode);
}

fn execute_mode_switch(mode: &str) {
    let command_output = execute_command(&format!("ckb-next --mode {}", mode));

    if command_output.contains("ckb-next is not running.") {
        handle_ckb_not_running();
    }

    save_mode(mode);
}

fn handle_ckb_not_running() {
    execute_command_in_background("ckb-next -b &");
    mode_switch(MODES[0])
}

fn execute_command_in_background(cmd: &str) {
    let args: Vec<&str> = cmd.split_whitespace().collect();

    let child = Command::new("nohup")
        .arg(&args[0])
        .args(&args[1..])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    match child {
        Ok(_) => {
            println!("Command started in the background.");
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            exit(1);
        }
    }
}

fn execute_command(cmd: &str) -> String {
    let args: Vec<&str> = cmd.split_whitespace().collect();

    let child = Command::new(&args[0])
        .args(&args[1..])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    match child {
        Ok(process) => {
            let output = process.wait_with_output().expect("Failed to wait for command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Command executed successfully:\n{}", stdout);
                stdout.to_string()
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Command failed:\n{}", stderr);
                stderr.to_string()
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            exit(1);
        }
    }
}

fn read_last_mode() -> String {
    let file_path = get_mode_file_path();

    match read_to_string(&file_path) {
        Ok(content) => content.trim().to_string(),
        Err(_) => String::new(), // Return an empty string if the file doesn't exist or cannot be read
    }
}

fn save_mode(mode: &str) {
    let file_path = get_mode_file_path();

    if let Err(e) = write(&file_path, mode) {
        eprintln!("Error saving mode: {}", e);
    }
}

fn get_next_mode(current_mode: &str) -> &str {
    // Find the index of the current mode
    if let Some(index) = MODES.iter().position(|&mode| mode == current_mode) {
        // Calculate the next index (wrapping around if it exceeds the array size)
        let next_index = (index + 1) % MODES.len();

        // Return the mode at the next index
        MODES[next_index]
    } else {
        // If the current mode is not in the array, return the first mode as a default
        MODES[0]
    }
}

fn get_mode_file_path() -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let file_path = home_dir.join(".my_desktop/rgb/current_mode");

    // Convert the path to a string
    file_path.to_str().expect("Failed to convert file path to string").to_string()
}