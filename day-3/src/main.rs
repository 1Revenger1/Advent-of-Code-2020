use std::io::Read;

struct Slope {
    dx: u32,
    dy: u32
}

fn part1(field: &Vec<Vec<bool>>) {
    let mut x = 0;
    let mut obstacles = 0;

    for y in 0..field.len() {
        if field[y][x] {
            obstacles += 1;
        }
        x += 3;
        x %= field[0].len();
    }

    print!("{}\n", obstacles);
}

fn part2(field: &Vec<Vec<bool>>) {
    let mut total_obstacles = 1;
    
    let slopes = [
        Slope {dx: 1, dy: 1},
        Slope {dx: 3, dy: 1},
        Slope {dx: 5, dy: 1},
        Slope {dx: 7, dy: 1},
        Slope {dx: 1, dy: 2}
    ];

    for slope in &slopes {
        let mut x = 0;
        let mut obstacles = 0;
    
        for y in (0..field.len()).step_by(slope.dy as usize) {
            if field[y][x] {
                obstacles += 1;
            }
            x += slope.dx as usize;
            x %= field[0].len();
        }

        total_obstacles *= obstacles;
    }

    print!("{}\n", total_obstacles);
}

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let field: Vec<Vec<bool>> = contents.lines()
                            .map(|l| l.trim())
                            .map(|l| l.chars().map(|l| l == '#').collect())
                            .collect();

    part1(&field);
    part2(&field);
}
