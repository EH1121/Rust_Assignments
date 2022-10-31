/*  
    Checks for prime by:
    1. If it is a multiplication of 2, its not prime
    2. Divide the current value by the current iteration, convert is to integer (no decimals),
    If after conversion then multiplying it by the divisor, it gives the same output as before division, then it is not prime
    3. Checks up to half of i
*/

use std::{io::{self, Write}, num::ParseIntError};

fn is_prime(i: &i32) -> bool{
    // 2: Special Case
    if i % 2 == 0 && *i != 2{
        return false;
    }  
    let max = *i / 2;
    for x in (3..(max)).step_by(2)  {
        let curr = *i / x;
        if curr * x == *i {
            return false
        }
    }
    true
}

// Helper function to convert all integer vecs into bool vecs
fn vect_prime_int_to_bool(vec: &Vec<i32>) -> Vec<bool>{
    let mut x: Vec<bool> = vec![];
    for i in vec.iter(){
        x.push(is_prime(i))
    }
    x
}

// Converts string to integer, otherwise error
fn convert_to_integer(to_parse: &str) -> Result<i32, ParseIntError>{
    let integer = to_parse.parse::<i32>()?;
    Ok(integer)
}

// For input purposes, immediately flush to console
#[allow(unused_must_use)]
fn print(line: &str){
    print!("{}", line);
    io::stdout().flush(); 
}

// Helper input function
#[allow(unused_assignments)]
fn get_input_as_integer(line: &str) -> i32{
    let mut x = 0;
    loop {
        let mut input = String::new();
        print(line);
        while io::stdin().read_line(&mut input).is_err(){
            println!("Failed to input string")
        }

        match convert_to_integer(&input.trim()){
            Ok(e) => {
                x = e;
                break;
            },
            Err(_e) => println!("Invalid Input, Please try again")
        }
    }
    x
}

fn check_prime_vect(){
    let x: i32 = get_input_as_integer("Input number of inputs to check: ");

    let mut vect:Vec<i32> = vec![];
    
    for i in 0..x{
        vect.push(get_input_as_integer(&format!("Input {}: ", &i + 1)));
    }

    let bool_vec = vect_prime_int_to_bool(&vect);
    
    for i in bool_vec{
        println!("{}", i);
    }

}

// If prime: true
// Not prime: false
fn main() {
    
    loop {
        let mut input = String::new();

        println!("=======================");
        println!("1. Check Prime");
        println!("2. Exit");
        print("Input option[1 - 2]: ");
        
        while io::stdin().read_line(&mut input).is_err(){
            println!("Failed to input string")
        }

        match input.trim(){
            "1" => check_prime_vect(),
            "2" => break,
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vect_prime_int_to_bool;

    #[test]
    fn prime(){
        let x = vec![900003701, 900007091, 59119, 59197, 64919, 56249, 48799, 43889];
        assert_eq!(vect_prime_int_to_bool(&x), vec![true, true, true, true, true, true, true, true]);
    }
    #[test]
    fn non_prime(){
        let x = vec![581, 451, 13465, 18882, 581689, 6189811, 99931, 13775177];
        assert_eq!(vect_prime_int_to_bool(&x), vec![false, false, false, false, false, false, false, false]);
    }
    #[test]
    fn prime_non_prime(){
        let x = vec![2, 5, 100, 200, 300, 301, 1001, 5167];
        assert_eq!(vect_prime_int_to_bool(&x), vec![true, true, false, false, false, false, false, true]);
    }
}

