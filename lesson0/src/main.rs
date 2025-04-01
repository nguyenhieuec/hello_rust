fn main() {
    println!("Hello, world! fib(10) = {}", fib(10));
    // timing function for fib(40)
    let start = std::time::Instant::now();
    println!("Hello, world! fib_iterative(40) = {}", fib_iterative(40));
    println!("Time elapsed: {:?}", start.elapsed());

    let mut some_x = 5;
    println!("The value of some_x is: {}", some_x);
    // some_x = 6; // this is not allowed. In Rust, variables are immutable by default. We must explicitly declare a variable as mutable if we want to change its value.
    some_x = 6;
    println!("The value of some_x is: {}", some_x);
    // better idea to use explicit type declaration
    let some_y: i32 = 5_000;
    println!("The value of some_y is: {}", some_y);

    // shadowing
    let some_x = some_x + 1;
    println!("The value of some_x is: {}", some_x);

    // shadowing with different types
    let some_x = "hello";
    println!("The value of some_x is: {}", some_x);

    let x = 2.0; // f64
    println!("The value of x is: {}", x);
    let y: f32 = 3.0; // f32
    println!("The value of y is: {}", y);

    let yes: bool = true;
    let no: bool = false;
    let not: bool = !yes;
    let or: bool = yes || no;
    let and: bool = yes && no;
    println!("The value of and is: {}", and);
    println!("The value of or is: {}", or);
    println!("The value of not is: {}", not);
    println!("The value of yes is: {}", yes);
    println!("The value of no is: {}", no);

    //some operator
    let x:i32 = 5;
    let y:i32 = 10;
    let gt = x > y;
    let lt = x < y;
    let eq = x == y;
    let neq = x != y;
    let ge = x >= y;
    let le = x <= y;
    println!("The value of gt is: {}", gt);
    println!("The value of lt is: {}", lt);
    println!("The value of eq is: {}", eq);
    println!("The value of neq is: {}", neq);
    println!("The value of ge is: {}", ge);
    println!("The value of le is: {}", le);
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);

    //char

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // heap allocated string
    let s = String::from("hello, 😻!");
    println!("The value of s is: {}", s);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);

    // array
    let arr:[i32; 3] = [1, 2, 3]; // fixed size.
    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    let [a, b, c]  = arr; // 
    println!("The value of a, b, c is: {}, {}, {}", a, b, c);

    // array with default values
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    // while
    let mut x = 0;
    while x < 5 {
        println!("The value of x is: {}", x);
        x += 1;
    }

    // for
    for x in 0..5 {
        println!("The value of x is: {}", x);
    }

    // for with range
    for x in (0..5).rev() {
        println!("The value of x is: {}", x);
    }

    // if
    let x = 5;

    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    }

    // if in let

    let condition = true;

    // if expression
    let number = if condition {
        5
    } else {
        6
    };

    // if statement
    if condition {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    println!("The value of number is: {}", number);

    // functions
    let x = 5;
    let y = 10;
    let sum = add(x, y);
    println!("The value of sum is: {}", sum);

    returns_nothings();

    // expressions

    let x = {
        let y = 5;
        y + 1
    };

    println!("The value of x is: {}", x);

    //scope
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn returns_nothings() {
    println!("This function returns nothing");
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}


