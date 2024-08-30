# Metadata Cleaner for MP4 Files

This project provides a Rust-based tool for removing metadata from `.mp4` files. The script processes video files within a specified directory (or a default directory) and overwrites the original files after cleaning, effectively preserving disk space. 

## Features
- **Metadata Removal**: Strips out all metadata from `.mp4` files using FFmpeg.
- **File Overwrite**: Overwrites the original files after removing metadata to avoid creating unnecessary copies and saving disk space.
- **Customizable Directory**: By default, the script processes files in `/media/sergio/nenc/vds`, but you can specify any other directory via command-line arguments.
- **Error Handling**: Includes robust error handling for file operations and ensures the integrity of the original files during the cleaning process.

## Usage
1. **Clone the repository**:
   
   ```bash
   git clone <repository_url>
   ```
2. **Build the project**:
   
   ```bash
    cargo build --release
   ```
3. **Run the script**:
- Default directory:
  
   ```bash
    cargo run
    ```
- Custom directory:
  
   ```bash
   cargo run /path/to/your/directory
    ```
