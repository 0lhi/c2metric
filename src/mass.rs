// Mass-Calc

pub fn calc(metric_main: &mut f64, end_unit: &mut String) {
    if *metric_main > 1000000.0 {
        *metric_main *= 0.000001;
        *end_unit = "t".to_string();
    };
    if *metric_main > 1000.0 {
        *metric_main *= 0.001;
        *end_unit = "kg".to_string();
    };
    if *metric_main < 0.1 {
        *metric_main *= 100.0;
        *end_unit = "mg".to_string();
    };
    if *metric_main < 0.0001 {
        *metric_main *= 10000.0;
        *end_unit = "μg".to_string();
    };
}
