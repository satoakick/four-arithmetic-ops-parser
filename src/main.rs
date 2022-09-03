use std::{env, collections::VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Number(usize),
    Lparen,
    Rparen,
}

#[derive(Debug, PartialEq)]
pub struct Parser {
    pub chars: VecDeque<char>,
    pub look: Option<Token>,
}
impl Parser {
    pub fn new(str: &str) -> Self {
        Self {
            chars: str.chars().collect(),
            look: None,
        }
    }

    fn next_without_whitespace(&mut self) -> Option<char> {
        let mut ret = self.chars.pop_front();
        if ret.is_none() { return None }
        while let Some(ch) = ret {
            if ch.is_whitespace() {
                ret = self.chars.pop_front();
                continue;
            } else {
                break;
            }
        }
        ret
    } 

    fn next(&mut self) -> Option<Token> {
        let mut value = self.next_without_whitespace();

        if value.is_none() { return None }

        match value.unwrap() {
            '+' => Some(Token::Add),
            '-' => Some(Token::Sub),
            '*' => Some(Token::Mul),
            '/' => Some(Token::Div),
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            '0' ..= '9' => {
                let mut nums: Vec<char> = vec![];
                while let Some('0'..='9') = value {
                    nums.push(value.unwrap());
                    if let Some('0'..='9') = self.chars.get(0) {
                        value = self.chars.pop_front();
                    } else {
                        break;
                    }
                }
                Some(
                    Token::Number(
                        nums.into_iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap()
                        )
                    )
            },
            _   => panic!("unexpected token!!")
        }
    }

    pub fn scan(&mut self) -> &Option<Token> {
        self.look = self.next();
        &self.look
    }

}

#[derive(Debug, PartialEq)]
struct Node {
    token: Token,
    nodes: Vec<Node>,
    value: Option<f64>,
}

impl Node {

    pub fn is_first(parser: &Parser) -> bool {
        let token = parser.look.as_ref();
        token.is_none() && !parser.chars.is_empty()
    }

    pub fn is_finish(parser: &Parser) -> bool {
        let token = parser.look.as_ref();
        token.is_none() && parser.chars.is_empty()
    }

    // <expr> ::= <term> [ ('+'|'-') <term> ]*
    pub fn expr(parser: &mut Parser) -> Option<Node> {
        if Self::is_first(parser) { parser.scan(); }
        if Self::is_finish(parser) { return None }

        if let mut left @ Some(_) = Self::term(parser) {
            loop {
                let token = parser.look.as_ref();
                if Self::is_finish(parser) {
                    break;
                } else {
                    let token = token.unwrap();
                    match token {
                        Token::Add | Token::Sub => {
                            let mut node = Self::create_node(token);
                            parser.scan();

                            let right = Self::term(parser);
                            if left.is_some() && right.is_some() {
                                Self::add_children(&mut node, left.unwrap(), right.unwrap());
                            }
                            left = Some(node);
                        },
                        _ => break,
                    }
                }
            }
            left
        } else {
            None
        }
    }

    // <term> ::= <factor> [ ('*'|'/') <factor> ]*
    pub fn term(parser: &mut Parser) -> Option<Node> {
        if let mut left @ Some(_) = Self::factor(parser) {
            loop {
                let token = parser.look.as_ref();
                if token.is_none() { break; }
                let token = token.unwrap();
                match token {
                    Token::Mul | Token::Div => {
                        let mut node = Self::create_node(&token);
                        parser.scan();
                        let right = Self::factor(parser);
                        if left.is_some() && right.is_some() {
                            Self::add_children(&mut node, left.unwrap(), right.unwrap());
                        }
                        left = Some(node);
                    },
                    _ => break,
                }
            }
            left
        } else {
            None
        }
    }

    // <factor> ::= <number> | '(' <expr> ')'
    pub fn factor (parser: &mut Parser) -> Option<Node> {
        if let Some(Token::Lparen) = parser.look {
            parser.scan();
            let node = Self::expr(parser);
            parser.scan();
            if let Some(Token::Rparen) = parser.look {
                panic!("should be rparen");
            } else {
                node
            }
        } else {
            // <number> ::= [ 1 | 2 | ... | 9 | 0 ]*
            if let Some(Token::Number(_)) = parser.look {
                let node = Self::create_node(&parser.look.as_ref().unwrap());
                parser.scan();
                Some(node)
            } else {
                panic!("should be number");
            }
        }
    }

    pub fn create_node(token: &Token) -> Node {
        if let Token::Number(nums) = token {
            let value: String = nums.to_string();
            let value2 = value.parse::<f64>();
            Node { token: token.clone(), nodes: vec![], value: Some(value2.unwrap()) }
        } else {
            Node { token: token.clone(), nodes: vec![], value: None }
        }
    }

    pub fn add_children(parent: &mut Node, child_1: Node, child_2: Node) {
        parent.nodes.push(child_1);
        parent.nodes.push(child_2);
    }

    pub fn print_ast(node: &Node) {
        Self::print_ast_inner(node);
        println!("");
    }

