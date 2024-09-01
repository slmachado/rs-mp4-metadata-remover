use std::process::Command;
use std::fs::{self, File};
use std::env;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};

fn remove_metadata(input_file: &str) {
    // Preserve the file extension when creating the temporary file
    let temp_file = format!("{}.temp{}", input_file, Path::new(input_file).extension().and_then(|ext| ext.to_str()).map(|ext| format!(".{}", ext)).unwrap_or_default());

    let status = Command::new("ffmpeg")
        .args(&["-i", input_file, "-map_metadata", "-1", "-c", "copy", &temp_file])
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        fs::rename(&temp_file, input_file).expect("Error overwriting the original file");
        println!("Metadata removed and file overwritten: {}", input_file);
    } else {
        eprintln!("Error removing metadata from: {}", input_file);
        if Path::new(&temp_file).exists() {
            fs::remove_file(&temp_file).expect("Error removing temporary file");
        }
    }
}

fn process_files_from_list(results_file_path: &str, target_directory: &str) {
    let results_file = File::open(results_file_path).expect("Failed to open results.txt");
    let reader = BufReader::new(results_file);

    for line in reader.lines() {
        let line = line.expect("Error reading line from results.txt");

        // Check if the line ends with '.mp4'
        if line.trim().ends_with(".mp4") {
            let file_name = line.trim();
            let mut file_path = PathBuf::from(target_directory);
            file_path.push(file_name);

            // Convert file path to string
            if let Some(file_path_str) = file_path.to_str() {
                if Path::new(file_path_str).exists() {
                    remove_metadata(file_path_str);
                } else {
                    eprintln!("File not found: {}", file_path_str);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <results_file_directory> <target_directory>", args[0]);
        std::process::exit(1);
    }

    let results_file_directory = &args[1];
    let target_directory = &args[2];

    let mut results_file_path = PathBuf::from(results_file_directory);
    results_file_path.push("results.txt");

    if !Path::new(&results_file_path).exists() {
        eprintln!("Error: results.txt not found in the specified directory.");
        std::process::exit(1);
    }

    if !Path::new(target_directory).is_dir() {
        eprintln!("Error: The target directory is not a valid directory.");
        std::process::exit(1);
    }

    process_files_from_list(results_file_path.to_str().unwrap(), target_directory);
}
