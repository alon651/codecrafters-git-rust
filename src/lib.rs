use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use sha1::{Digest, Sha1};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io;

pub fn cat_file(mode: &str, blob_sha: &str) {
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
                "p" => {
                    print!("{}", split[1])
                }
                "s" => {
                    println!("{}", header[1]);
                }
                "t" => {
                    println!("{}", header[0]);
                }
                _ => {
                    println!("invalid option:{mode}");
                }
            }
        }
        _ => {
            println!("invalid object at {}", &path);
        }
    }
}

pub fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    println!("Initialized git directory")
}

pub fn hash_object(path: &str, write: bool)->Result<String,io::Error> {
    let file = File::open(path);
    match file {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut file_content = String::new();
            let length = reader.read_to_string(&mut file_content).unwrap();
            file_content = format!("blob {}",length)+"\0"+&file_content;
            let mut hasher = Sha1::new();
            hasher.update(&file_content);
            let hash:[u8;20] = hasher.finalize().as_slice().try_into().expect("error");
            println!("{:#?}", hash);
            let hash_str = hash.iter().fold("".to_string(), |acc, f| {
                acc + &format!("{:01$x}", f, 2).to_string()
            });
            println!("hash_str: {hash_str}");

            if write {
                let dir = format!(".git/objects/{}",&hash_str[0..2]);
                let final_path = format!("{}/{}",dir,&hash_str[2..]);
                fs::create_dir(dir);
                let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                e.write(file_content.as_bytes()).unwrap();
                let compressed = e.finish().unwrap();
                let mut finalFile = File::create(final_path).unwrap();
                finalFile.write_all(&compressed);
            }
            return Ok(hash_str);
        }
        _ => {
            println!("file {path} no exists");
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
    };
}
