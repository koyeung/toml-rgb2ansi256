use std::io::{self, Read, Write};

mod patch;
mod toml_traverse;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let buf = patch::patch_doc(&buf);

    let mut output = io::stdout();
    output.write_all(buf.as_bytes())?;
    output.flush()?;

    Ok(())
}
