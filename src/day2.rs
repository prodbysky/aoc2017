const TESTING: bool = false;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn get_input() -> String {
    let path = if TESTING {"inputs/d2/example.in"} else {"inputs/d2/input.in"};
    std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .to_owned()
}

fn part1() -> i32 {
    let input = get_input();

    let mut sum = 0;

    for line in input.lines() {
        let max = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).max().unwrap();
        let min = line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).min().unwrap();
        sum += max - min;
    }
    sum
}

fn part2() -> i32 {
    let input = get_input();

    let mut sum = 0;

    for line in input.lines() {
        for num1 in line.split_whitespace().map(|x| x.parse::<i32>().unwrap())  {
            for num2 in line.split_whitespace().map(|x| x.parse::<i32>().unwrap())  {
                if num1 == num2 {
                    continue
                } 
                if num1 % num2 == 0 {
                    sum += num1 / num2;
                } else if num2 % num1 == 0 {
                    sum += num2 / num1;
                }
            }
        }
    }
    sum / 2
   
}
