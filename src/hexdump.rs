use std::fmt;

pub fn hexdump(data: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
    let mut width = 0;
    let mut count = 0;

    for chunk in data.chunks(16) {
        if count != 0 {
            writeln!(f)?;
        }

        write!(f, "{:08x}  ", count)?;
        count += chunk.len();

        for byte in chunk {
            write!(f, "{:02x} ", byte)?;
            width += 1;
            if width == 8 {
                write!(f, " ")?;
            }
        }

        while width < 16 {
            write!(f, "   ")?;
            width += 1;
            if width == 8 {
                write!(f, " ")?;
            }
        }

        write!(f, " |")?;

        for byte in chunk {
            if byte.is_ascii_alphanumeric() || byte.is_ascii_punctuation() {
                write!(f, "{}", *byte as char)?;
            } else {
                write!(f, ".")?;
            }
        }

        write!(f, "|")?;

        width = 0;
    }

    Ok(())
}
