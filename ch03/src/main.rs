fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const is {}", THREE_HOURS_IN_SECONDS);
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    {
        // Shadowing!
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is {}", x);

    // Variables
    let _guess: u32 = "42".parse().expect("Not a number");
    // _ before variable name means the compiler doesn't bother us with a warning `not used`
    let tup: (i32, f64, u8) = (500, 6.5, 1);
    println!("The value of tup is {:?}", tup);
    let a = [3, 5];
    let b = [3; 5];
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Functions

    another_function(42);
    print_labeled_measurement(5, 'h');
    let s1 = {
        let y = 5;
        y + 2
    };
    println!("S1: {}", s1);
    let s2 = plus_one(s1);
    println!("S2: {}", s2);

    // Control flow
    println!("\nControl flow\n");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("\tremaining = {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End of count = {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("Hello there! \t -> {}", x);
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("The measurement is {}{}", value, label);
}