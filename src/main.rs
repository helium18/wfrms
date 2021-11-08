mod utils;
mod wfrms;

use std::io;

fn main() {

    let mut path_to_alarm = String::new();

    println!("Enter the absolute path to the audio file");

    io::stdin()
        .read_line(&mut path_to_alarm)
        .expect("Failed to read the path!");

    let path_to_alarm: &str = &path_to_alarm.trim()[..];

    let mut hours: String = String::new();

    println!("\nEnter the time to sleep\nFormat: hrs,mins\nExample: `0,1` where the computer sleeps for 1 minute\n");

    // Reading hours
    io::stdin()
        .read_line(&mut hours)
        .expect("Failed to read time (hours)!");

    // Calling the mega function
    wfrms::wfrms(hours, path_to_alarm);
}
