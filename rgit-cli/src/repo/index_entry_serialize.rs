use std::io::{self, Write};

pub trait IndexEntrySerialize {
    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}
