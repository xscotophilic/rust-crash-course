use simple_types::{match_with, on_off, print_array, print_difference, print_distance};

fn main() {
    let coords: (f32, f32) = (20.0, 15.0);
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);
    print_difference(coords.0, coords.1);
    print_distance(coords.0, coords.1);

    let series = [0, 1, 1, 2, 3, 5, 8, 13];
    match_with(series[7]);
    match_with(series[0]);

    let mess = ([0, 1], 9.99, 99, "abcd", [(false, -1), (true, 1)]);
    on_off(mess.4[1].0);
    on_off(mess.4[0].0);
}
