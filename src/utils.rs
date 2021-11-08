use std::process::{Command, Output};

// Gets the number of seconds when supplied with time in the format `hrs,minutes`
pub(crate) fn get_number_of_seconds(time: String) -> i32 {
    let mut seconds: i32 = 0;

    let split_time: Vec<&str> = time.trim().split(",").collect();

    let hours: i32 = split_time[0].parse::<i32>().unwrap();
    let minutes: i32 = split_time[1].parse::<i32>().unwrap();

    if minutes >= 60 {
        panic!("\n\nMinutes can't be more than or equal to 60!")
    }

    seconds += hours * 3600;
    seconds += minutes * 60;

    seconds
}

// Sleeps the thread to create a delay
pub(crate) fn sleep_thread(seconds: u32) {
    Command::new("sleep")
        .arg(&seconds.to_string()[..])
        .output()
        .expect("Failed to sleep the thread");
}

// Plays the alarm
pub(crate) fn play_alarm(path_to_alarm: &str) {
    let path_to_alarm = &String::from(path_to_alarm)[..];

    println!("Playing the alarm!");

    Command::new("vlc")
        .args(["-L", path_to_alarm])
        .output()
        .expect("Failed to play alarm");
}

// Prints the stdout and stderr
pub(crate) fn print_stdout_and_stderr(command: Output) {
    let output = match String::from_utf8(command.stdout) {
        Ok(str) => str,
        Err(_) => panic!("Failed to parse stdout"),
    };

    let err = match String::from_utf8(command.stderr) {
        Ok(str) => str,
        Err(_) => panic!("Failed to parse stderr"),
    };

    println!("{} {}", output, err)
}
