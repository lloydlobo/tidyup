use clap::Parser;
use std::{
    collections::{HashMap},
};
use walkdir::WalkDir;

type Result<T> = anyhow::Result<T>;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
    // command: Commands,
}

enum Commands {}

fn main() -> Result<()> {
    println!("Hello, world!");
    let args = Cli::parse();

    if let Some(path) = args.path {
        let history: &mut Vec<(String, String)> = &mut vec![];

        WalkDir::new(&path).into_iter().for_each(|entry| {
            let entry_path = entry.as_ref().unwrap().path(); // println!("{}", entry.as_ref().unwrap().path().display());
            match entry_path.extension() {
                Some(os_str) => {
                    history.push((
                        os_str.to_str().unwrap().to_owned().clone(),
                        entry_path.to_str().unwrap().to_owned().clone(),
                    )); // println!("[{:?}] {}", os_str, entry_path.display(),);
                }
                None => {
                    dbg!("No extension");
                }
            }

            // if entry_path.is_dir() { // sort_files(entry_path.to_str().unwrap());
            // } else if entry_path.is_file() { // println!("{}", entry_path.to_str().unwrap()); }
            //
        });
        // TODO: Convert map to hashset.
        let mut now_we_know_map: HashMap<String, Option<Vec<String>>> = HashMap::new();
        let history_cache = history.clone();

        for (k, _v) in history_cache.iter() {
            if now_we_know_map.get(k).is_none() {
                now_we_know_map.insert(k.to_owned(), None);
            }

            if now_we_know_map.get(k).is_some() {
                let _map_val = now_we_know_map.get(k).unwrap().clone().unwrap_or_default();

                let mut vals = vec![];
                history_cache.iter().for_each(|(a, b)| {
                    if a == k {
                        vals.push(b.to_owned());
                    }
                });
                now_we_know_map.insert(k.to_owned(), Some(vals));
            }
        }

        dbg!(&now_we_know_map);
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
