enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self {
            Shot::Bullseye => 5,
            Shot::Miss => 0,
            Shot::Hit(x) => {
                if x < 3.0 {
                    return 2;
                } else {
                    return 1;
                }
            }
        }
    }
}

fn main() {
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    for coord in arrow_coords {
        coord.print_description();
        let distance_from_center = coord.distance_from_center();
        if distance_from_center < 1.0 {
            shots.push(Shot::Bullseye);
        } else if distance_from_center < 5.0 {
            shots.push(Shot::Hit(distance_from_center));
        } else {
            shots.push(Shot::Miss);
        };
    }

    let mut total = 0;
    for shot in shots {
        total += shot.points();
    }

    println!("Final point total is: {}", total);
}

#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y
        );
    }
}

fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}
