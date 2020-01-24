fn main() {
    let h = 3;
    println!(
        "At height: {}, the total number of spheres is: {}",
        h,
        cannonball(h)
    );
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
