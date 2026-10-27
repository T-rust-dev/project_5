/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
 
Refactor the function above to use the `match` statement
instead of if, else if, and else.
 
Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
 
Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.
 
The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.
 
Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn colour_to_number_elif(colour: String) -> u32 {
    if colour == "red".to_string() {
        2
    } 
    else if colour == "green".to_string() {
        3
    }
    else if colour == "blue".to_string() {
        4
    }else {
        0
    }
}

fn colour_to_number_match(colour: String ) -> u32 {
    match colour.as_str() {
        "red" => 2,
        "green" => 3,
        "blue" => 4,
        _ => 0 
    }
}

fn factorial_reculsion(num: u32) -> u32 {
    
    if num == 0 {
        return 1
    }

    let next = num - 1;

    num * factorial_reculsion(next)
}
fn factorial_wo_reculsion(mut num: u32) -> u32 {
    let mut fct =  1; 
    while num > 0 {
        fct *= num;
        num -= 1;
    }
    fct
}

fn main() {
    println!("{}", factorial_reculsion(67) );
    println!("{}", factorial_wo_reculsion(67));
}
