use std::{fs::File, io::Read};

pub fn part1(filename: &str) -> () {
    let mut content = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let numbers: Vec<i32> = content
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let value = numbers
        .windows(2)
        .filter(|numbs| numbs[0] < numbs[1])
        .count();
    println!("Part 1 {:?}", value);

    let value = numbers
        .windows(3)
        .map(|numbs| numbs[0] + numbs[1] + numbs[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|numbs| numbs[0] < numbs[1])
        .count();
    println!("Part 2 {:?}", value);
    ()
}
