
fn parse_input() -> String {
    std::fs::read_to_string("src/inputs/day_01.txt").expect("Cannot read input file!")
}

pub fn part_1() -> String {
    let input = parse_input();
    let instructions : Vec<&str> = input.split(", ").collect();

    let modifiers = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut direction = 0;

    let mut x = 0;
    let mut y = 0;

    for instruction in instructions.into_iter() {
        direction = match &instruction[..1] {
            "L" => if direction == 0 {3} else {direction - 1},
            "R" => if direction == 3 {0} else {direction + 1},
            _ => unreachable!()
        };
        let distance: i32 = instruction[1..].parse().unwrap();
        x += distance * modifiers[direction].0;
        y += distance * modifiers[direction].1;
    }

    let blocks = x.abs() + y.abs();
    blocks.to_string()
}