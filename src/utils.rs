pub fn vprintln(input_text: String) {
    if log::log_enabled!(log::Level::Warn) {
        // if verbose mode is on (Log Level Warn), print the line
        println!("{}", input_text);
    }
}

pub fn expand_paths(input_path: String) -> String {
    let expanded_path = shellexpand::tilde(&input_path).to_string();
    // use shellexpand to allow the usage of ~ in paths
    return expanded_path;
}

pub fn parse_time(input_time: String) -> i64 {
    let full_time: String = input_time.to_string().to_lowercase();
    // convert time to all lowercase
    let time: Vec<&str> = full_time.split_inclusive(char::is_alphabetic).collect();
    // split the string at any alphabetic char (including that char) into a vector
    let mut hours = 0;
    let mut minutes = 0; // boring variable initialization
    let mut seconds = 0;
    // i'm sure there's an easier way to do this but i'm tired
    for val in time { // for each value in the vector
        if val.ends_with('h') { // if it ends with char
            hours = val.replace('h', "").parse::<i64>().unwrap();
            // remove the letter and convert to i64
        } else if val.ends_with('m') {
            minutes = val.replace('m', "").parse::<i64>().unwrap();
        } else if val.ends_with('s') {
            seconds = val.replace('s', "").parse::<i64>().unwrap();
        }
    }
    vprintln(format!("Parsed time as: {} Hour(s), {} Minute(s), and {} Second(s)", hours, minutes, seconds));
    let final_time = (hours * 3600) + (minutes * 60) + seconds;
    // convert the individual values to seconds
    return final_time;
}