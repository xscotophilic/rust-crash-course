const MATCHWITH: i32 = 13;

pub fn print_array(arr: [f32; 2]) {
    println!("The coordinates are [{}, {}]!", arr[0], arr[1]);
}

pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}!", x, y, (x - y).abs());
}

pub fn print_distance(x: f32, y: f32) {
    println!(
        "Distance to the origin is {}!",
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    );
}

pub fn match_with(x: i32) {
    if x == MATCHWITH {
        println!("Ding, match found {}!", x);
    } else {
        println!("{} can not be found!", x);
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    } else {
        println!("Lights are off!");
    }
}
