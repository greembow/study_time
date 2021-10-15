// a ""simple"" study timer written by greembow
// https://github.com/greembow

use std::fs::*;
use std::io::*;
extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;

fn main() -> std::io::Result<()> {
    let app_path: &str = "/home/greembow/Documents/Programming/study_time/test/"; 
        // sets the path to where the apps will be blocked, usually /usr/share/applications
    let app_list: &str = "/home/greembow/.config/studytime/apps.txt";
        // sets the path to the list of apps
    let block_suffix: &str = ".blocked";
        // sets what will be added to the name of the file once it is blocked
    let file = File::open(app_list).unwrap();
        // Open the file in read-only mode (ignoring errors)
        //TODO properly handle errors
    let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead
    for (_index, line) in reader.lines().enumerate() {
        // for each line in file, found by lines().enumerate(), run this code
        let line = line.unwrap();
            // Ignore errors here
            //TODO properly handle errors
        let f1 = format!("{}{}", app_path, line);
            // create the path to the original file by concatenating app_path and the line from the file
        let f2 = format!("{}{}{}", app_path, line, block_suffix);
            // do the same as above, but add the suffix used when blocked to the end
        std::fs::rename(f1, f2)?;
            // finally, rename the files, using f1 as the original and f2 as the result
    }

/* blocking finished, start timer */

    let timer = timer::Timer::new();
        //create a new timer object
    let (tx, rx) = channel();
        // setup transmitting and receiving channel
    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(10), move || {
        // setup a guard, then start the timer, and move it away from this thread
        //TODO make this duration customizable
        let _ignored = tx.send(());
            // if it errors, or finishes, send the result to the tx channel
    });
    rx.recv().unwrap();
        // upon receiving the result, proceed

/* timer finished, start unblocking */

// here could definitely use some work, but so far it is working!
    let file2 = File::open(app_list).unwrap();
        // define file2 as the app list
        //TODO properly handle errors
    let reader2 = BufReader::new(file2);
        // create a BufReader to read file2 (the same file)
    for (_index, line2) in reader2.lines().enumerate() {
        // for each line in file2, found by lines().enumerate(), run this code
        let line2 = line2.unwrap();
            // Ignore errors.
            //TODO properly handle errors
        let f3 = format!("{}{}{}", app_path, line2, block_suffix);
            // make f3 the file that was blocked
        let f4 = format!("{}{}", app_path, line2);
            // make f4 the original file
        println!("{}", f3);
        std::fs::rename(f3, f4)?;
            // use rename to undo the blocking
    }
    Ok(())
}