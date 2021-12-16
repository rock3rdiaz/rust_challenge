fn get_balance(input: &mut String) -> i8 {
    let mut left_count: i8 = 0;
    let mut right_count: i8 = 0;
    for c in input.chars() {
        match c {
            '(' => {
                left_count += 1;
            },
            ')' => {
                right_count += 1;
            },
            _ => println!("Undefined char"),
        }
    }
    let balance: i8 = left_count - right_count;
    for _iter in 0..balance.abs() {
        if balance > 0 {
            input.push(')');    
        } else {
            input.insert(0, '(');
        }
    }
    balance.abs()
}

fn main() {
    let mut input: String = String::from("()(()");
    let balance = get_balance(&mut input);
    println!("balance is {}, text is {}", balance, input);
}
