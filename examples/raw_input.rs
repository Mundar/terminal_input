use terminal_input::user::raw_input;

static QUIT_SEQUENCE: [u8; 2] = [26, 16];    // CTRL-Z & CTRL-P
fn main() {
    let stdin = raw_input(&QUIT_SEQUENCE);

    println!("Use CTRL-Z then CTRL-P to Exit.");
    for b in stdin {
        println!("Received {:?}", char::from(b));
    }
}
