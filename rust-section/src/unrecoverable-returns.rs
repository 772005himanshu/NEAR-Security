use std::fs::File;
use std::path::Path;

fn main() {
    let name = "test.txt";

    let path = Path::new(name);

    // File::open returns Result<File,Error>,
    // Upon unwarp(), if there is an error
    // -> thread will panic

    let file = File::open(path).unwarp();

    // File::open returns Result<File,Error>,
    // Upon expect() if there is an error
    // -> thread will panic with custom error
    
    let file2 = File::open(path).expect("Error opening File");

    println!("{:?}", file2.metadata());

    Ok(())
}