    fn print_ast_inner(node: &Node) {
        print!("({:?}", node.token);
        for child in &node.nodes {
            if let Token::Number(_) = child.token.clone() {
                print!("  {}", child.value.unwrap());
            } else {
                Self::print_ast_inner(child);
            }
        }
        print!(")");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_init_test() {
        assert_eq!(
            Parser {
                chars: VecDeque::from(['1', '+', '2']),
                look: None
            },
            Parser::new("1+2")
        );
    }

    #[test]
    fn parser_init_test_2() {
        assert_eq!(
            Parser {
                chars: VecDeque::from(['1', '2', '+', '3']),
                look: None
            },
            Parser::new("12+3")
        );
    }


    #[test]
    fn scan_test() {
        let parser = &mut Parser::new("1+2");
        let token_0 = parser.look.clone();
        parser.scan();
        let token_1 = parser.look.clone();
        parser.scan();
        let token_2 = parser.look.clone();
        parser.scan();
        let token_3 = parser.look.clone();
        parser.scan();
        let token_4 = parser.look.clone();

        assert_eq!(token_0, None);
        assert_eq!(token_1.unwrap(), Token::Number(1));
        assert_eq!(token_2.unwrap(), Token::Add);
        assert_eq!(token_3.unwrap(), Token::Number(2));
        assert_eq!(token_4, None);
    }

    #[test]
    fn tree_node_one_add_test() {
        let expression = "1+2";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();
        println!("{} => {:?}", expression, node);

        assert_eq!(
            Node { 
                token: Token::Add, 
                nodes: vec![
                    Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                    Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn tree_node_one_multiple_test() {
        let expression = "1*2";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();
        println!("{} => {:?}", expression, node);

        assert_eq!(
            Node { 
                token: Token::Mul, 
                nodes: vec![
                    Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                    Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn tree_node_one_divisor_test() {
        let expression = "1/2";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();
        println!("{} => {:?}", expression, node);

        assert_eq!(
            Node { 
                token: Token::Div, 
                nodes: vec![
                    Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                    Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn tree_node_add_and_multiple_test() {
        let expression = "(1+2)*3";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();
        println!("{} => {:?}", expression, node);

        assert_eq!(
            Node { 
                token: Token::Mul, 
                nodes: vec![
                    Node { 
                        token: Token::Add, 
                        nodes: vec![
                            Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                            Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                        ],
                        value: None,
                    },
                    Node { token: Token::Number(3), nodes: vec![], value: Some(3 as f64) }
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn tree_node_add_deep_test() {
        let expression = "1+2+3+4";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();

        Node::print_ast(&node);

        assert_eq!(
            Node { 
                token: Token::Add, 
                nodes: vec![
                    Node { 
                        token: Token::Add, 
                        nodes: vec![
                            Node {
                                token: Token::Add, 
                                nodes: vec![
                                    Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                                    Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                                ],
                                value: None,
                            },
                            Node { token: Token::Number(3), nodes: vec![], value: Some(3 as f64) }
                        ],
                        value: None,
                    },
                    Node { token: Token::Number(4), nodes: vec![], value: Some(4 as f64) }
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn tree_node_add_and_parens_test() {
        let expression = "(1+2)+(3+4)";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();

        Node::print_ast(&node);
        println!("");

        assert_eq!(
            Node { 
                token: Token::Add, 
                nodes: vec![
                    Node {
                        token: Token::Add, 
                        nodes: vec![
                            Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                            Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                        ],
                        value: None,
                    },
                    Node {
                        token: Token::Add, 
                        nodes: vec![
                            Node { token: Token::Number(3), nodes: vec![], value: Some(3 as f64) },
                            Node { token: Token::Number(4), nodes: vec![], value: Some(4 as f64) }
                        ],
                        value: None,
                    },
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn next_without_whitespace_test_1() {
        let mut parser = Parser::new("   1");

        let node = Node::expr(&mut parser).unwrap();
        Node::print_ast(&node);
        
        assert_eq!(Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) }, node);
    }
    #[test]
    fn next_without_whitespace_test_2() {
        let mut parser = Parser::new("  2  ");

        let node = Node::expr(&mut parser).unwrap();
        Node::print_ast(&node);
        
        assert_eq!(Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }, node);
    }
    #[test]
    fn next_without_whitespace_test_3() {
        let mut parser = Parser::new("    ");

        let node = Node::expr(&mut parser);
        assert!(node.is_none());
    }

    #[test]
    fn skip_whitespace_test() {
        let expression = " 1 + 2  ";
        let value = expression;
        let mut parser = Parser::new(value);

        let node = Node::expr(&mut parser).unwrap();

        Node::print_ast(&node);

        assert_eq!(
            Node { 
                token: Token::Add, 
                nodes: vec![
                    Node { token: Token::Number(1), nodes: vec![], value: Some(1 as f64) },
                    Node { token: Token::Number(2), nodes: vec![], value: Some(2 as f64) }
                ],
                value: None,
            },
            node
        );
    }

    #[test]
    fn nums_test() {
        let mut parser = Parser::new("11");

        let node = Node::expr(&mut parser).unwrap();

        Node::print_ast(&node);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let expr = args[1..].into_iter().map(|s| s.as_str()).collect::<String>();

    let mut parser = Parser::new(&expr);
    if let Some(node) = Node::expr(&mut parser) {
        Node::print_ast(&node);
    }
}
