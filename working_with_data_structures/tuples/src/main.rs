use std::io::{self, Write};

// Returns Option, if color doesnt exist: Return None, else tuple
fn get_rgb_color(warna: &str) -> Option<(i32, i32, i32, i32)>{
    match warna{
        "merah" => Some((255, 0, 0, 0)),
        "merah muda" => Some((255, 127, 127, 0)),
        "merah tua" => Some((128, 0, 0, 0)),
        "hijau" => Some((0, 255, 0, 0)),
        "hijau muda" => Some((127, 255, 127, 0)),
        "biru" => Some((0, 0, 255, 0)),
        "biru muda" => Some((127, 127, 255, 0)),
        "biru tua" => Some((0, 0, 128, 0)),
        "kuning" => Some((255, 255, 0, 0)),
        "kuning muda" => Some((255, 255, 102, 0)),
        "kuning tua" => Some((128, 128, 0, 0)),
        _ => None
    }
}

// For input purposes, immediately flush to console
#[allow(unused_must_use)]
fn print(line: &str){
    print!("{}", line);
    io::stdout().flush(); 
}

fn main() {

    let mut input = String::new();

    loop{
        println!("Colors: ");
        println!("1. Merah\n2. Merah Muda \n3. Hijau \n4. Hijau Muda \n5. Biru \n6. Biru Muda \n7. Biru Tua \n9. Kuning \n10. Kuning Muda \n11. Kuning Tua \n");
        
        print("Input (Color Name): ");

        while io::stdin().read_line(&mut input).is_err(){
            println!("Failed to input string");
        }

        // destructuring
        let (r, g, b, a) = get_rgb_color(&input.to_lowercase().trim()).unwrap_or((-1, -1, -1, -1));
        
        if r == -1 && g == -1 && b == -1 && a == -1{
            println!("Invalid Color");
        } else{
            println!("r: {} g: {} b: {} a: {}", r, g, b, a);
        }
        print("Press Enter to Continue...");
        io::stdin().read_line(&mut input).expect("Failed to input string");
        println!("==============================================================");
    }
}

#[cfg(test)]
mod tests {
    use crate::get_rgb_color;

    #[test]
    fn test_colors(){
        assert_eq!(get_rgb_color("merah").unwrap(), (255, 0, 0, 0));
        assert_eq!(get_rgb_color("merah muda").unwrap(), (255, 127, 127, 0));
        assert_eq!(get_rgb_color("merah tua").unwrap(), (128, 0, 0, 0));

        assert_eq!(get_rgb_color("hijau").unwrap(), (0, 255, 0, 0));
        assert_eq!(get_rgb_color("hijau muda").unwrap(), (127, 255, 127, 0));

        assert_eq!(get_rgb_color("biru").unwrap(), (0, 0, 255, 0));
        assert_eq!(get_rgb_color("biru muda").unwrap(), (127, 127, 255, 0));
        assert_eq!(get_rgb_color("biru tua").unwrap(), (0, 0, 128, 0));

        assert_eq!(get_rgb_color("kuning").unwrap(), (255, 255, 0, 0));
        assert_eq!(get_rgb_color("kuning muda").unwrap(), (255, 255, 102, 0));
        assert_eq!(get_rgb_color("kuning tua").unwrap(), (128, 128, 0, 0));
    }
}