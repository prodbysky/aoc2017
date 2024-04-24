const TESTING: bool = false;


fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn get_input() -> String {
    let path = if TESTING {"inputs/p1/example.in"} else {"inputs/p1/input.in"};
    std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .to_owned()
}

fn part1() -> i32 {
    let input = get_input();
    let first_num = input.chars().nth(0).unwrap();
    let mut input = input.chars().peekable();
    
    let mut sum = 0;

    while let Some(num) = input.next() {
        if let None = input.peek() {
            if num == first_num {
                sum += num.to_string().parse::<i32>().unwrap();
            }
            break
        }

        if num == *input.peek().unwrap() {
            sum += num.to_string().parse::<i32>().unwrap();
        }
    }
    sum
}

fn part2() -> i32 {
    let input = get_input();
    let offset = input.len() / 2;
    let parse = |(_, x): (usize, char)| x.to_string().parse::<i32>();
    input.chars().enumerate().filter(|(i, x)| { *x == input.chars().nth((i + offset) % input.len()).unwrap()}).map(|x| parse(x).unwrap()).sum()
}
