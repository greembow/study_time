use std::fs::{File, rename};
use std::io::{BufReader, BufRead};
use crate::utils;

pub fn begin_study(app_list: &String, app_dir: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = File::open(app_list)?; // open the app list
    let reader = BufReader::new(file); // create the buffered reader
    for line in reader.lines() {
        // run the following for every line
        let line = line.expect("Couldn't read a line from the app list");
        /*FIXME badly convert the result type to a String so we can use it
        This really needs to be fixed because right now if the program
        errors the user has no idea why as it just panics */  
        let f1 = format!("{}/{}", app_dir, line);
            // concatenate the name of the app to the directory
        let f2 = format!("{}.blocked", &f1); // concatenate .blocked to f1
        rename(f1, f2)?; // rename the files from foo.desktop to foo.desktop.blocked
        utils::vprintln(format!("Blocked {}", line));
    }
    Ok(())
}

pub fn end_study(app_list: &String, app_dir: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = File::open(app_list)?; // open the app list
    let reader = BufReader::new(file); // create the buffered reader
    for line in reader.lines() {
        // run the following for every line
        let line = line.expect("Couldn't read lines from the app list");
        /*FIXME badly convert the result type to a String so we can use it
        This really needs to be fixed because right now if the program
        errors the user has no idea why as it just panics */        
        let f2 = format!("{}/{}", app_dir, line);
        let f1 = format!("{}.blocked", &f2);
        rename(f1, f2)?;
        utils::vprintln(format!("Unblocked {}", line));
    }
    Ok(())
}