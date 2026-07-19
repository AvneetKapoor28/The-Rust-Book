fn main() {
    //Mutable variable
    println!("----------MUTABLE VARIABLE----------");
    let mut  x = 10;
    println!("Value of x is : {x}");
    x = 5;
    println!("Value of x is : {x}");

    //Declaring Constants
    const THREE_HOURS_IN_SECONDS: u32 = 3*60*60;
    println!("CONSTANT DECLARED = {THREE_HOURS_IN_SECONDS}");

    //Shadowing
    println!("\n----------SHADOWING----------");
    let s = 5;
    let s = s+1;
    {
        let s = s*2;
        println!("Value of s in inner scope = {s}");
    }
    println!("Value of s = {s}");

    println!("--Shadowing allows us to change the datatype also(using mut doesn't)--");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Before - spaces:string\nAfter - spaces:u32 = {spaces}");

    //bool and char (Scalar Datatypes)
    println!("--------------------");
    let t: bool = true;
    let c: char = 'f';
    println!("bool t= {t}");
    println!("char c= {c}");

    //Compound types
    let tup: (i32,f64,char) = (34,58.3,'f'); //Creating a tuple
    let(x,y,_z) = tup;  //Destructuring a tuple
    println!("values in tup are = {x}, {y}, {}", tup.2); //accessing value from tuple using tup.index method

    //Array
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr2 = [3;5]; //creates [3,3,3,3,3]
    let _arr_first = arr1[0];






}
