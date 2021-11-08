use crate::utils;
use std::process::Command;

// Main program that sleeps the Computer
pub(crate) fn wfrms(hours: String, path_to_alarm: &str) {
    // Getting seconds from hours
    let seconds: i32 = utils::get_number_of_seconds(hours);

    let suspend_state: String;

    // Place of suspending
    // If the time is more than 4 hours then the PC is hibernated
    // Else it is suspended (or slept)
    if suspend_to_mem(seconds) == false {
        suspend_state = String::from("disk");
    } else {
        suspend_state = String::from("mem");
    }

    let suspend_state: &str = &suspend_state[..];
    let seconds = &(seconds as i32).to_string()[..];

    // Printing suspend info
    println!("\nSuspending the computer to {}", &suspend_state);

    // Actual command
    let cmd = Command::new("sudo")
        .args(["rtcwake", "-m", suspend_state, "-s", seconds])
        .output()
        .expect(
            "Failed to sleep the PC!\n
        Try rerunning.",
        );
    utils::print_stdout_and_stderr(cmd);

    // Sleeping the thread before playing the alarm
    utils::sleep_thread(5);

    // Playing the alarm
    utils::play_alarm(path_to_alarm);
}

// Tells whether the PC should be suspended to memory or the disk
fn suspend_to_mem(seconds: i32) -> bool {
    if seconds >= 14400 {
        false
    } else {
        true
    }
}
