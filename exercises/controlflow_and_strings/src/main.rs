fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let lower = 1;
    let upper = 100;
    let mut sum = 0;

    for i in lower..=upper {
        sum += i;
    }

    println!("The sum of integers from {} to {} is {}", lower, upper, sum);
}

fn double() {
    let upper = 100;
    let mut count = 0;
    let mut x = 1;

    while x <= upper {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than {}",
        count, upper
    );
}

fn count(arg: String) {
    let mut count = 0;
    let repeat_times = 5;

    loop {
        count += 1;
        println!("{}", arg);
        println!();
        if count == repeat_times {
            break;
        }
    }
}
