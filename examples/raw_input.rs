use std::{
    sync::mpsc,
    thread
};
use terminal_input::{
    user::{stdin_handler, check_quit_buffer},
};

static QUIT_SEQUENCE: [u8; 2] = [24, 3];    // CTRL-X & CTRL-C
fn main() {
    // Set up channels
    let (from_user, to_main) = mpsc::channel();
    let user_thread = thread::spawn(move || {stdin_handler(from_user, &QUIT_SEQUENCE);});
    let mut quit_buf = vec![];

    println!("Use CTRL-X & CTRL-C to Exit.");

    loop {
        let b = to_main.recv().unwrap();
        if (b' ' <= b) && (b'~' >= b) { // Normal ASCII range.
            let c = char::from(b);
            println!("Received {:?}", c);
        }
        else {                          // Outside normal ASCII range.
            println!("Received \"\\u{{{:x}}}\"", b);
        }
        if check_quit_buffer(b, &mut quit_buf, &QUIT_SEQUENCE) {
            // Once we have been told to quit, first, wait for the user handler to shut down.
            user_thread.join().unwrap();
            // Finally, we break out of the loop and quit.
            break;
        }
    }
}
