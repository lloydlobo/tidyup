use anyhow::anyhow;
use clap::Parser;
use std::{collections::HashMap, path::PathBuf, process::Command};
use walkdir::WalkDir;

type Result<T> = anyhow::Result<T>;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

enum Commands {}

fn main() -> Result<()> {
    println!("Hello, world!");
    let args = Cli::parse();
    let mut paths_schema = HashMap::new(); // TODO: Convert map to hashset.

    if let Some(ref path) = args.path {
        let history: &mut Vec<(String, String)> = &mut vec![];

        WalkDir::new(path).into_iter().for_each(|entry| {
            let entry_path = entry.as_ref().unwrap().path();
            // println!("{}", entry.as_ref().unwrap().path().display());
            match entry_path.extension() {
                Some(os_str) => {
                    history.push((
                        os_str.to_str().unwrap().to_owned(),
                        entry_path.to_str().unwrap().to_owned(),
                    ));
                }
                None => {
                    dbg!("No extension");
                }
            }
        });

        // if entry_path.is_dir() { // sort_files(entry_path.to_str().unwrap()); } else if
        // entry_path.is_file() { // println!("{}", entry_path.to_str().unwrap()); }

        history.iter().for_each(|(k, _v)| {
            if paths_schema.get(k).is_none() {
                paths_schema.insert(k.to_owned(), None);
            }
            if paths_schema.get(k).is_some() {
                let vals = history
                    .iter()
                    .filter(|(a, _b)| a == k)
                    .map(|(_a, b)| b.to_owned())
                    .collect::<Vec<_>>();

                paths_schema.insert(k.to_owned(), Some(vals));
            }
        });
    }

    dbg!(&paths_schema);

    if let Some(ref path) = args.path {
        dbg!(&path);
        println!();
        let cmd = Command::new("pwd").output()?;
        let cmd_stdout = cmd.stdout;
        println!("I am here: {}", String::from_utf8_lossy(&cmd_stdout));
        println!("Will create folders here: {}", path);
        let folders = paths_schema
            .clone()
            .drain()
            .map(|(k, _v)| k)
            .collect::<Vec<_>>();
        println!("The folders will be: `{}`", folders.join(" ,"));

        for folder in folders {
            let cmd = Command::new("mkdir")
                .arg(format!("{}{}", path, folder))
                .output()?;
            if !cmd.status.success() {
                print!(
                    "{}",
                    String::from_utf8_lossy(cmd.stderr.as_slice())
                        .to_string()
                        .as_str()
                );
            } else {
                println!("Created `{}` folder in {}", folder, path);
            }
        }

        let cmd = Command::new("ls").args(["-a", path]).output()?;
        let cmd_stdout = cmd.stdout;
        println!(
            "I am in {}: It looks like: [\n{:2}]",
            path,
            String::from_utf8_lossy(&cmd_stdout)
        );

        println!(
            r#"I will move these (file)s with similar (ext)ensions into each of their corresponding (folder)s:
        "#,
        );

        for (k, v) in paths_schema {
            let schema_path_value = v.as_ref().unwrap();
            for prev_path in schema_path_value.iter() {
                let filename = PathBuf::from(prev_path);
                let filename = &filename.file_name().unwrap();
                let new_path = format!("{base_path}{folder}", base_path = path, folder = k);
                let new_path = format!("{}/{}", &new_path, filename.to_string_lossy().to_string());
                dbg!((&prev_path, "to", &new_path));
                // TODO: See if the prev_path is in the dir at `path` so we don't repeat.
                match Command::new("mv -n")
                    .args(["-n", prev_path, &new_path])
                    .output()
                {
                    Ok(it) => {
                        if !it.status.success() {
                            println!("{:?}", anyhow!("{:?}", cmd.stderr));
                        } else if cmd.status.success() {
                            println!("Moved files to new destination.");
                        }
                    }
                    Err(err) => {
                        println!("{:?}", anyhow!("{:?}", err));
                    }
                };
            }
            dbg!(&(k, v));
        }

        // let cmd = Command::new("ls").arg("-a").spawn()?;
    }

    Ok(())
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
fn sort_files(path: &str) {
    dbg!(&path);
    // TODO
    // sort by name, group by extension.
    // create dir for each filetype
    // move files of similar extension into their respective dir.
    // cd back to original path.
    // cd into each dir.
}
