#[cfg(test)]
mod tests;
use rand::Rng;

// RandomColor is a representation of an rgb()-formatted color
// I may update this library to use an Enum in place of this struct
#[derive(Debug, PartialEq)]
pub struct RandomColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32,
}

impl RandomColor {
    // Using a series of min and max values, return a RandomColor struct
    pub fn rand_color_struct(min_red: u8, max_red: u8, min_green: u8, max_green: u8, min_blue: u8, max_blue: u8, min_alpha: f32, max_alpha: f32) -> Self {
        // reuse this single rand instance across all generated numbers
        let mut rng = rand::thread_rng();
        
        // using the provided min/max values to build a range, generate a random value from said range
        let rand_red = rng.gen_range(min_red..=max_red);        
        let rand_green = rng.gen_range(min_green..=max_green);
        let rand_blue = rng.gen_range(min_blue..=max_blue);
        // do some math to output only two decimal places for alpha
        let rand_alpha = (rng.gen_range(min_alpha * 100.0..=max_alpha * 100.0).round()) / 100.0;

        RandomColor { red: rand_red, green: rand_green, blue: rand_blue, alpha: rand_alpha }
    }

    // Using a series of min and max values, return a String in "rbga(...)" format
    pub fn rand_color_string(min_red: u8, max_red: u8, min_green: u8, max_green: u8, min_blue: u8, max_blue: u8, min_alpha: f32, max_alpha: f32) -> String {
        // reuse this single rand instance across all generated numbers
        let mut rng = rand::thread_rng();

        // using the provided min/max values to build a range, generate a random value from said range
        let rand_red = rng.gen_range(min_red..=max_red);        
        let rand_green = rng.gen_range(min_green..=max_green);
        let rand_blue = rng.gen_range(min_blue..=max_blue);
        let rand_alpha = rng.gen_range(min_alpha..=max_alpha);

        // use formatting annotation to limit f32 to two decimal places
        format!("rgba({}, {}, {}, {:.2})", rand_red, rand_green, rand_blue, rand_alpha)
    }
}