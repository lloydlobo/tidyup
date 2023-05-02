use anyhow::{anyhow, Context};
use clap::Parser;
use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

type Result<T> = anyhow::Result<T>;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

enum Commands {}

// TODO: Errors -> It sorts recursively. let it only affect only one level deep.
fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(ref path) = args.path {
        // let history: &mut Vec<(String, String)> = &mut vec![];
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

        // let mut paths_schema = HashMap::new(); // TODO: Convert map to hashset.
        //     WalkDir::new(path).into_iter().for_each(|entry| {
        //         let entry_path = entry.as_ref().unwrap().path(); // println!("{}", entry.as_ref().unwrap().path().display());
        //         match entry_path.extension() {
        //             Some(os_str) => { history.push(( os_str.to_str().unwrap().to_owned(), entry_path.to_str().unwrap().to_owned(),)); }
        //             None => { dbg!("No extension"); }
        //         }
        //     });
        //     // if entry_path.is_dir() { // sort_files(entry_path.to_str().unwrap()); } else if
        //     // entry_path.is_file() { // println!("{}", entry_path.to_str().unwrap()); }
        //     history.iter().for_each(|(k, _v)| {
        //         if paths_schema.get(k).is_none() { paths_schema.insert(k.to_owned(), None); }
        //         if paths_schema.get(k).is_some() {
        //             let vals = history .iter() .filter(|(a, _b)| a == k) .map(|(_a, b)| b.to_owned()) .collect::<Vec<_>>();
        //             paths_schema.insert(k.to_owned(), Some(vals));
        //         }
        //     });
    }

    Ok(())

    // if let Some(ref path) = args.path {
    // let cmd = Command::new("pwd").output()?;
    // let cmd_stdout = cmd.stdout;
    // println!("I am here: {}", String::from_utf8_lossy(&cmd_stdout));
    // println!("Will create folders here: {}", path);
    // let folders = paths_schema .clone() .drain() .map(|(k, _v)| k) .collect::<Vec<_>>();
    // println!("The folders will be: `{}`", folders.join(" ,"));
    //
    // for folder in folders {
    //     let cmd = Command::new("mkdir") .arg(format!("{}/{}", path, folder)) .output()?;
    //     if !cmd.status.success() {
    //         print!( "{}", String::from_utf8_lossy(cmd.stderr.as_slice()) .to_string() .as_str());
    //     } else { println!("Created `{}` folder in {}", folder, path); }
    // }
    // let cmd = Command::new("ls").args(["-a", path]).output()?;
    // let cmd_stdout = cmd.stdout;
    // println!( "I am in {}: It looks like: [\n{:2}]", path, String::from_utf8_lossy(&cmd_stdout));
    // println!( r#"I will move these (file)s with similar (ext)ensions into each of their corresponding (folder)s: "#,);

    // for (key, value) in paths_schema {
    //     let schema_path_value = value.as_ref().unwrap();
    //     for prev_path in schema_path_value.iter() {
    //         let filename = PathBuf::from(prev_path);
    //         let filename = &filename.file_name().unwrap();
    //         let new_path = format!( "{base_path}/{folder}/{filename}", base_path = path, folder = key, filename = filename.to_string_lossy().to_string());
    //         dbg!((&prev_path, "to", &new_path)); // TODO: See if the prev_path is in the dir at `path` so we don't repeat.
    //         match Command::new("mv") .args(["-n", prev_path, &new_path]) .output() {
    //             Ok(it) => {
    //                 if !it.status.success() {
    //                     println!("{:?}", anyhow!("{:?}", cmd.stderr));
    //                 } else if cmd.status.success() {
    //                     println!("Moved files to new destination.");
    //                 }
    //             }
    //             Err(err) => { println!("{:?}", anyhow!("{:?}", err)); }
    //         };
    //     }
    // }
    // let cmd = Command::new("ls").arg("-a").spawn()?;
    // }
}

fn create_folders(base_path: &Path, folders: &HashSet<&String>) -> Result<()> {
    for folder in folders.into_iter() {
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
        } // match entry_path.extension() { Some(os_str) => { history.push(( os_str.to_str().unwrap().to_owned(), entry_path.to_str().unwrap().to_owned(),)); } None => { dbg!("No extension"); } }
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
