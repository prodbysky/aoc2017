const TESTING: bool = true;


fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn get_input() -> String {
    let path = if TESTING {"inputs/d1/example.in"} else {"inputs/d1/input.in"};
    std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .to_owned()
}

fn part1() -> i32 {
    let input = get_input().parse::<i32>().unwrap();
    
    -1
}

fn part2() -> i32 {
    -1
}
