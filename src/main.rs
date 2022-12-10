use flate2::bufread::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

mod lib;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            lib::init();
        }
        "cat-file" => {
            let mode: Vec<&str> = args[2].split('-').collect();
            lib::cat_file(mode[1], &args[3]);
        }
        "hash-object" => {
            let write = args[2] == "-w";
            let path = if write {
                args[3].as_str()
            } else {
                args[2].as_str()
            };
            let hashResult = lib::hash_object(path, write);
            match hashResult {
                Ok(h)=>{
                    println!("{h}");
                }
                Err(e)=>{
                    println!("failed to hash object {}",e.to_string());
                }
            }
        }
        _ => {
            println!("command not found: {}", args[1]);
        }
    }
}
