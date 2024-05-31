use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f: File = open_or_create_file("hello.txt");
    let mut contents = String::new();

    f.read_to_string(&mut contents).ok();

    print!("{}", contents);
    Ok(())
}

fn open_or_create_file(f: &str) -> File {
    let file_result = File::open(f);
    
    let file: File = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(f) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    file
}
