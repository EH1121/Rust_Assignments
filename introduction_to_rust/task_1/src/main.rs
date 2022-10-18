fn concat_string(param1:String, param2:String) -> String{
    return param1 + " " + &param2;
}

fn main() {
    let param1:String = String::from("I love");
    let param2:String = String::from("Rust");
    let concatenated = concat_string(param1, param2);
    println!("{concatenated}");
}
