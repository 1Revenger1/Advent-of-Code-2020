use std::io::Read;

struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String
}

fn part1(passwords: &Vec<Password>) {
    let badPasswords: Vec<&Password> = passwords.iter().filter(|password| {
        let matches: Vec<&str> = password.password.matches(password.character).collect();
        return matches.len() >= password.min && matches.len() <= password.max;
    }).collect();

    print!("{}\n", badPasswords.len());
}

fn part2(passwords: &Vec<Password>) {
    let badPasswords: Vec<&Password> = passwords.iter().filter(|password| {
        let mut chars = password.password.chars();
        return (chars.nth(password.min - 1).unwrap() == password.character) ^ (password.password.chars().nth(password.max - 1).unwrap() == password.character);
    }).collect();

    print!("{}\n", badPasswords.len());
}

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let passwords: Vec<Password> = contents.lines()
                            .map(|s| s.trim())
                            .map(|s| {
                                let line: Vec<&str> = s.split(" ").collect();

                                let minMax: Vec<&str> = line[0].split("-").collect();
                                let character = line[1].replace(":", "");
                                return Password {
                                    min: minMax[0].parse().unwrap(),
                                    max: minMax[1].parse().unwrap(),
                                    character: character.chars().next().unwrap(),
                                    password: line[2].to_string()
                                };
                            })
                            .collect();

    part1(&passwords);
    part2(&passwords);
}
