//! This test checks whether or not we can read from a reader when
//! stdin is not a terminal.

use std::io::Cursor;

extern crate rpassword;
use rpassword::read_password_with_reader;

#[cfg(unix)]
extern crate libc;

#[cfg(unix)]
fn close_stdin() {
    unsafe {
        libc::close(libc::STDIN_FILENO);
    }
}

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
fn close_stdin() {
    unsafe {
        kernel32::CloseHandle(
            kernel32::GetStdHandle(winapi::STD_INPUT_HANDLE)
        );
    }
}

#[cfg(not(any(unix, windows)))]
fn close_stdin() {
    unimplemented!()
}

fn mock_input_crlf() -> Cursor<&'static [u8]> {
    Cursor::new(&b"A mocked response.\r\n"[..])
}

fn mock_input_lf() -> Cursor<&'static [u8]> {
    Cursor::new(&b"A mocked response.\n"[..])
}

#[test]
fn can_read_from_redirected_input() {
    close_stdin();

    let response = ::read_password_with_reader(Some(mock_input_crlf())).unwrap();
    assert_eq!(response, "A mocked response.");
    let response = ::read_password_with_reader(Some(mock_input_lf())).unwrap();
    assert_eq!(response, "A mocked response.");
}
