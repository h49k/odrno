use std::path::Path;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;

fn main() {
    let src = odrnos("src.txt");
    let dist = odrnos("dist.txt");

    let diff = src.difference(&dist);

    for dif in diff {
        println!("{}", dif);
    }
}

fn odrnos(filename: &str) -> HashSet<i64> {

    let file_path = Path::new(filename);
    let in_file = File::open(file_path).unwrap();
    let bufred = BufReader::new(in_file);
    let lines = bufred.lines();
    let lines = lines.filter_map(|x|x.ok());

    let mut hash = HashSet::new();

    for line in lines {

        match line.parse::<i64>() {
            Ok(a) => {
                hash.insert(a);
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    hash

}