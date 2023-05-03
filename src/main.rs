use anyhow::Context;
use clap::Parser;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

type Result<T> = anyhow::Result<T>;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

// enum Commands {}

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(ref path) = args.path {
        let path_ext: Vec<(String, Vec<String>)> = read_path_extensions(path)?;

        let folders: HashSet<_> = path_ext.iter().map(|(ext, _)| ext).collect();
        let base_path = PathBuf::from(path);

        create_folders(&base_path, &folders)?;

        for (ext, paths) in &path_ext {
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

fn read_path_extensions(path: &str) -> Result<Vec<(String, Vec<String>)>> {
    let mut paths: Vec<(String, Vec<String>)> = Vec::new();

    for entry in WalkDir::new(path).into_iter() {
        let entry_path = entry.as_ref().unwrap().path();
        if entry_path.is_file() {
            if let Some(os_str) = entry_path.extension() {
                paths
                    .iter_mut()
                    .find(|(ext, _)| ext == os_str.to_str().unwrap())
                    .map(|(_, paths)| paths.push(entry_path.to_string_lossy().to_string()))
                    .unwrap_or_else(|| {
                        paths.push((
                            os_str.to_owned().to_string_lossy().to_string(),
                            vec![entry_path.to_string_lossy().to_string()],
                        ))
                    });
            } else {
                dbg!("No extension");
            }
        }
    }

    Ok(paths)
}

fn sort_files_by_size(folder_path: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    files.sort_by(|a, b| {
        a.metadata()
            .unwrap()
            .len()
            .cmp(&b.metadata().unwrap().len())
    });

    Ok(files)
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_read_path_extensions() {
        let path = ".";
        let result = read_path_extensions(path);
        assert!(result.is_ok());
        let paths: Vec<(String, Vec<String>)> = result.unwrap();
        // Check that the vector contains tuples with the expected format.
        for (ext, files) in paths {
            assert_eq!(ext.is_empty(), false);
            assert_eq!(files.is_empty(), false);
            for file in files {
                assert!(file.ends_with(&format!(".{}", ext)));
            }
        }
    }

    #[test]
    fn test_create_folders() -> Result<()> {
        // Create a temporary directory.
        let temp_dir = tempfile::tempdir()?;
        let base_path = temp_dir.path();
        assert!(base_path.to_str().unwrap().to_string().starts_with("/tmp")); // "/tmp/.tmphLZCpR"

        // Create a set of folders, some which already exist.
        let binding = ["folder1", "folder2", "folder3"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let folders: HashSet<&String> = binding.iter().collect();
        let existing_folder_path = base_path.join("folder2");
        fs::create_dir(existing_folder_path)?;

        // Call the create_folders function.
        let result = create_folders(base_path, &folders);

        // Assert that the function succeeded and created the expected folders.
        assert!(result.is_ok());
        let created_folders: HashSet<String> = fs::read_dir(base_path)?
            .filter_map(|x| x.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        let created_folders: HashSet<&String> = created_folders.iter().collect();
        assert_eq!(created_folders, folders);

        Ok(())
    }
}
