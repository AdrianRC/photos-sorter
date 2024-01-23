use std::fs;
use std::path::Path;

fn main() {
    let folder_path = "./BeautyShots";
    organize_files(folder_path).expect("File organization failed");
}

fn organize_files(folder_path: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.ends_with(".ARW") {
                    let name_without_extension = &file_name[..file_name.len() - 4]; // Remove ".ARW"
                    const FIXED_SUFFIX: usize = 5; // however many chars are fixed to the end of the name
                    if name_without_extension.len() > FIXED_SUFFIX {
                        let folder_name =
                            &name_without_extension[1..name_without_extension.len() - FIXED_SUFFIX]; //also remove the leading underscore
                        let new_folder_path = Path::new(folder_path).join(folder_name);
                        fs::create_dir_all(&new_folder_path)?;

                        let new_file_path = new_folder_path.join(file_name);
                        fs::rename(path, new_file_path)?;
                    }
                }
            }
        }
    }
    println!("Ok!");
    Ok(())
}
