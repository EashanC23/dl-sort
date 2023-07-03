mod config;
use std::path::{Path, PathBuf};
#[warn(non_snake_case)]
use std::{fs, io};
extern crate dirs;

fn main() -> io::Result<()> {
    let a = config::ConfigData;
    let user_downloads = match dirs::download_dir() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot locate downloads folder",
            ));
        }
    };

    let required_folders = [
        "Photos",
        "Videos",
        "PDFs",
        "Developments",
        "Audios",
        "Compressed",
    ];
    let dl_folder: Vec<_> = fs::read_dir(user_downloads.clone())?
        .filter_map(|res| {
            if let Ok(entry) = res {
                let path = entry.path();
                if path.is_dir() {
                    path.file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    for i in (0..required_folders.len()).rev() {
        if !dl_folder.contains(&required_folders[i].to_string()) {
            let p = user_downloads.join(&required_folders[i].to_string());
            match fs::create_dir_all(p) {
                Ok(_) => (),
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::NotFound, e));
                }
            };
        }
    }

    let audio_exts = ["mp3", "m4a", "aac", "flac", "wav"];
    let image_exts = ["jpg", "png", "heic", "jpeg"];
    let video_exts = ["mp4", "mov", "webm", "mkv", "avi"];
    let dev_exts = ["rs", "js", "jsx", "css", "java", "jar", "html"];
    let pdf_exts = ["pdf", "txt", "md", "docx", "epub"];
    let compressed_exts = ["zip", "iso", "rar", "ipa"];

    let handle_each = |file_name: &str, file_path: &PathBuf, folder: &str| -> io::Result<()> {
        let mut to_path: PathBuf = user_downloads.join(folder).join(file_name.clone());
        if Path::exists(&to_path) {
            let mut i: i16 = 1;
            while Path::exists(
                PathBuf::from(format!(
                    "{}/{}/{} {}.{}",
                    &user_downloads.to_string_lossy(),
                    folder,
                    &to_path.file_stem().unwrap().to_string_lossy(),
                    i,
                    &to_path.extension().unwrap().to_string_lossy()
                ))
                .as_path(),
            ) {
                i += 1;
            }
            let concat_path = format!(
                "{}/{}/{} {}.{}",
                &user_downloads.to_string_lossy(),
                folder,
                to_path.file_stem().unwrap().to_string_lossy(),
                i,
                to_path.extension().unwrap().to_string_lossy()
            );
            println!("AAAAAAA {}", concat_path);
            to_path = PathBuf::from(concat_path);
            println!("{} already exists ! Renaming to {:?}.", file_name, to_path);
        }
        fs::rename(file_path, to_path)?;
        println!("Moved {} to {}", file_name, folder);
        Ok(())
    };

    let dls = fs::read_dir(user_downloads.clone())?;
    for file_path in dls {
        let file_path = file_path?.path();
        if let Some(ex) = file_path.extension().and_then(|e| e.to_str()) {
            // println!("{:?}", e);
            let ext = ex.clone().to_lowercase();
            let file_name = file_path.file_name().unwrap().to_string_lossy();
            if audio_exts.contains(&ext.as_str()) {
                handle_each(&file_name, &file_path, "Audios")?;
            } else if image_exts.contains(&ext.as_str()) {
                handle_each(&file_name, &file_path, "Photos")?;
            } else if video_exts.contains(&ext.as_str()) {
                handle_each(&file_name, &file_path, "Videos")?;
            } else if dev_exts.contains(&ext.as_str()) {
                handle_each(&file_name, &file_path, "Developments")?;
            } else if pdf_exts.contains(&ext.as_str()) {
                handle_each(&file_name, &file_path, "PDFs")?;
            } else if compressed_exts.contains(&ext.as_str()) {
                handle_each(&file_name, &file_path, "Compressed")?
            }
        }
    }

    Ok(())
}
