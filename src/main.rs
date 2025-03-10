use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get folder path from command-line arguments (for drag and drop functionality)
    let args: Vec<String> = env::args().collect();
    let folder_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // If no argument is provided (double-clicked), use the parent directory
        if let Ok(exe_path) = env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                parent.to_string_lossy().into_owned()
            } else {
                String::from(".")
            }
        } else {
            // Fallback to current directory if we can't determine exe path
            match env::current_dir() {
                Ok(path) => path.to_string_lossy().into_owned(),
                Err(_) => String::from("."),
            }
        }
    };

    match organize_files(&folder_path) {
        Ok(_) => println!("Files organized successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn organize_files(folder_path: &str) -> std::io::Result<()> {
    println!("Organizing files in: {}", folder_path);

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if extension.eq_ignore_ascii_case("jpg") || extension.eq_ignore_ascii_case("arw") {
                    if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        // Get extension length
                        let extension_len = extension.len() + 1; // +1 for the dot

                        let name_without_extension = &file_name[..file_name.len() - extension_len];
                        const FIXED_SUFFIX: usize = 5; // however many chars are fixed to the end of the name

                        if name_without_extension.len() > FIXED_SUFFIX {
                            let folder_name = &name_without_extension
                                [1..name_without_extension.len() - FIXED_SUFFIX]; //also remove the leading underscore

                            let new_folder_path = Path::new(folder_path).join(folder_name);

                            // Create directory if it doesn't exist
                            fs::create_dir_all(&new_folder_path)?;

                            let new_file_path = new_folder_path.join(file_name);
                            println!("Moving: {:?} -> {:?}", path, new_file_path);
                            fs::rename(path, new_file_path)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
