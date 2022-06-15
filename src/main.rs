use std::io;
mod area;
mod length;
mod mass;

fn metric_calc(x: f64, y: f64, s: &str) -> (String, f64) {
    let end_unit: String = s.to_string();
    (end_unit, x * y)
}

fn main() {
    let mut input = String::new(); //TODO: Look at Regular Expressions later.
    io::stdin().read_line(&mut input).unwrap();
    let number: f64 = input
        .trim()
        .replace(|c: char| !rdlc::digital_scale(c) || c == ' ', "") // rdlc::is_not_numeric
        .trim()
        .parse()
        .expect("Program crashes. Did you use two points?");
    let start_unit: String = input
        .trim()
        .to_lowercase()
        .replace(rdlc::digital_scale, "")
        .trim()
        .to_string();
    println!(
        "Let's see if this number {} and unit {:?} work.",
        number, start_unit
    );
    #[rustfmt::skip]
    let (mut end_unit, mut metric_main) = match start_unit.as_str() {
        // Areas
        "squareinch" | "squareinches"  | "sqin"
        => metric_calc(number, 0.0006452, "m²"),
        "squaremile" | "squaremiles" | "sqmi"
        => metric_calc(number, 2590000.0, "m²"),
        // Lengths
        "inch" | "inches" | "in"
        => metric_calc(number, 0.0254, "meter"),
        "mile" | "miles" | "mi"
        => metric_calc(number, 1609.0, "meter"),
        // Masses
        "longton" | "lt"
        => metric_calc(number, 28.35, "g"),
        "ounce" | "ounces" | "oz"
        => metric_calc(number, 28.35, "g"),
        "pound" | "pounds" | "lb" | "lbs"
        => metric_calc(number, 453.6, "g"),
        // Speed
        "mileperhour" | "milesperhour" | "mph" | "ml/h"
        => metric_calc(number, 0.447, "m/s"),
        "footpersecond" | "feetpersecond" | "ft/s" | "fps"
        => metric_calc(number, 0.3048, "m/s"),
        // Temperatures
        "°f" | "degreefahrenheit" | "degreesfahrenheit"
        => metric_calc(number - 32.0, 5.0 / 9.0, "°C"),
        "°r" | "degreerankine" | "degreesrankine"
        => metric_calc(number - 491.67, 5.0 / 9.0, "°C"),
        "°k" | "degreekelvin" | "degreeskelvin"
        => metric_calc(number - 273.15, 1.0, "°C"),
        "°c" | "degreecelsius" | "degreescelsius"
        => metric_calc(number, 1.0, "°C"),
        _ => panic!("Error LOL"),
    };

    match end_unit.as_str() {
        "meter" => length::calc(&mut metric_main, &mut end_unit),
        "m²" => area::calc(&mut metric_main, &mut end_unit),
        "g" => mass::calc(&mut metric_main, &mut end_unit),
        "m/s" => (),
        "°C" => (),
        _ => panic!("Error 2 L0L"),
    };
    if end_unit == "m/s" {
        println!("{} m/s / {} km/h", metric_main, metric_main * 3.6)
    } else if end_unit == "°C" && metric_main <= 0.0 {
        println!("{} °C [{:.2} °K]", metric_main, metric_main + 273.15)
    } else {
        println!("{} {}", metric_main, end_unit);
    }
}
