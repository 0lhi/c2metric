pub fn calc(metric_main: &mut f64, end_unit: &mut String) {
    if *metric_main > 10000.0 {
        *metric_main *= 0.001_f64.powi(2);
        *end_unit = "km²".to_string();
    };
    if *metric_main < 0.1 {
        *metric_main *= 100.0_f64.powi(2);
        *end_unit = "cm²".to_string();
    };
}
