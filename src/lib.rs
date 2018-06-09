#![feature(try_from)]
use std::convert::TryFrom;

use std::fmt;

fn is_valid(text: &str) -> bool {
    text.len() >= 3 && text.len() <= 500
}

pub struct Square {
    pub buf: String,
}

impl<'a> TryFrom<&'a str> for Square {
    type Error = ();

    fn try_from(text: &'a str) -> Result<Square, Self::Error> {
        if is_valid(text) {
            Ok(Square {
                buf: to_square(text),
            })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buf)
    }
}

pub struct Star {
    pub buf: String,
}

impl<'a> TryFrom<&'a str> for Star {
    type Error = ();

    fn try_from(text: &'a str) -> Result<Star, Self::Error> {
        if is_valid(text) {
            Ok(Star {
                buf: to_star(text),
            })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buf)
    }
}

pub struct Arrow {
    pub buf: String,
}

impl<'a> TryFrom<&'a str> for Arrow {
    type Error = ();

    fn try_from(text: &'a str) -> Result<Arrow, Self::Error> {
        if is_valid(text) {
            Ok(Arrow {
                buf: to_arrow(text),
            })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buf)
    }
}

pub struct Sw {
    pub buf: String,
}

impl<'a> TryFrom<&'a str> for Sw {
    type Error = ();

    fn try_from(text: &'a str) -> Result<Sw, Self::Error> {
        if is_valid(text) {
            Ok(Sw {
                buf: to_sw(text),
            })
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Sw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buf)
    }
}

fn collect_chars(s: &str) -> Vec<char> {
    s.chars().flat_map(|c| c.to_uppercase()).collect()
}

fn sqr(x: usize) -> usize {
    x * x
}

fn to_square(s: &str) -> String {
    let chars = s.chars().flat_map(|c| c.to_uppercase()).collect::<Vec<_>>();
    let len = chars.len();
    let side = len * 2 - 1;
    let area = side * side;
    let mut buf = String::with_capacity(area * 2 - 1);
    let mut row_idx;
    let mut col_idx;
    for row in 0..side {
        row_idx = if row >= len { side - row - 1 } else { row };
        for col in 0..side {
            col_idx = if col >= len { side - col - 1 } else { col };
            buf.push(chars[len - 1 - if row_idx <= col_idx { row_idx } else { col_idx }]);
            if col != side - 1 {
                buf.push(' ');
            }
        }
        if row != side - 1 {
            buf.push('\n');
        }
    }
    buf
}

fn to_star(s: &str) -> String {
    let chars = collect_chars(s);
    let len = chars.len();
    let mut output = String::with_capacity(sqr(len * 2));

    // top lines
    for (i, &c) in chars.iter().skip(1).enumerate().rev() {
        for _ in 0..(len - i - 2) * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push('\n');
    }

    // middle line
    for &c in chars.iter().skip(1).rev() {
        output.push(c);
        output.push(' ');
    }
    for (i, &c) in chars.iter().enumerate() {
        output.push(c);
        if i == len - 1 {
            output.push('\n')
        } else {
            output.push(' ')
        }
    }

    // bottom lines
    for (i, &c) in chars.iter().skip(1).enumerate() {
        for _ in 0..(len - i - 2) * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push('\n');
    }

    output
}

fn to_arrow(s: &str) -> String {
    let chars = collect_chars(s);
    let len = chars.len();
    let mut output = String::with_capacity(sqr(len * 2));

    // top line
    for (i, &c) in chars.iter().enumerate() {
        output.push(c);
        if i == len - 1 {
            output.push('\n')
        } else {
            output.push(' ')
        }
    }

    // bottom lines
    for (i, &c) in chars.iter().skip(1).enumerate() {
        output.push(c);
        output.push(' ');
        for _ in 0..i * 2 {
            output.push(' ');
        }
        output.push(c);
        output.push('\n');
    }

    output
}

fn to_sw(s: &str) -> String {
    let chars = collect_chars(s);
    let len = chars.len();
    let mut buf = String::new();

    // top lines
    for (a, b) in (0..len).zip((1..len).rev()) {
        buf.push(chars[a]);
        buf.extend(vec![' '; (len - 2) * 2 + 1]);
        if a == 0 {
            for x in chars.iter().rev() {
                buf.push(*x);
                buf.push(' ');
            }
        } else {
            buf.push(chars[b]);
        }
        buf.push('\n');
    }

    // middle line
    for &c in chars.iter().skip(1).rev() {
        buf.push(c);
        buf.push(' ');
    }
    for (i, &c) in chars.iter().enumerate() {
        buf.push(c);
        if i == len - 1 {
            buf.push('\n')
        } else {
            buf.push(' ')
        }
    }

    // bottom lines
    for (a, b) in (1..len).zip((0..len - 1).rev()) {
        if b == 0 {
            for x in chars.iter() {
                buf.push(*x);
                buf.push(' ');
            }
            buf.extend(vec![' '; (len - 2) * 2]);
        } else {
            buf.extend(vec![' '; (len - 1) * 2]);
            buf.push(chars[a]);
            buf.extend(vec![' '; (len - 2) * 2 + 1]);
        }
        buf.push(chars[b]);
        buf.push('\n');
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_text_square(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T T T T T T T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("T X E E E X T"));
        assert_eq!(lines.next(), Some("T X X X X X T"));
        assert_eq!(lines.next(), Some("T T T T T T T"));
    }

    fn assert_text_star(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T     T     T"));
        assert_eq!(lines.next(), Some("  X   X   X"));
        assert_eq!(lines.next(), Some("    E E E"));
        assert_eq!(lines.next(), Some("T X E T E X T"));
        assert_eq!(lines.next(), Some("    E E E"));
        assert_eq!(lines.next(), Some("  X   X   X"));
        assert_eq!(lines.next(), Some("T     T     T"));
    }

    fn assert_text_arrow(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T E X T"));
        assert_eq!(lines.next(), Some("E E"));
        assert_eq!(lines.next(), Some("X   X"));
        assert_eq!(lines.next(), Some("T     T"));
    }

    fn assert_text_sw(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("R         T S U R U R "));
        assert_eq!(lines.next(), Some("U         S"));
        assert_eq!(lines.next(), Some("R         U"));
        assert_eq!(lines.next(), Some("U         R"));
        assert_eq!(lines.next(), Some("S         U"));
        assert_eq!(lines.next(), Some("T S U R U R U R U S T"));
        assert_eq!(lines.next(), Some("          U         S"));
        assert_eq!(lines.next(), Some("          R         U"));
        assert_eq!(lines.next(), Some("          U         R"));
        assert_eq!(lines.next(), Some("          S         U"));
        assert_eq!(lines.next(), Some("R U R U S T         R"));
    }

    #[test]
    fn transform_square() {
        let i = "text";
        let data = to_square(i);
        assert_text_square(&data);
    }

    #[test]
    fn transform_star() {
        let i = "text";
        let data = to_star(i);
        assert_text_star(&data);
    }

    #[test]
    fn transform_arrow() {
        let i = "text";
        let data = to_arrow(i);
        assert_text_arrow(&data);
    }

    #[test]
    fn transform_sw() {
        let i = "rurust";
        let data = to_sw(i);
        assert_text_sw(&data);
    }
}
