use std::{env, path::{Path, PathBuf}, fs::{self}, collections::HashMap};
use std::io::prelude::*;

fn get_file_path() -> PathBuf {
    Path::new(&env::var("HOME").unwrap()).join("todo.txt")
}

fn get_items() -> (HashMap<usize, String>, HashMap<usize, String>) {
    let path = get_file_path();
    let cont = fs::read_to_string(path).unwrap();

    let mut todo = HashMap::new();
    let mut done = HashMap::new();

    for (i, v) in cont.split("\n").enumerate() {
        if v.len() < 1 {
            continue;
        }
        if v.chars().nth(0).unwrap() == '.' {
            todo.insert(i, v[1..].to_string());
        }
        else {
            done.insert(i, v[1..].to_string());
        }
    }

    return (todo, done);
}

fn print_todo() {
    let (todo, done) = get_items();
    println!("TODO:");
    for (i, v) in todo.iter() {
        println!("  {i}: {v}");
    }
    println!("\nDONE:");
    for (i, v) in done.iter() {
        println!("  {i}: {v}");
    }
}

fn add_todo(item : &String) {
    let path = get_file_path();
    let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
    file.write_all(b".").unwrap();
    file.write_all(item.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}

fn mark_complete(item : &String) {
    let id : usize = item.parse().unwrap();
    let (mut todo, mut done) = get_items();
    if !todo.contains_key(&id) {
        println!("ID not in todo list");
        return;
    }

    let val = todo.remove(&id).unwrap();
    done.insert(id, val);

    let mut buf = String::new();
    for v in todo.values() {
        buf += &format!(".{v}\n").as_str();
    }
    for v in done.values() {
        buf += &format!("-{v}\n").as_str();
    }

    let path = get_file_path();
    fs::write(path, buf).unwrap();
}


fn main() {
    let args : Vec<_> = env::args().collect();

    if  args.len() < 2 {
        print_todo();
        return;
    }

    match args[1].as_str() {
        "add" => {
            assert!(args.len() == 3);
            add_todo(&args[2]);
        }
        "done" => {
            assert!(args.len() == 3);
            mark_complete(&args[2]);
        }
        _ => { }
    }
}
