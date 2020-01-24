fn main() {
    let h = 3;
    println!(
        "At height: {}, the total number of spheres is: {}",
        h,
        cannonball(h)
    );

    let m = 5;
    let n = 7;

    println!("{} raised to power {} is: {}", m, n, raise_to_power(m, n));

    println!("{}", n % m);

    println!(
        "The greatest common divisor of {} and {}  is {}",
        n,
        m,
        greatest_common_divisor(n, m)
    );

    let num = 1789989;

    println!("Sum of digits in {} is {}", num, digit_sum(num));
}

fn cannonball(height: i32) -> i32 {
    // check for a simple case
    if height == 1 {
        1
    } else {
        // compute and stack items in layers
        height * height + cannonball(height - 1)
    }
}

fn raise_to_power(number: i32, power: i32) -> i32 {
    // check for simple case
    if power == 0 {
        1
    } else {
        number * raise_to_power(number, power - 1)
    }
}

fn greatest_common_divisor(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        greatest_common_divisor(y, x % y)
    }
}

fn digit_sum(x: i32) -> i32 {
    if x % 10 == x {
        x
    } else {
        x % 10 + digit_sum(x / 10)
    }
}
