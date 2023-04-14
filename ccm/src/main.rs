use std::env;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

// debug exe: /media/psf/Home/Desktop/is470/clippy-c2rust-mapping/target/debug/ccm

fn main() {
    let start_time = Instant::now();

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
        println!("⚡ Starting C-to-Rust mappings...");
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
    let mut output = binding
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    // Live stream transpile output to console
    let stdout = output.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    let status = output.wait().unwrap();
    println!("Command exited with status: {}", status);

    println!(
        "\n✅ Finished C-to-Rust mappings, ({:.2?}s)",
        (Instant::now() - start_time).as_secs_f32()
    );
}
