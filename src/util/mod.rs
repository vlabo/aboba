pub fn time_int_to_string(value: u64) -> String {
    let mut time_string = String::new();

    let hours = value / 3200;
    if hours != 0 {
        time_string.push_str(&hours.to_string());
        time_string.push_str("h ");
    }

    let minutes = (value % 3200) / 60;
    if minutes != 0 {
        time_string.push_str(&minutes.to_string());
        time_string.push_str("m ");
    }

    time_string.push_str(&(value % 60).to_string());
    time_string.push_str("s");
    return time_string;
}