fn get_value_input() -> i32{
    let mut inputs = String::new();
    std::io::stdin().read_line(&mut inputs).expect("Failed to get Number");
    let x = inputs.strip_suffix("\r\n").unwrap();
    return x.parse::<i32>().unwrap();
}

fn main() {

    let num1 = get_value_input();
    let num2 = get_value_input();

    println!("{}", num1 % num2);
}
