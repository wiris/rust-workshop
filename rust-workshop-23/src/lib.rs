//! # Welcome to the Rust workshop 2023
//!
//! Here you have the source code for this workshop.
//! Complete those blocks containing the `TODO` comment.

/// Represents the available operators and operands an expression may have
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(usize),
    Add,
    Sub,
    Prod,
    Div,
}

impl Token {
    /// Returns true if, and only if, the Token itself is an operator (aka.: Add, Sub, Mul, Div)
    fn is_operator(&self) -> bool {
        matches!(self, Token::Num(_))
    }

    /// Returns true if, and only if, the Token is a product or a division instead
    fn is_prod_or_div(&self) -> bool {
        matches!(self, Token::Prod) || matches!(self, Token::Div)
    }
}

/// Represents a node of a binary tree
#[derive(Debug, PartialEq)]
pub struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(token: Token) -> Self {
        Node {
            token,
            left: Default::default(),
            right: Default::default(),
        }
    }
}

/// Given a string containing a mathematical expresion, returns a vector with all those tokens contained
/// in the expression.
///
/// ## Preconditions
/// - Potential tokens will be separated by whitespaces.
fn tokenize(expression: &str) -> Vec<Token> {
    // TODO: implement here the tokenizer algorithm
    todo!() // remove this line before starting
}

/// Given a string containing a mathematical expresion, returns the root node of a binary tree
/// representing that expresion.
///
/// ## Preconditions
/// - Potential tokens will be separated by whitespaces.
/// - Two or more operators will never be consecutive.
/// - Two or more operands (aka.: [`Token::Num`]) will never be consecutive.
///
/// ## Restrictions
/// - The deserialization must be [__left associative__](https://en.wikipedia.org/wiki/Operator_associativity).
/// That means that in case of two or more operations with the same priority, them must be bracket up starting
/// by the left. Here is an example: the expression `1 + 2 + 3` will be deserialized as it was `((1 + 2) + 3)`.
pub fn deserialize(expression: &str) -> Node {
    // TODO: implement here the deserializer algorithm
    todo!() // remove this line before starting
}

#[cfg(test)]
mod tests {
    use super::{deserialize, tokenize, Node, Token};

    #[test]
    fn tokenize_simple_expression() {
        let tokens = tokenize("1 + 2");
        assert_eq!(tokens, vec![Token::Num(1), Token::Add, Token::Num(2)]);
    }

    #[test]
    fn tokenize_long_number() {
        let tokens = tokenize("123");
        assert_eq!(tokens, vec![Token::Num(123),]);
    }

    #[test]
    fn tokenize_should_panic_on_unknown_tokens() {
        let result = std::panic::catch_unwind(|| tokenize("("));
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_simple_expression() {
        let got = deserialize("1 + 2");
        let want = Node {
            token: Token::Add,
            left: Some(Node::new(Token::Num(1)).into()),
            right: Some(Node::new(Token::Num(2)).into()),
        };

        assert_eq!(got, want);
    }

    #[test]
    fn deserialize_expresion_of_three_sums() {
        let got = deserialize("1 + 2 + 3");
        let want = Node {
            token: Token::Add,
            left: Some(
                Node {
                    token: Token::Add,
                    left: Some(Node::new(Token::Num(1)).into()),
                    right: Some(Node::new(Token::Num(2)).into()),
                }
                .into(),
            ),
            right: Some(Node::new(Token::Num(3)).into()),
        };

        assert_eq!(got, want);
    }

    #[test]
    fn deserialize_expresion_with_hierarchy() {
        let got = deserialize("1 + 2 * 3");
        let want = Node {
            token: Token::Add,
            left: Some(Node::new(Token::Num(1)).into()),
            right: Some(
                Node {
                    token: Token::Prod,
                    left: Some(Node::new(Token::Num(2)).into()),
                    right: Some(Node::new(Token::Num(3)).into()),
                }
                .into(),
            ),
        };

        assert_eq!(got, want);
    }

    #[test]
    fn deserialize_complex_expresion_with_all_operators() {
        let got = deserialize("1 + 2 * 3 / 4 - 5");
        let want = Node {
            token: Token::Sub,
            left: Some(
                Node {
                    token: Token::Add,
                    left: Some(
                        Node {
                            token: Token::Num(1),
                            left: None,
                            right: None,
                        }
                        .into(),
                    ),
                    right: Some(
                        Node {
                            token: Token::Div,
                            left: Some(
                                Node {
                                    token: Token::Prod,
                                    left: Some(
                                        Node {
                                            token: Token::Num(2),
                                            left: None,
                                            right: None,
                                        }
                                        .into(),
                                    ),
                                    right: Some(
                                        Node {
                                            token: Token::Num(3),
                                            left: None,
                                            right: None,
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                            right: Some(
                                Node {
                                    token: Token::Num(4),
                                    left: None,
                                    right: None,
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            right: Some(
                Node {
                    token: Token::Num(5),
                    left: None,
                    right: None,
                }
                .into(),
            ),
        };

        assert_eq!(got, want);
    }
}
