fn main() {
    let mut counter = 0;

    let result = loop {
        println!("{counter}");
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result = {result}\n\n");

    //Loop Labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    println!("\n\n");

    //while loop
    let mut number = 3;
    while number!=0 {
        println!("number = {number}");
        number -= 1;
    }
    println!("While loop escaped!");

    println!("\n\n");

    //for loop
    let arr = [10,20,30,40,50];
    for element in arr{
        println!("element = {element}");
    }

    //for loop with range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!")

}
