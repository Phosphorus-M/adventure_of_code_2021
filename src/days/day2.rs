use std::{fs::File, io::Read};

pub fn part1(filename: &str) -> () {
    let mut content = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let content_splited: Vec<&str> = content.split("\n").collect();
    let without_the_last_line = content_splited[0..content_splited.len() - 1]
        .iter()
        .map(|x| x)
        .collect::<Vec<&&str>>();

    let mut deph = 0;
    let mut horizontal = 0;

    for i in without_the_last_line.iter() {
        let value_splited = i.split(" ").collect::<Vec<&str>>();
        match value_splited[0] {
            "forward" => horizontal += value_splited[1].parse::<i32>().unwrap(),
            "up" => deph -= value_splited[1].parse::<i32>().unwrap(),
            "down" => deph += value_splited[1].parse::<i32>().unwrap(),
            _ => println!("{:?}", value_splited[0]),
        }
    }

    println!("Horizontal: {} Deph: {}", horizontal, deph);
    println!("Result: {}", horizontal * deph);

    ()
}

pub fn part2(filename: &str) -> () {
    let mut content = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let content_splited: Vec<&str> = content.split("\n").collect();
    let without_the_last_line = content_splited[0..content_splited.len() - 1]
        .iter()
        .map(|x| x)
        .collect::<Vec<&&str>>();

    let mut aim = 0;
    let mut deph = 0;
    let mut horizontal = 0;

    for i in without_the_last_line.iter() {
        let value_splited = i.split(" ").collect::<Vec<&str>>();
        match value_splited[0] {
            "forward" => {
                horizontal += value_splited[1].parse::<i32>().unwrap();
                deph += aim * value_splited[1].parse::<i32>().unwrap();
            }
            "up" => aim -= value_splited[1].parse::<i32>().unwrap(),
            "down" => aim += value_splited[1].parse::<i32>().unwrap(),
            _ => println!("{:?}", value_splited[0]),
        }
    }

    println!("Horizontal: {} Deph: {} Aim: {}", horizontal, deph, aim);
    println!("Result: {}", horizontal * deph);
}
