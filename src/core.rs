use crate::user_interfaces;
use std::io;

pub fn main() -> io::Result<()> {
    user_interfaces::terminal::main()
}
