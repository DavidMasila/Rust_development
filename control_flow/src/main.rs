fn main() {
    divisble(20);
    if_in_let();
    looping();
    loop_label();
    while_loopie();
    for_loopie();
    neat_for();
}


fn divisble(x: i32) {
    if x % 4 == 0 {
        println!("number is divisble by 4");
    } else if x % 3 == 0 {
        println!("Number is divisible by 3");
    } else if x % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("{x} is neither divisible by 4, 3 or 2")
    }
}

//using an if in a let statement

fn if_in_let() {
    let condition: bool = true;
    
    let number: i32 = if condition {5} else {6};

    println!("The value of the number is: {number}");
}

//loops - for, while and loop
fn looping() {
    let mut counter: i32 = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}")
}

//loop labels
fn loop_label() {
    let mut counter: i32 = 0;
    'counting_up: loop {
        println!("Counting Up: {counter}");

        let mut remaining: i32 = 10;
        loop {
            println!("Counting down: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter ==2 {
                break 'counting_up;
            }
            remaining -=1;
        };
        counter +=1;
    };
    println!("Counting up ended at {counter}");
}

//while loop
fn while_loopie() {
    let mut number: i32 = 3;

    while number !=0 {
        print!("{number}...");

        number -=1;
    };
    println!("\nTo mars");
}

//for loop
fn for_loopie() {
    let a: [i32; 5] = [23,54,6,8,4];
    for item in a {
        println!("The value is {item}")
    }
}

//neater for loop
fn neat_for() {
    for countdown in (1..4).rev() {
        println!("{countdown}");
    }
    println!("Off we go!!!!");
}