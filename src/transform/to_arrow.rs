use super::{collect_chars, Bounds, Error, Result};

const ARROW_BOUNDS: Bounds = (3, 100);

pub fn to_arrow(orig: &str) -> Result<String> {
    let len = orig.len();
    if len < ARROW_BOUNDS.0 || len > ARROW_BOUNDS.1 {
        return Err(Error::InvalidLength {
            min: ARROW_BOUNDS.0,
            max: ARROW_BOUNDS.1,
        });
    }

    let chars = collect_chars(&orig);
    let len = chars.len();
    let mut buf = String::new(); // TODO: calc capacity

    // top line
    for (i, &c) in chars.iter().enumerate() {
        buf.push(c);
        if i == len - 1 {
            buf.push('\n')
        } else {
            buf.push(' ')
        }
    }

    // bottom lines
    for (i, &c) in chars.iter().skip(1).enumerate() {
        buf.push(c);
        buf.push(' ');
        for _ in 0..i * 2 {
            buf.push(' ');
        }
        buf.push(c);
        buf.push('\n');
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow() {
        let transformed = to_arrow("text").unwrap();
        let mut lines = transformed.lines();
        assert_eq!(lines.next(), Some("T E X T"));
        assert_eq!(lines.next(), Some("E E"));
        assert_eq!(lines.next(), Some("X   X"));
        assert_eq!(lines.next(), Some("T     T"));
    }
}
