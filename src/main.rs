use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;

fn remove_metadata(input_file: &str, output_file: &str) {
    let status = Command::new("ffmpeg")
        .args(&["-i", input_file, "-map_metadata", "-1", "-c", "copy", output_file])
        .status()
        .expect("failed to execute ffmpeg");

    if status.success() {
        println!("Metadata removed successfully: {}", output_file);
    } else {
        eprintln!("Error while removing metadata from: {}", input_file);
    }
}

fn process_directory(directory: &str) {
    for entry in fs::read_dir(directory).expect("Directory Not Found") {
        let entry = entry.expect("Error while reading file");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("mp4") {
            let input_file = path.to_str().unwrap();
            let output_file = format!("{}/clean_{}", directory, path.file_name().unwrap().to_str().unwrap());
            remove_metadata(input_file, &output_file);
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
