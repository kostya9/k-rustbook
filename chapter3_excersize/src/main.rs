fn main() {
    // Exercise 1
    println!("Exercise 1");
    print_fahrenheit_to_celsius(0.0);
    print_fahrenheit_to_celsius(-459.67);
    print_fahrenheit_to_celsius(32.0);

    // Exercise 2
    println!("\nExercise 2");
    println!("First 10 fibonacci numbers");
    print_fibonacci(10);

    // Exercise 3
    println!("\nExercise 3");
    twelve_days();
}

// Exercise 3
fn twelve_days() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for (day, day_text) in days.iter().enumerate() {
        println!("On the {day_text} day of Christmas,");
        println!("my true love gave to me");

        let last_gift = gifts[0];
        if day == 0 {
            println!("A {last_gift}.");
        } else {
            for gift in gifts.iter().skip(1).take(day).rev() {
                println!("{gift},");
            }

            let stop_character = if day == gifts.len() - 1 { "!" } else { "." };
            println!("And a {last_gift}{stop_character}");
        }

        println!("");
    }
}

// Exercise 2
fn print_fibonacci(n_until: u32) {
    for i in 0..n_until {
        println!("{}", fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Exercise 1
fn fahrenheit_to_celsius(temp: f64) -> f64 {
    return (temp - 32.0) * (5.0 / 9.0);
}

fn print_fahrenheit_to_celsius(temp: f64) {
    let result = fahrenheit_to_celsius(temp);
    println!("{temp}F is {result}C");
}
