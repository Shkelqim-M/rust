fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const is {}", THREE_HOURS_IN_SECONDS);
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is {}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");
    let sum = 5 + 10;
    let difference = 95.5 - 48.5;

}
