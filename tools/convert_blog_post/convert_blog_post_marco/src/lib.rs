#[macro_export]
macro_rules! set_field_value {
    ( $a:expr, $b1:expr, $b2:literal ) => {{
        if let Ok(_tmp_value) = get_field_value(&$b1, $b2) {
            $a = _tmp_value;
        }
    }};
}
