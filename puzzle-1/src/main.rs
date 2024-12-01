use std::{collections::HashMap, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let (left, right) = input()?;

    part_1(left.clone(), right.clone())?;
    part_2(left, right)?;

    Ok(())
}

fn part_1(mut left: Vec<i32>, mut right: Vec<i32>) -> Result<(), Box<dyn Error>> {
    left.sort();
    right.sort();

    let acc = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (i, j)| acc + i32::abs(i - j));

    println!("Part 1: {acc}");

    Ok(())
}

fn part_2(left: Vec<i32>, right: Vec<i32>) -> Result<(), Box<dyn Error>> {
    let mut r_count = HashMap::new();

    for j in right {
        *r_count.entry(j).or_insert(0) += 1;
    }

    let acc = left
        .into_iter()
        .fold(0, |acc, i| acc + i * r_count.get(&i).unwrap_or(&0));

    println!("Part 2: {acc}");

    Ok(())
}

fn input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let input = read_to_string("input")?
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut left = Vec::with_capacity(input.len());
    let mut right = Vec::with_capacity(input.len());

    for i in input {
        let lr = i
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        assert_eq!(2, lr.len(), "invalid input format");
        left.push(lr[0]);
        right.push(lr[1]);
    }

    Ok((left, right))
}
