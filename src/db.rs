use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_db(filename: impl AsRef<Path>) -> Vec<u64> {
    match File::open(filename) {
        Ok(file) => {
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line").parse::<u64>().unwrap())
                .collect()
        }
        Err(_) => vec![],
    }
}

pub fn write_db(tweets: &Vec<u64>, filename: impl AsRef<Path>) {
    let mut f = File::create(filename).expect("Unable to create file");
    for i in tweets {
        let id = format!("{}\n", i.to_string());
        f.write_all(id.as_bytes()).expect("Unable to write data");
    }
}

#[test]
fn test_read_write() {
    let values: Vec<u64> = vec![1, 2, 3];
    write_db(&values, "test.db");
    let read_values = read_db("test.db");
    assert_eq!(values, read_values);
}
