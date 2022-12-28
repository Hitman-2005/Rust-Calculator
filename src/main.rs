use std::io;
use std::io::{stdout, Write};
fn main() {
    // Num1
    print!("Enter Num1: ");
    stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Num1: Failed to read line!");
    let convert_num1: f64 = num1.trim().parse().unwrap();
    // Num2
    print!("Enter Num2: ");
    stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Num2: Failed to read line!");
    let convert_num2: f64 = num2.trim().parse().unwrap();
    // Operator
    println!("Add: +, Subtract: -, Mul: *, Div: /");
    print!("Select Operator: ");
    stdout().flush().unwrap();
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Operator: Failed to read line!");
    let operator = operator.trim();
    // If Condtion
    if operator == "+" {
        // Add operator
        println!("Sum of {convert_num1} and {convert_num2}: {}", convert_num1 + convert_num2);
    } else if operator == "-" {
        // Sub operator
        println!("Diffrence of {convert_num1} and {convert_num2}: {}", convert_num1 - convert_num2);
    } else if operator == "*" {
        // Mul operator
        println!("Multiplication of {convert_num1} and {convert_num2}: {}", convert_num1 * convert_num2);
    } else if operator == "/" {
        // Div operator
        println!("Division of {convert_num1} and {convert_num2}: {}", convert_num1 / convert_num2);
    } else {
        println!("Operator: Wrong Here!");
    }
}
