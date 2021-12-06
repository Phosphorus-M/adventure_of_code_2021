use std::{fs::File, io::Read};

pub fn part1(filename: &str) -> () {
    let mut content = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let content_splited: Vec<&str> = content.split("\n").collect();
    let values: Vec<i32> = content_splited[0..content_splited.len() - 1]
        .iter()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();

    let x = (0..12).map(|bit| {
            let mut c = [0,0];
            values.iter().for_each(|&x| {
              c[(x as usize >> bit) & 1] += 1
            });
            (c[1] >= c[0]) as u32
          } << bit).sum::<u32>();

    let power = x * (!x & 0xfff);

    println!("Power consumption of the submarine: {:?}", power);

    ()
}

pub fn part2(filename: &str) -> () {
    let mut content = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let content_splited: Vec<&str> = content.split("\n").collect();
    let values: Vec<u32> = content_splited[0..content_splited.len() - 1]
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    
    let oxygen_rating = calculate_rating(&values, 1);
    let co2_rating = calculate_rating(&values, 0);

    println!("Oxygen rating: {:?}", oxygen_rating);
    println!("CO2 rating: {:?}", co2_rating);
    println!("Result: {:?}", oxygen_rating * co2_rating);
    
    ()
}

pub fn calculate_rating(values: &Vec<u32>, oxygen:u32) -> u32 {
  let mut nums = values.clone();
  for bit in (0..12).rev() {
      let keep = {
          let mut c = [0, 0];
          nums.iter().for_each(|&x| c[(x as usize >> bit) & 1] += 1);
          (c[1] >= c[0]) as u32
      } ^ oxygen;
      nums.retain(|x| (x >> bit) & 1 == keep);
      if nums.len() == 1 {
          break;
      }
  }
  nums[0]
}