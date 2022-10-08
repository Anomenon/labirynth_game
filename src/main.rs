#![allow(unused)]
mod key_listener;
mod map_generator;
fn main() {
    map_generator::generator();
    // print!("{}", termion::cursor::Hide);
    // key_listener::keylistener();
    // print!("{}", termion::cursor::Show);
}