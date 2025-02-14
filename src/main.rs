mod config;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn main() -> io::Result<()> {
    let user_downloads = dirs::download_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot locate downloads folder"))?;

    let config = match config::Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config, {}", e);
            config::Config::load_default().unwrap()
        }
    };

    let required_folders: Vec<String> = config.categories.keys().cloned().collect();

    for folder in &required_folders {
        let path = user_downloads.join(folder);
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
    }

    let dls = fs::read_dir(&user_downloads)?;
    for file_entry in dls {
        let file_path = file_entry?.path();
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_lowercase();
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            for (folder, extensions) in &config.categories {
                if extensions.contains(&ext) {
                    move_file(&file_path, &file_name, &user_downloads, folder)?;
                    break;
                }
            }
        }
    }

    Ok(())
}

fn move_file(
    file_path: &PathBuf,
    file_name: &str,
    downloads_dir: &Path,
    folder: &str,
) -> io::Result<()> {
    let mut destination = downloads_dir.join(folder).join(file_name);

    let mut counter = 1;
    while destination.exists() {
        let new_name = format!(
            "{} {}.{}",
            file_path.file_stem().unwrap().to_string_lossy(),
            counter,
            file_path.extension().unwrap().to_string_lossy()
        );
        destination = downloads_dir.join(folder).join(new_name);
        counter += 1;
    }

    fs::rename(file_path, &destination)?;
    println!("Moved {} to {}", file_name, folder);
    Ok(())
}
