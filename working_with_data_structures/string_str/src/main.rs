// Converts string slice to String datatype
fn reverse_slice(s: &str) -> String{
    String::from(s)
}

fn main() {
    let slice_string = "Strings123";
    let string_string = reverse_slice(slice_string);
    println!("&str diubah menjadi String: {}", string_string);
}
