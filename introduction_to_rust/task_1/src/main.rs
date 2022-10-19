fn concat_string() -> String{
    let mut param1 = String::new();
    std::io::stdin().read_line(&mut param1).expect("Input for param1 Failed");

    let mut param2 = String::new();
    std::io::stdin().read_line(&mut param2).expect("Input for param2 Failed");

    let x = String::from(param1.strip_suffix("\r\n").unwrap());
    let y = String::from(param2.strip_suffix("\r\n").unwrap());
    
    return x + " " + &y;
}

fn main() {
    println!("{}", concat_string());
}
