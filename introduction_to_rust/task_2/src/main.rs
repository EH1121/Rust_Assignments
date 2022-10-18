fn input_remainder() -> i32{
    let mut param1 = String::new();
    let mut param2 = String::new();

    std::io::stdin().read_line(&mut param1).expect("param1 Input Failed");
    std::io::stdin().read_line(&mut param2).expect("param2 Input Failed");

    let x = param1.strip_suffix("\r\n").unwrap();
    let y = param2.strip_suffix("\r\n").unwrap();

    let x_int = x.parse::<i32>().unwrap();
    let y_int = y.parse::<i32>().unwrap();

    return x_int % y_int;
}

fn main() {
    println!("{}", input_remainder());
}
