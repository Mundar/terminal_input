extern crate termios;

use std::{
    io::{
        self,
        Read,
    },
    str::FromStr,
    sync::mpsc,
    thread,
    time::Duration,
};
use termios::{
    Termios,
    TCSANOW,
    tcsetattr,
};
use crate::{
    parse::{ExtendedChars, encode_extended_string},
};

/**
  This compares the first value against a partial portion of the second input value and returns
  true if it is an exact match.
  */
fn substr_cmp(buf: &[u8], seq: &[u8]) -> bool {
    if seq.len() < buf.len() { return false; }
    let mut i = 0;
    for b in buf.iter() {
        if *b == seq[i] {
            i += 1;
        }
        else {
            return false;
        }
    }
    true
}

pub fn check_quit_buffer(c: u8, buf: &mut Vec<u8>, seq: &[u8]) -> bool {
    let i = buf.len();
    if i >= seq.len() { return false; }
    buf.push(c);
    if c == seq[i] {
        buf.len() == seq.len()
    }
    else {
        // With a mismatch, we are going to adjust to see if the end of the quit buffer matches an
        // earlier point in the quit sequence.
        buf.remove(0);
        while !buf.is_empty() {
            if substr_cmp(buf, seq) {
                break;
            }
            buf.remove(0);
        }
        false
    }
}

static TIMEOUT: Duration = Duration::from_millis(100);
pub fn user_handler<T: From<ExtendedChars>>(to_main: mpsc::Sender<T>, quit_sequence: Vec<u8>) {
    let (to_user, from_stdin) = mpsc::channel();
    let stdin_quit = quit_sequence.clone();
    let stdin_thread = thread::spawn(move || {stdin_handler(to_user, &stdin_quit);});
    let mut quit_buf = Vec::new();

    let mut parse_buffer = String::new();
    loop {
        let c = if parse_buffer.is_empty() {
            from_stdin.recv().unwrap()
        }
        else {
            match from_stdin.recv_timeout(TIMEOUT) {
                Ok(c) => c,
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    to_main.send(encode_extended_string(&parse_buffer).into()).unwrap();
                    parse_buffer.clear();
                    from_stdin.recv().unwrap()
                },
                Err(e) => {
                    panic!("Recieved non-timeout error: {:?}", e);
                },
            }
        };
        // Monitor input for quit sequence.
        if check_quit_buffer(c, &mut quit_buf, &quit_sequence) {
            if 0 != parse_buffer.len() {
                to_main.send(encode_extended_string(&parse_buffer).into()).unwrap();
                parse_buffer.clear();
            }
            break;
        }
        parse_buffer.push(char::from(c));
        match ExtendedChars::from_str(&parse_buffer) {
            Err(_) => {
                to_main.send(ExtendedChars::CharString(parse_buffer.clone()).into()).unwrap();
                parse_buffer.clear();
            },
            Ok(ExtendedChars::Partial) => {},
            Ok(scc) => {
                to_main.send(scc.into()).unwrap();
                parse_buffer.clear();
            },
        }
    }
    drop(from_stdin);
    to_main.send(ExtendedChars::Quit.into()).unwrap();
    stdin_thread.join().unwrap();
}

/**
  This is a raw stdin handler that returns a stream of u8 items along a mpsc channel. For input, it
  takes a [mpsc::Sender<u8>] as the channel where it sends all of the input characters, and a
  sequence of bytes that will tell 
  */
pub fn stdin_handler<T: From<u8>>(tx: mpsc::Sender<T>, quit_sequence: &[u8]) {
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios that we will modify
    //show_debug_termios_data(&new_termios);
    new_termios.c_iflag &= !(
        termios::ICRNL |        // Don't interpret carriage return as linefeed
        termios::IXON |         // Ignore XON/XOFF flow control (^Q & ^S)
        0);
    new_termios.c_lflag &= !(
        termios::ISIG |         // Disable signal mode
        termios::ICANON |       // Disable canonical mode
        termios::ECHO |         // Disable echo
        termios::IEXTEN |       // Disable extended processing mode (probably not needed since I
                                // think it is only active in canonical mode, but just to be safe.)
        0);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    let mut quit_buf = Vec::new();
    loop {
        if let Err(_) = reader.read_exact(&mut buffer) {
            break;
        }
        if let Err(_) = tx.send(buffer[0].into()) {
            break;
        }
        if check_quit_buffer(buffer[0], &mut quit_buf, &quit_sequence) {
            break;
        }
    }
    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to original termios data
}

#[allow(dead_code)]
fn show_debug_termios_data(termios: &Termios) {
    println!("termios = {:#?}", termios);
    println!("IGNBRK flag is {}", (termios.c_iflag & termios::IGNBRK) != 0);
    println!("BRKINT flag is {}", (termios.c_iflag & termios::BRKINT) != 0);
    println!("ICRNL flag is {}", (termios.c_iflag & termios::ICRNL) != 0);
    println!("IXON flag is {}", (termios.c_iflag & termios::IXON) != 0);
    println!("IXOFF flag is {}", (termios.c_iflag & termios::IXOFF) != 0);
    println!("INPCK flag is {}", (termios.c_iflag & termios::INPCK) != 0);
    println!("ISIG flag is {}", (termios.c_lflag & termios::ISIG) != 0);
    println!("IEXTEN flag is {}", (termios.c_lflag & termios::IEXTEN) != 0);
}

