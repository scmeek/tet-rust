use std::io;

mod core;
mod user_interfaces;

fn main() -> io::Result<()> {
    core::main()
}
