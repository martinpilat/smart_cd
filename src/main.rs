use std::{collections::HashMap, env::args, path::PathBuf, str::FromStr};
use serde_json;
use home;

type PathDict = HashMap<String, String>;

fn get_paths() -> PathDict {
    let mut path = home::home_dir().unwrap();
    path.push(".paths.json");
    let data = std::fs::read_to_string(path).unwrap_or(String::from("{}"));
    serde_json::from_str(&data).unwrap()
}

fn save_paths(new_data: &PathDict) {
    let mut path = home::home_dir().unwrap();
    path.push(".paths.json");
    std::fs::write(path, serde_json::to_string(new_data).unwrap()).unwrap();
}


fn add(alias: &str, path: &str) {
    let mut path = PathBuf::from_str(path).unwrap();
    if let Ok(pathb) = path.canonicalize() {
        path = pathb.as_path().to_owned();
    } else {
        println!("File not found: {:?}", path);
        return;
    }
    let mut dict = get_paths();
    dict.insert(alias.to_owned(), path.to_str().unwrap().to_owned());
    save_paths(&dict);
}

fn remove(alias: &str) {
    let mut dict = get_paths();
    dict.remove(alias);
    save_paths(&dict);
}

fn normalize(path: &str) -> String {
    if path.starts_with("\\\\?\\") {
        path[4..].to_string()
    } else {
        path.to_string()
    }
}

fn get_path(alias: &str) -> String {
    let dict = get_paths();
    let path = dict.get(alias).unwrap_or(&String::from(alias)).to_owned();
    normalize(&path)
}

fn print_list() {
    let data = get_paths();
    let mut paths: Vec<_> = data.iter().collect();
    paths.sort_unstable_by(|a, b| a.1.cmp(b.1));
    for (k,v) in paths {
        println!("{:10}{}", k, normalize(v));
    }
}

fn main() {
    let v = args().collect::<Vec<_>>();
    match v[1].as_str() {
        "add" => {
            add(&v[2], &v[3]);
        },
        "remove" => {
            remove(&v[2]);
        },
        "list" => {
            print_list();
        },
        _ => {
            println!("{}", get_path(&v[1]));
        }
    }
}