#[cfg(test)]
mod tests {
    use crate::user::*;

    #[test]
    fn test_substr_cmp() {
        // Test case with empty first string.
        assert_eq!(substr_cmp(b"", b"abc"), true);
        // Test normal cases.
        for b in 0..=u8::MAX {
            let buf = vec![b];
            match b {
                b'a' => { assert_eq!(substr_cmp(&buf, b"abc"), true); },
                _ => { assert_eq!(substr_cmp(&buf, b"abc"), false); },
            }
        }
        for b in 0..=u8::MAX {
            let buf1 = vec![b, b'b'];
            let buf2 = vec![b'a', b];
            match b {
                b'a' => {
                    assert_eq!(substr_cmp(&buf1, b"abc"), true);
                    assert_eq!(substr_cmp(&buf2, b"abc"), false);
                },
                b'b' => {
                    assert_eq!(substr_cmp(&buf1, b"abc"), false);
                    assert_eq!(substr_cmp(&buf2, b"abc"), true);
                },
                _ => {
                    assert_eq!(substr_cmp(&buf1, b"abc"), false);
                    assert_eq!(substr_cmp(&buf2, b"abc"), false);
                },
            }
        }
        assert_eq!(substr_cmp(b"abc", b"abc"), true);
        // Test case where first string is too large.
        assert_eq!(substr_cmp(b"abcd", b"abc"), false);
    }

    #[test]
    fn test_check_quit_buffer() {
        // Simple test of single seqence value (Single CTRL-X)
        let test_seq = [24];
        for b in 0..=u8::MAX {
            let mut test_buf = vec![];
            assert_eq!(24 == b, check_quit_buffer(b, &mut test_buf, &test_seq));
            match b {
                24 => { assert_eq!(&test_buf, &[24]); }
                _ => { assert_eq!(&test_buf, &[]); }
            }
        }
        // Simple test of two character sequence (Double CTRL-X)
        let mut test_buf = vec![];
        let test_seq = [24, 24];
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq));
        assert_eq!(&test_buf, &[24]);
        assert_eq!(true, check_quit_buffer(24, &mut test_buf, &test_seq));
        assert_eq!(&test_buf, &[24, 24]);
        // Simple test of three character sequence (CTRL-X, CTRL-C, CTRL-X)
        let mut test_buf = vec![];
        let test_seq = [24, 3, 24];
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq));   // 1
        assert_eq!(&test_buf, &[24]);
        assert_eq!(false, check_quit_buffer(3, &mut test_buf, &test_seq));    // 2
        assert_eq!(&test_buf, &[24, 3]);
        assert_eq!(false, check_quit_buffer(3, &mut test_buf, &test_seq));    // 0
        assert_eq!(&test_buf, &[]);
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq));   // 1
        assert_eq!(&test_buf, &[24]);
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq));   // 1
        assert_eq!(&test_buf, &[24]);
        assert_eq!(false, check_quit_buffer(3, &mut test_buf, &test_seq));    // 2
        assert_eq!(&test_buf, &[24, 3]);
        assert_eq!(true, check_quit_buffer(24, &mut test_buf, &test_seq));    // 3
        assert_eq!(&test_buf, &[24, 3, 24]);
        // More complex test of three character sequence (CTRL-X, CTRL-C, CTRL-X, CTRL-D)
        let mut test_buf = vec![];
        let test_seq = [24, 3, 24, 4];
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq)); // 1
        assert_eq!(&test_buf, &[24]);
        assert_eq!(false, check_quit_buffer(3, &mut test_buf, &test_seq));  // 2
        assert_eq!(&test_buf, &[24, 3]);
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq)); // 3
        assert_eq!(&test_buf, &[24, 3, 24]);
        assert_eq!(false, check_quit_buffer(3, &mut test_buf, &test_seq));  // 2
        assert_eq!(&test_buf, &[24, 3]);
        assert_eq!(false, check_quit_buffer(24, &mut test_buf, &test_seq)); // 3
        assert_eq!(&test_buf, &[24, 3, 24]);
        assert_eq!(true, check_quit_buffer(4, &mut test_buf, &test_seq));   // 4
        assert_eq!(&test_buf, &[24, 3, 24, 4]);
        // If given an empty sequence, it will always return false.
        let mut test_buf = vec![];
        let test_seq = [];
        for b in 0..=u8::MAX {
            assert_eq!(false, check_quit_buffer(b, &mut test_buf, &test_seq)); // 3
            assert_eq!(&test_buf, &[]);
        }
    }
}
