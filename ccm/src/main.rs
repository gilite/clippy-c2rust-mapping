use std::env;
use std::path::Path;
use std::process::{Command};
use std::io::{self, BufRead};

// debug exe: /media/psf/Home/Desktop/is470/clippy-c2rust-mapping/target/debug/ccm

fn main() {
    // Detect if it's the gnulib directory - looks for compile_commands.json, src/ and gnulib-tests
    if !Path::new("compile_commands.json").exists() {
        panic!(
            "{}",
            "Make sure to generate a compile_commands.json file for this repository."
        );
    }

    if !Path::new("gnulib-tests").exists() || !Path::new("src").exists() {
        panic!("{}", "Invalid place for execution. Run clippy-c2rust-mapping in the Gnulib coreutils folder.");
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("find src/ gnulib-tests/ -type f -name \"*.rs\" -exec rm {} +")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("⚡ Starting C-to-Rust mappings...\n");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed: {}", stderr);
    }

    // Run the C2Rust transpile program
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let c2rust_transpiile_path = exe_path
        .parent()
        .expect("Failed to get parent path")
        .join("c2rust-transpile");

    let args = [env::current_dir()
        .expect("Failed to get current directory")
        .join("compile_commands.json")];
    let mut binding = Command::new(&c2rust_transpiile_path);
    let output = binding
        .args(args);
        // .output()
        // .expect("Failed to execute command");

    // Print transpile output
    let mut child = output.spawn().unwrap();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let mut stdout_reader = io::BufReader::new(stdout);
    let mut stderr_reader = io::BufReader::new(stderr);

    loop {
        let mut line = String::new();
        let stdout_bytes_read = stdout_reader.read_line(&mut line).unwrap();
        if stdout_bytes_read == 0 {
            println!("output loop exited");
            break;
        }
        print!("{}", line);
    }

    loop {
        let mut line = String::new();
        let stderr_bytes_read = stderr_reader.read_line(&mut line).unwrap();
        if stderr_bytes_read == 0 {
            println!("err loop exited");
            break;
        }
        print!("{}", line);
    }

    child.kill().unwrap();
    
    match child.wait() {
        Ok(status) => {
            println!("Command exited with status: {}", status);
        }
        Err(error) => {
            eprintln!("Error waiting for command to exit: {}", error);
        }
    }

    // if output.status.success() {
    //     println!("Succesfully executed C2Rust.")
    // } else {
    //     let stderr = String::from_utf8_lossy(&output.stderr);
    //     println!("Command failed: {}", stderr);
    // }

    println!("\n✅ Finished C-to-Rust mappings");
}
