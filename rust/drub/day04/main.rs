use std::io;

mod first;
mod second;

fn main() -> io::Result<()> {
    first::main()?;
    second::main()
}
