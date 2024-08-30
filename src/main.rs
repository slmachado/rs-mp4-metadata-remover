use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;
use std::fs::rename;

fn remove_metadata(input_file: &str) {
    // Preserve the file extension when creating the temporary file
    let temp_file = format!("{}.temp{}", input_file, Path::new(input_file).extension().and_then(|ext| ext.to_str()).map(|ext| format!(".{}", ext)).unwrap_or_default());

    let status = Command::new("ffmpeg")
        .args(&["-i", input_file, "-map_metadata", "-1", "-c", "copy", &temp_file])
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        rename(&temp_file, input_file).expect("Error overwriting the original file");
        println!("Metadata removed and file overwritten: {}", input_file);
    } else {
        eprintln!("Error removing metadata from: {}", input_file);
        if Path::new(&temp_file).exists() {
            fs::remove_file(&temp_file).expect("Error removing temporary file");
        }
    }
}

fn process_directory(directory: &str) {
    for entry in fs::read_dir(directory).expect("Directory not found") {
        let entry = entry.expect("Error reading file");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("mp4") {
            let input_file = path.to_str().unwrap();
            remove_metadata(input_file);
        }
    }
}

fn main() {
    let default_directory = "/media/sergio/nenc/vds";
    let args: Vec<String> = env::args().collect();

    let directory = if args.len() > 1 {
        &args[1]
    } else {
        default_directory
    };

    if !Path::new(directory).is_dir() {
        eprintln!("Error: The specified path is not a valid directory.");
        std::process::exit(1);
    }

    process_directory(directory);
}
