use anyhow::{Context};
use clap::Parser;
use std::{
    collections::{HashSet},
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

type Result<T> = anyhow::Result<T>;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

enum Commands {}

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(ref path) = args.path {
        let history: Vec<(String, Vec<String>)> = get_history(path)?;

        let folders: HashSet<_> = history.iter().map(|(ext, _)| ext).collect();
        let base_path = PathBuf::from(path);

        create_folders(&base_path, &folders)?;

        for (ext, paths) in &history {
            for prev_path in paths {
                let filename = Path::new(prev_path).file_name().unwrap();
                let new_path = base_path.join(ext).join(filename);
                if let Err(err) = fs::rename(prev_path, &new_path) {
                    println!(
                        "Error: failed to move `{}` to `{}`: {}",
                        prev_path,
                        new_path.display(),
                        err
                    );
                } else {
                    println!("Moved `{}` to `{}`", prev_path, new_path.display());
                }
            }
        }

        println!(
            "Finished sorting files in `{}` according to their extensions.",
            path
        );
    }

    Ok(())
}

fn create_folders(base_path: &Path, folders: &HashSet<&String>) -> Result<()> {
    for folder in folders.iter() {
        let folder_path = base_path.join(folder);
        if folder_path.is_dir() {
            println!(
                "Folder `{}` already exists in `{}`",
                folder,
                base_path.display()
            );
        } else {
            fs::create_dir(&folder_path).with_context(|| {
                format!(
                    "Failed to create folder `{:?}` in `{:?}`",
                    folder_path.display(),
                    base_path.display()
                )
            })?;
            println!("Created folder `{}` in `{}`", folder, base_path.display());
        }
    }

    Ok(())
}

fn get_history(path: &str) -> Result<Vec<(String, Vec<String>)>> {
    let mut history: Vec<(String, Vec<String>)> = Vec::new();

    for entry in WalkDir::new(path).into_iter() {
        let entry_path = entry.as_ref().unwrap().path();
        if entry_path.is_file() {
            if let Some(os_str) = entry_path.extension() {
                history
                    .iter_mut()
                    .find(|(ext, _)| ext == os_str.to_str().unwrap())
                    .map(|(_, paths)| paths.push(entry_path.to_string_lossy().to_string()))
                    .unwrap_or_else(|| {
                        history.push((
                            os_str.to_owned().to_string_lossy().to_string(),
                            vec![entry_path.to_string_lossy().to_string()],
                        ))
                    });
            } else {
                dbg!("No extension");
            }
        } 
    }

    Ok(history)
}

// Input Output
// Take in path
// list all types of file and/or FUTURE:Folder.
// if folder, cd into that.
// sort by name, group by extension.
// create dir for each filetype
// move files of similar extension into their respective dir.
// cd back to original path.
// cd into each dir.
// sort by name.
// move files into their respective dir.
// cd back to original path.
// cd into each dir.
// sort by size.
// move files into their respective dir.
// cd back to original path.
// ...
