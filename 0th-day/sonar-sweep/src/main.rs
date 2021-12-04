use std::env;
use std::fs;

fn main() {

    let file_name = "depths";

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let mut depths = contents.split_whitespace();

    let mut count = 1;
    let mut increases = 0;
    let mut previous:i32 = depths.next().unwrap().parse().unwrap();
    for depthString in depths{
        let depth:i32 = depthString.parse().unwrap();
        if previous < depth {
            increases = increases + 1;
        }
        count=count+1;
        previous = depth;
    }

    println!("Depths count: {}", count);
    println!("Increases detected: {}", increases);
}