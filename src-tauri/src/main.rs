#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
    buildops40_lib::run();
}
