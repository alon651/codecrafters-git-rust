use flate2::bufread::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            hande_init();
        }
        "cat-file" => {
            let mode: Vec<&str> = args[2].split('-').collect();
            handle_cat_file(mode[1], &args[3]);
        }
        _ => {
            println!("command not found: {}", args[1]);
        }
    }
}

fn handle_cat_file(mode: &str, blob_sha: &str) {
    let split = blob_sha.split_at(2);
    let path = format!(".git/objects/{}/{}", split.0, split.1);
    let file = File::open(&path);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut d = ZlibDecoder::new(reader);
            let mut s = String::new();
            d.read_to_string(&mut s).unwrap();
            let split: Vec<&str> = s.split('\0').collect();
            let header: Vec<&str> = split[0].split(" ").collect();
            match mode {
                "p"=>{
                    print!("{}",split[1])
                }
                "s"=>{
                    println!("{}",header[1]);
                }
                "t"=>{
                    println!("{}",header[0]);
                }
                _=>{
                    println!("invalid option:{mode}");
                }
            }
        }
        _=>{
            println!("invalid object at {}",&path);
        }
    }
}

fn hande_init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}
