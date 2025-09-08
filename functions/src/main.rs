fn main() {
    println!("Hello, world!");
    another_function(56);
    let calculation: i32 = plus_one(10);
    println!("From the returned value in functon: plus one, we get {calculation}");
}


fn another_function(x: i32) {
    let y: i32 = five();
    println!("Hello from the other function. The value of your argument is {x} and the other value returned is {y}");

}

//functions with return values

fn five() -> i32 {
    5
    //function that returns a value. No semicolons are included on the end to avoid maing it a statement
    /*
     Rust is expression-based language.
     */
}

fn plus_one(z: i32) -> i32{
    z + 1
    /*
    No semicolon to ensure its an expression
    */
}