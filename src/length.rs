pub fn calc(metric_main: &mut f64, end_unit: &mut String) {
    if *metric_main > 1000.0 {
        *metric_main *= 0.001; // km
        *end_unit = "km".to_string();
    };
    if *metric_main < 0.1 {
        *metric_main *= 100.0;
        *end_unit = "cm".to_string();
    };
}
