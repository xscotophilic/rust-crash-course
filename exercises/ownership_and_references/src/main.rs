// valid argutments: ['banana', 'bananas', 'apple', 'apples', 'black mulberry', 'black mulberries']

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    let mut material = "mud".to_string();
    println!("This material is just {}.", material);
    bedazzle(&mut material);
    println!("Wow! The material is {} now!", material);
    println!();

    inspect(&arg);

    change(&mut arg);
    println!("{}", arg);

    if predict(arg) {
        println!("Prediction: Argument can be apple(s)");
    } else {
        println!("Prediction: Argument cannot be apple(s)");
    }
}

fn bedazzle(s: &mut String) {
    *s = (*s).replace("mud", "glitter");
}

fn inspect(s: &String) {
    let valid_args = vec![
        "apple",
        "apples",
        "banana",
        "bananas",
        "black mulberry",
        "black mulberries",
    ];
    if !valid_args.contains(&s.as_str()) {
        println!("{} is not a valid argument", s);
        std::process::exit(-1);
    }

    if s.ends_with("s") || s.ends_with("es") {
        println!("Do you want some {}?", s);
    } else {
        println!("I want more {}!", s);
    }
}

fn change(s: &mut String) {
    s.insert_str(0, "Okay, let's eat ");
}

fn predict(s: String) -> bool {
    (s.ends_with("e") || s.ends_with("es")) && s.contains("pp")
}
