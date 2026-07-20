fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number {number} is divisible by 2");
    } else {
        println!("Number is not divisible by 2,3 or 4");
    }

    //Assigning the result of an if expression to a variable
    let condition = true;
    let number  = if condition {5} else {6};
    println!("number = {number}");

}
