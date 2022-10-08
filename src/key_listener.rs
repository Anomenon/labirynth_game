#![allow(unused)]
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

pub fn keylistener(){
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(
        stdout, 
        "{}{}", 
        termion::clear::All, 
        termion::cursor::Goto(1, 1), 
    ).unwrap();

    stdout.flush().unwrap();

    for key in stdin.keys() {
        write!(
            stdout, 
            "{}{}", 
            termion::cursor::Goto(1, 1), 
            termion::clear::CurrentLine
        ).unwrap();

        match key.unwrap() {
            Key::Char('q') => break,
            Key::Char(key)   => println!("{}", key),
            Key::Alt(key)    => println!("Alt-{}", key),
            Key::Ctrl(key)   => println!("Ctrl-{}", key), 
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => println!("<up>"),
            Key::Down      => println!("<down>"),
            _              => println!("Other"),
        }

        stdout.flush().unwrap();
    }
}
    