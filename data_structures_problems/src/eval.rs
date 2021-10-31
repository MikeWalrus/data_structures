use data_structures::{
    seq_list::SeqList,
    stack::{SeqStack, Stack},
    List,
};

fn main() {
    let postfix = infix_to_postfix("1+1").unwrap();
    let result = eval_postfix(postfix).unwrap();
    println!("{}", result);
}

#[derive(PartialEq, Clone, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn precedence(&self) -> u32 {
        match self {
            Operator::Add => 1,
            Operator::Subtract => 1,
            Operator::Multiply => 2,
            Operator::Divide => 2,
        }
    }

    fn apply(&self, operand1: i32, operand2: i32) -> Option<i32> {
        let f = match self {
            Operator::Add => i32::checked_add,
            Operator::Subtract => i32::checked_sub,
            Operator::Multiply => i32::checked_mul,
            Operator::Divide => i32::checked_div,
        };
        f(operand1, operand2)
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u32::partial_cmp(&self.precedence(), &other.precedence())
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Token {
    Operator(Operator),
    Num(u32),
}

const RADIX: u32 = 10;

enum NonNumber {
    Operator(Operator),
    LeftParenthesis(usize),
}

#[derive(Debug, PartialEq)]
struct ExprError {
    what: ExprErrorType,
    pos: usize,
}

#[derive(Debug, PartialEq)]
enum ExprErrorType {
    IllegalChar,
    UnmatchedParenthesis,
    I32Overflow,
}

fn infix_to_postfix(s: &str) -> Result<SeqList<Token>, ExprError> {
    let mut i = s.chars().enumerate().peekable();
    let mut ret = SeqList::new();
    let mut stack: SeqStack<NonNumber> = SeqStack::new();

    while let Some((pos, c)) = i.next() {
        match c {
            _ if c.is_digit(RADIX) => {
                let number = get_number(c, &mut i).ok_or(ExprError {
                    what: ExprErrorType::I32Overflow,
                    pos,
                })?;
                ret.push(Token::Num(number));
            }
            '(' => stack.push(NonNumber::LeftParenthesis(pos)),
            ')' => {
                let has_left_parenthesis = close_parenthesis(&mut stack, &mut ret);
                if !has_left_parenthesis {
                    return Err(ExprError {
                        what: ExprErrorType::UnmatchedParenthesis,
                        pos,
                    });
                }
            }
            _ if c.is_whitespace() => (),
            _ => handle_an_operator(&mut stack, c, &mut ret)
                .map_err(|what| ExprError { what, pos })?,
        }
    }
    while let Some(non_number) = stack.pop() {
        match non_number {
            NonNumber::Operator(op) => ret.push(Token::Operator(op)),
            NonNumber::LeftParenthesis(pos) => {
                return Err(ExprError {
                    what: ExprErrorType::UnmatchedParenthesis,
                    pos,
                })
            }
        }
    }
    Ok(ret)
}

fn close_parenthesis(stack: &mut SeqStack<NonNumber>, ret: &mut SeqList<Token>) -> bool {
    let mut has_left_parenthesis = false;
    while let Some(n) = stack.pop() {
        match n {
            NonNumber::LeftParenthesis(_) => {
                has_left_parenthesis = true;
                break;
            }
            NonNumber::Operator(op) => {
                ret.push(Token::Operator(op));
            }
        }
    }
    has_left_parenthesis
}

fn handle_an_operator(
    stack: &mut SeqStack<NonNumber>,
    c: char,
    postfix: &mut SeqList<Token>,
) -> Result<(), ExprErrorType> {
    let current_op = match c {
        '+' => Operator::Add,
        '-' => Operator::Subtract,
        '/' => Operator::Divide,
        '*' => Operator::Multiply,
        _ => return Err(ExprErrorType::IllegalChar),
    };
    while let Some(NonNumber::Operator(top)) = stack.peek() {
        if top < &current_op {
            break;
        }
        postfix.push(Token::Operator(top.clone()));
        stack.pop();
    }
    stack.push(NonNumber::Operator(current_op));
    Ok(())
}

fn get_number<T: Iterator<Item = (usize, char)>>(
    c: char,
    i: &mut std::iter::Peekable<T>,
) -> Option<u32> {
    let mut ret = c.to_digit(RADIX).unwrap();
    while let Some((_, c)) = i.peek() {
        if let Some(digit) = c.to_digit(RADIX) {
            ret = ret.wrapping_mul(RADIX).wrapping_add(digit);
            i.next();
        } else {
            break;
        }
    }
    if ret as i32 >= 0 {
        Some(ret)
    } else {
        None
    }
}

#[derive(Debug)]
enum EvalError {
    MissingOperand,
    Overflow(Operator, i32, i32),
    ExtraOperands,
}

fn eval_postfix(postfix: SeqList<Token>) -> Result<i32, EvalError> {
    let mut stack = SeqStack::<i32>::new();
    for token in postfix.into_iter() {
        match token {
            Token::Operator(op) => {
                let operand2 = stack.pop().ok_or(EvalError::MissingOperand)?;
                let operand1 = stack.pop().ok_or(EvalError::MissingOperand)?;
                let result = op
                    .apply(operand1, operand2)
                    .ok_or_else(|| EvalError::Overflow(op.clone(), operand1, operand2))?;
                stack.push(result)
            }
            Token::Num(n) => stack.push(*n as i32),
        }
    }
    let result = stack.pop().ok_or(EvalError::ExtraOperands)?;
    if stack.pop().is_none() {
        Ok(result)
    } else {
        Err(EvalError::ExtraOperands)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let test_cases: Vec<(_, _, Option<i32>)> = vec![
            (
                "1 +1",
                Ok(vec![
                    Token::Num(1),
                    Token::Num(1),
                    Token::Operator(Operator::Add),
                ]),
                Some(2),
            ),
            (
                "a",
                Err(ExprError {
                    what: ExprErrorType::IllegalChar,
                    pos: 0,
                }),
                None,
            ),
            (
                "1+2*3",
                Ok(vec![
                    Token::Num(1),
                    Token::Num(2),
                    Token::Num(3),
                    Token::Operator(Operator::Multiply),
                    Token::Operator(Operator::Add),
                ]),
                Some(7),
            ),
            (
                "(( 1 + 2 ) * 3)",
                Ok(vec![
                    Token::Num(1),
                    Token::Num(2),
                    Token::Operator(Operator::Add),
                    Token::Num(3),
                    Token::Operator(Operator::Multiply),
                ]),
                Some(9),
            ),
            (
                "(1+1",
                Err(ExprError {
                    what: ExprErrorType::UnmatchedParenthesis,
                    pos: 0,
                }),
                None,
            ),
            (
                "1+1)",
                Err(ExprError {
                    what: ExprErrorType::UnmatchedParenthesis,
                    pos: 3,
                }),
                None,
            ),
            (
                "1 + 1 + 1",
                Ok(vec![
                    Token::Num(1),
                    Token::Num(1),
                    Token::Operator(Operator::Add),
                    Token::Num(1),
                    Token::Operator(Operator::Add),
                ]),
                Some(3),
            ),
            (
                "1 - (2 + 3)",
                Ok(vec![
                    Token::Num(1),
                    Token::Num(2),
                    Token::Num(3),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Subtract),
                ]),
                Some(-4),
            ),
            (
                " 1000000000000000000",
                Err(ExprError {
                    what: ExprErrorType::I32Overflow,
                    pos: 1,
                }),
                None,
            ),
        ];
        let mut i = test_cases
            .iter()
            .map(|(expr, expect, result)| {
                let postfix = infix_to_postfix(expr);
                let is_good = match expect {
                    Ok(expect) => {
                        postfix.is_ok() && postfix.as_ref().unwrap().into_iter().eq(expect.iter())
                    }
                    Err(e) => postfix.is_err() && postfix.as_ref().err().unwrap() == e,
                };
                if !is_good {
                    println!("Expression:");
                    println!("{}", expr);
                    println!("Got:");
                    println!("{:?}", postfix);
                    println!("Expect:");
                    println!("{:?}", expect)
                }
                if result.is_some() {
                    assert_eq!(&eval_postfix(postfix.unwrap()).ok(), result);
                }
                is_good
            })
            .filter(|b| !b);
        assert!(i.next().is_none())
    }

    #[test]
    fn test_get_number() {
        let mut i = "123a ".chars().enumerate().peekable();
        while let Some((_, c)) = i.next() {
            match c {
                '0'..='9' => {
                    let number = get_number(c, &mut i).unwrap();
                    assert_eq!(number, 123);
                    assert_eq!(i.next().unwrap().1, 'a')
                }
                _ => {
                    i.next();
                }
            }
        }
    }
}
