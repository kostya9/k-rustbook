fn main() {
    control();
}

fn control() {
    let number = 3;

    let hehe = if number < 5 {
        println!("Condition was true");
        true
    } else {
        println!("Condition was false");
        false
    };

    let mut counter = 0;
    let result = loop {
        println!("The value of counter is: {counter}");
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };
    };

    'counting_up: loop {
        println!("The value of counter is: {counter}");
        let mut remaining = 10;

        loop {
            println!("The value of remaining is: {remaining}");
            remaining -= 1;
            if remaining == 0 {
                break 'counting_up;
            }
        }
    }

    for i in 1..=4 {
        println!("The value of i is: {i}");
    }

    for i in 1..4 {
        println!("The value of i is: {i}");
    }

    for i in (1..=4).rev() {
        println!("The value of i is: {i}");
    }

    let arr = [1, 2, 3, 4];
    for element in arr {
        println!("The value of element is: {element}");
    }
}

// fn expressions() {
//     let x = 5;
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {y}");
// }

// fn params(hehe: i32) {
//     println!("The value of hehe is: {hehe}");
// }

// fn arrays() {
//     let a = [1, 2, 3, 4, 5];
//
//     println!("Please enter an array index.");
//
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//
//     let element = a[index];
//
//     println!("The value of the element at index {index} is: {element}");
// }

// fn vars() {
//     let x = 5;
//     let x = x + 1;
//
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is {x}");
//     }
//
//     let x = "   ";
//     let x = x.len();
//     let x = 99_u8;
//
//     let tup = (x, 1, 'h');
//     let unit = ();
//
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
// }
