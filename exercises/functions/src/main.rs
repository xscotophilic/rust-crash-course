fn main() {
    let width = 2;
    let height = 3;
    let depth = 5;

    println!(
        "Width is {}, height is {}, and depth is {}",
        width, height, depth,
    );
    println!("Area is {}", area_of(width, height));
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
