use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input("input")?;

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

    left.sort();
    right.sort();

    let mut acc = 0;
    for (i, j) in left.iter().zip(right.iter()) {
        acc += i32::abs(i - j);
    }

    println!("{acc}");

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(read_to_string(filename)?
        .lines()
        .map(String::from)
        .collect())
}
