/*
 Instructions

Given a mathematical expression as a string you must return the result as a number.

Numbers

Number may be both whole numbers and/or decimal numbers. The same goes for the returned result.

Operators

You need to support the following mathematical operators:

Multiplication *
Division / (as floating point division)
Addition +
Subtraction -
Operators are always evaluated from left-to-right, and * and / must be evaluated before + and -.

Parentheses

You need to support multiple levels of nested parentheses, ex. (2 / (2 + 3.33) * 4) - -6

Whitespace

There may or may not be whitespace between numbers and operators.

An addition to this rule is that the minus sign (-) used for negating numbers and parentheses will never be separated by whitespace. I.e all of the following are valid expressions.

1-1    // 0
1 -1   // 0
1- 1   // 0
1 - 1  // 0
1- -1  // 2
1 - -1 // 2
1--1   // 2

6 + -(4)   // 2
6 + -( -4) // 10
And the following are invalid expressions

1 - - 1    // Invalid
1- - 1     // Invalid
6 + - (4)  // Invalid
6 + -(- 4) // Invalid
Validation

You do not need to worry about validation - you will only receive valid mathematical expressions following the above rules.

Restricted APIs

NOTE: std::process::Command is disallowed in your solution.
 */

pub fn calc(expr: &str) -> f64 {
    Parser { input: expr, pos: 0 }.parse_expr()
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn eat(&mut self, expected: char) -> bool {
        self.skip_ws();
        if self.peek() == Some(expected) {
            self.pos += expected.len_utf8();
            true
        } else {
            false
        }
    }

    // expr := term (('+' | '-') term)*
    fn parse_expr(&mut self) -> f64 {
        let mut value = self.parse_term();
        loop {
            if self.eat('+') {
                value += self.parse_term();
            } else if self.eat('-') {
                value -= self.parse_term();
            } else {
                break;
            }
        }
        value
    }

    // term := factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> f64 {
        let mut value = self.parse_factor();
        loop {
            if self.eat('*') {
                value *= self.parse_factor();
            } else if self.eat('/') {
                value /= self.parse_factor();
            } else {
                break;
            }
        }
        value
    }

    // factor := '-' factor | '(' expr ')' | number
    fn parse_factor(&mut self) -> f64 {
        self.skip_ws();
        if self.peek() == Some('-') {
            self.pos += 1;
            return -self.parse_factor();
        }
        if self.eat('(') {
            let value = self.parse_expr();
            self.eat(')');
            return value;
        }
        self.parse_number()
    }

    fn parse_number(&mut self) -> f64 {
        self.skip_ws();
        let start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '.') {
            self.pos += 1;
        }
        self.input[start..self.pos].parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }
}
