fn main() {
    // For loop example
    println!("For loop example:");
    for i in 1..5 {
        println!("i = {}", i);
    }

    // While loop example
    println!("\nWhile loop example:");
    let mut j = 5;
    while j > 0 {
        println!("j = {}", j);
        j -= 1;
    }

    // Loop (infinite loop) example with break
    println!("\nLoop (infinite loop) example with break:");
    let mut count = 0;
    loop {
        println!("Looping...");
        count += 1;
        if count == 3 {
            println!("Reached count = 3, breaking out of the loop.");
            break;
        }
    }
}

