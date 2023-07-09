use crate::RandomColor;

#[test]
    fn color_to_struct_works() {
        let random_color = RandomColor::rand_color_struct(100, 200, 100, 200, 33, 200, 0.0, 1.0);
        println!("This is a random color struct: {:?}", random_color);
}

#[test]
fn color_to_string_works() {
    let random_color = RandomColor::rand_color_string(100, 200, 100, 200, 33, 200, 0.0, 1.0);
    println!("This is a random color string: {}", random_color);
}

#[test]
fn color_to_struct_range_works() {
    let min_red = 100;
    let max_red = 200;
    let min_green = 100;
    let max_green = 200;
    let min_blue = 33;
    let max_blue = 200;
    let min_alpha = 0.0;
    let max_alpha = 1.0;
    let random_color = RandomColor::rand_color_struct(min_red, max_red, min_green, max_green, min_blue, max_blue, min_alpha, max_alpha);
    assert!(random_color.red >= min_red && random_color.red <= max_red);
    assert!(random_color.green >= min_green && random_color.green <= max_green);
    assert!(random_color.blue >= min_blue && random_color.blue <= max_blue);
    assert!(random_color.alpha >= min_alpha && random_color.alpha <= max_alpha);
}