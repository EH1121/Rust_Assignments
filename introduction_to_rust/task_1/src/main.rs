fn concat_string(param1:String, param2:String) -> String{
    return param1 + " " + &param2;
}

fn get_input() -> String{
    let mut inputs = String::new();
    std::io::stdin().read_line(&mut inputs).expect("Failed to get line");
    let x = inputs.strip_suffix("\r\n").unwrap();
    return String::from(x);
}

fn main() {
    let param1 = get_input();
    let param2 = get_input();
    let concatenated = concat_string(param1, param2);
    println!("{concatenated}");
}
