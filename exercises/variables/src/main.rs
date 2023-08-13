const MISSILES: i32 = 100;
const READY: i32 = 10;

fn main() {
    let (missiles, ready): (i32, i32) = (MISSILES, READY);
    println!("Gonna fire {} of {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
