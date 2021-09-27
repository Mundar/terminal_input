use terminal_input::{
    user::parsed_input,
};

static QUIT_SEQUENCE: [u8; 2] = [26, 16];   // Ctrl-Z and Ctrl-P
fn main() {
    let stdin = parsed_input(&QUIT_SEQUENCE);

    println!("Use CTRL-Z then CTRL-P to Exit.");

    for e in stdin {
        println!("Display = \"{}\"; Debug = {:?}", e, e);
    }
}
