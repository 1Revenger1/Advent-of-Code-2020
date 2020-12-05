use std::io::Read;

fn part1(numbers: &Vec<i32>) {
    for x in 0..numbers.len() {
        for y in x..numbers.len() {
            if numbers[x] + numbers[y] == 2020 {
                print!("{} * {} = {}\n", numbers[x], numbers[y], numbers[x] * numbers[y]);
                break;
            }
        }
    }
}

fn part2(numbers: &Vec<i32>) {
    for x in 0..numbers.len() {
        for y in x..numbers.len() {
            for z in y..numbers.len() {
                if numbers[x] + numbers[y] + numbers[z] == 2020 {
                    print!("{} * {} * {} = {}\n", numbers[x], numbers[y], numbers[z], numbers[z] * numbers[x] * numbers[y]);
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let numbers: Vec<i32> = contents.split("\n").map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse().unwrap())
                            .collect();

    part1(&numbers);
    part2(&numbers);
}
