use std::{
    sync::mpsc,
    thread
};
use terminal_input::{
    parse::ExtendedChars,
    user::user_handler,
};

fn main() {
    // Set up channels
    let (from_user, to_main) = mpsc::channel();
    let user_thread = thread::spawn(move || {user_handler(from_user, vec![0x18]);});

    println!("Use CTRL-X to Exit.");

    //let stdout = io::stdout();
    //stdout.lock().flush().unwrap();
    loop {
        match to_main.recv().unwrap() {
            ExtendedChars::Quit => {
                // Once we have been told to quit, first, wait for the user handler to shut down.
                user_thread.join().unwrap();
                // Finally, we break out of the loop and quit.
                break;
            },
            e => {
                println!("Received {}", e);
            },
        }
    }
}
