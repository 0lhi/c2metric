use std::io;
mod length;
use length::length_calc;
mod area;
use area::area_calc;
mod mass;
use mass::mass_calc;
// mod speed;
// use speed::speed_calc;

fn calc_template(x: f64, y: f64, s: &str) -> (String, f64) {
    let end_unit: String = s.to_string();
    (end_unit, x * y)
}

fn main() {
    let mut input = String::new(); //TODO: Look at Regular Expressions later.
    io::stdin().read_line(&mut input).unwrap();
    let number: f64 = input
        .trim()
        .replace(rdlc::is_not_float, "") // rdlc::is_not_numeric
        .trim()
        .parse()
        .expect("Program crashes. Did you use two points?");
    let start_unit: String = input
        .trim()
        .to_lowercase()
        .replace(
            |c: char| !rdlc::is_not_float(c) || c == ' ', //To be replaced with not_latin.
            "",
        )
        .trim()
        .to_string();
    println!(
        "Let's see if this number {} and unit {:?} work.",
        number, start_unit
    );
    #[rustfmt::skip]
    let (mut end_unit, mut metric_main) = match start_unit.as_str() {
        // Areas
        "square inch" | "square inches" | "sqin"
        => calc_template(number, 0.0006452, "m²"),
        "square mile" | "square miles" | "sqmi"
        => calc_template(number, 2590000.0, "m²"),
        // Lengths
        "inch" | "inches" | "in"
        => calc_template(number, 0.0254, "meter"),
        "mile" | "miles" | "mi"
        => calc_template(number, 1609.0, "meter"),
        // Masses
        "ounce" | "ounces" | "oz"
        => calc_template(number, 28.35, "g"),
        "pound" | "pounds" | "lb" | "lbs"
        => calc_template(number, 453.6, "g"),
        // Speed
        "mileperhour" | "milesperhour" | "mph" | "ml/h"
        => calc_template(number, 0.447, "m/s"),
        "footpersecond" | "feetpersecond" | "ft/s" | "fps"
        => calc_template(number, 0.3048, "m/s"),
        // Temperatures
        "°f" | "degreefahrenheit" | "degreesfahrenheit"
        => calc_template(number - 32.0, 5.0 / 9.0, "°C"),
        "°r" | "degreerankine" | "degreesrankine"
        => calc_template(number - 491.67, 5.0 / 9.0, "°C"),

        _ => panic!("Error LOL"),
    };

    match end_unit.as_str() {
        "meter" => length_calc(&mut metric_main, &mut end_unit),
        "m²" => area_calc(&mut metric_main, &mut end_unit),
        "g" => mass_calc(&mut metric_main, &mut end_unit),
        "m/s" => (), //speed_calc(&mut metric_main, &mut end_unit),
        "°C" => (),
        _ => panic!("Error 2 L0L"),
    };
    println!("{} {}", metric_main, end_unit);
    if end_unit == "m/s" {
        println!("{} km/h", metric_main * 3.6)
    };
}
