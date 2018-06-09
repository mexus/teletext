use std::{error::Error, fmt};

type Bounds = (usize, usize);
const ARROW_BOUNDS: Bounds = (3, 100);
const SQUARE_BOUNDS: Bounds = (2, 100);
const STAR_BOUNDS: Bounds = (3, 100);
const SW_BOUNDS: Bounds = (3, 100);

pub type TransformResult<T> = Result<T, TransformError>;

pub trait ToArrow {
    fn to_arrow(&self) -> TransformResult<String>;
}

impl<'a> ToArrow for &'a str {
    fn to_arrow(&self) -> TransformResult<String> {
        let len = self.len();
        if len < ARROW_BOUNDS.0 || len > ARROW_BOUNDS.1 {
            return Err(TransformError::invalid_length(
                ARROW_BOUNDS.0,
                ARROW_BOUNDS.1,
            ));
        }

        let chars = collect_chars(&self);
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
}

pub trait ToSquare {
    fn to_square(&self) -> TransformResult<String>;
}

impl<'a> ToSquare for &'a str {
    fn to_square(&self) -> TransformResult<String> {
        let len = self.len();
        if len < SQUARE_BOUNDS.0 || len > SQUARE_BOUNDS.1 {
            return Err(TransformError::invalid_length(
                SQUARE_BOUNDS.0,
                SQUARE_BOUNDS.1,
            ));
        }

        let chars = collect_chars(&self);
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
        Ok(buf)
    }
}

pub trait ToStar {
    fn to_star(&self) -> TransformResult<String>;
}

impl<'a> ToStar for &'a str {
    fn to_star(&self) -> TransformResult<String> {
        let len = self.len();
        if len < STAR_BOUNDS.0 || len > STAR_BOUNDS.1 {
            return Err(TransformError::invalid_length(STAR_BOUNDS.0, STAR_BOUNDS.1));
        }

        let chars = collect_chars(&self);
        let len = chars.len();
        let mut buf = String::new(); // TODO: calc capacity

        // top lines
        for (i, &c) in chars.iter().skip(1).enumerate().rev() {
            for _ in 0..(len - i - 2) * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
            buf.push(c);
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
        for (i, &c) in chars.iter().skip(1).enumerate() {
            for _ in 0..(len - i - 2) * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
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
}

pub trait ToSw {
    fn to_sw(&self) -> TransformResult<String>;
}

impl<'a> ToSw for &'a str {
    fn to_sw(&self) -> TransformResult<String> {
        let len = self.len();
        if len < SW_BOUNDS.0 || len > SW_BOUNDS.1 {
            return Err(TransformError::invalid_length(SW_BOUNDS.0, SW_BOUNDS.1));
        }

        let chars = collect_chars(&self);
        let len = chars.len();
        let mut buf = String::new(); // TODO: calc capacity

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

        Ok(buf)
    }
}

#[derive(Debug)]
pub struct TransformError {
    description: String,
}

impl TransformError {
    fn invalid_length(min: usize, max: usize) -> TransformError {
        TransformError {
            description: format!("Text must contain from {} up to {} characters", min, max),
        }
    }
}

impl Error for TransformError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for TransformError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description())
    }
}

fn collect_chars(s: &str) -> Vec<char> {
    s.chars().flat_map(|c| c.to_uppercase()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_text_arrow(text: &str) {
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("T E X T"));
        assert_eq!(lines.next(), Some("E E"));
        assert_eq!(lines.next(), Some("X   X"));
        assert_eq!(lines.next(), Some("T     T"));
    }

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
    fn transform_arrow() {
        assert_text_arrow(&"text".to_arrow().unwrap());
    }

    #[test]
    fn transform_square() {
        assert_text_square(&"text".to_square().unwrap());
    }

    #[test]
    fn transform_star() {
        assert_text_star(&"text".to_star().unwrap());
    }

    #[test]
    fn transform_sw() {
        assert_text_sw(&"rurust".to_sw().unwrap());
    }
}
