const TESTING: bool = false;

fn get_input() -> String {
    let path = if TESTING {"inputs/p1/example.in"} else {"inputs/p1/input.in"};
    std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .to_owned()
}

fn main() {
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

    println!("{}", sum);

}
