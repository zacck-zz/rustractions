fn main() {
    let h = 3;
    println!(
        "At height: {}, the total number of spheres is: {}",
        h,
        cannonball(h)
    );

    let m = 3;
    let n = 5;

    println!("{} raised to power {} is: {}", m, n, raise_to_power(m, n));
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
