use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Operation{
    Subtract,
    Add,
    Multiply,
    Divide,
    ParenthesisOpen,
    ParenthesisClose,
}

pub const OPERATOR_PRECEDENCES: [u8; 6] = [
    1, // Subtract
    1, // Add
    2, // Multiply
    2, // Divide
    0, // Parenthesis Open
    0, // Parenthesis Closed
];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token{
    Op(Operation),
    Number(f64),
}

pub fn string_to_tokens(strings: Vec<&str>) -> Result<Vec<Token>, String>{
    let mut result: Vec<Token> = Vec::new();

    for string in strings {
        match string.parse::<f64>() {
            Ok(val) => result.push(Token::Number(val)),
            Err(_) => {
                match string {
                    "+" => result.push(Token::Op(Operation::Add)),
                    "-" => result.push(Token::Op(Operation::Subtract)),
                    "*" => result.push(Token::Op(Operation::Multiply)),
                    "/" => result.push(Token::Op(Operation::Divide)),
                    "(" => result.push(Token::Op(Operation::ParenthesisOpen)),
                    ")" => result.push(Token::Op(Operation::ParenthesisClose)),
                    _ => return Err(format!("{string} is not a valid number or operation!"))
                }
            }
        }
    }

    Ok(result)
}

pub fn convert_tokens_to_reverse_polish_notation(tokens: Vec<Token>) -> Result<VecDeque<Token>, &'static str>{
    let mut holding_stack: Vec<Operation> = Vec::new();
    let mut output_stack: VecDeque<Token> = VecDeque::new();

    // push tokens to output vector in reverse polish notation
    for token in tokens {
        if matches!(token, Token::Number{..}) { // token is a number
            output_stack.push_back(token.clone());
        } else {
            if token == Token::Op(Operation::ParenthesisOpen) {
                holding_stack.push(match token {
                    Token::Op(op) => op,
                    _ => unreachable!()
                });
                continue;
            }
            if !holding_stack.is_empty() { // check if holding stack has content
                if token == Token::Op(Operation::ParenthesisClose){
                    loop {
                        println!("{:?}", holding_stack.last().unwrap());
                        if holding_stack.is_empty() { return Err("Missing closing parenthesis!"); }
                        if *(holding_stack.last().unwrap()) == Operation::ParenthesisOpen { break; }
                        output_stack.push_back(Token::Op(holding_stack.pop().unwrap()));
                    }
                    holding_stack.pop();
                    continue;
                }
                loop {
                    // pop elements from holding stack onto output stack until precedence of top
                    // holding stack element is less than of the new token
                    let last_holding_stack_op = holding_stack.last();
                    if last_holding_stack_op.is_some(){
                        if OPERATOR_PRECEDENCES[*last_holding_stack_op.unwrap() as usize] >= OPERATOR_PRECEDENCES[match token {Token::Op(op) => op as usize, _ => unreachable!()}]{
                            output_stack.push_back(Token::Op(holding_stack.pop().unwrap()));
                        } else { break; }
                    } else { break; }
                }
            }
            holding_stack.push(match token {Token::Op(op) => op, _ => unreachable!()});
        }
    }
    loop {
        if holding_stack.is_empty() { break; }
        output_stack.push_back(Token::Op(holding_stack.pop().unwrap()));
    }
    
    println!("{:?}", output_stack);

    Ok(output_stack)
}

pub fn collapse_reverse_polish_notation(tokens: VecDeque<Token>) -> Result<f64, &'static str>{
    let mut number_stack: Vec<f64> = Vec::new();
    let mut tokens_mut = tokens.clone();

    while !tokens_mut.is_empty(){
        let current_token: Token = tokens_mut.pop_front().unwrap();
        if matches!(current_token, Token::Number{..}){
            number_stack.push(match current_token {Token::Number(num) => num, _ => unreachable!()});
        } else {
            let op = match current_token{Token::Op(op) => op, _ => unreachable!()};
            if number_stack.len() < 2 { return Err("operator does not have enough numbers!"); }
            let num_a = number_stack.pop().unwrap();
            let num_b = number_stack.pop().unwrap();
            number_stack.push(match op {
                Operation::Subtract => num_b - num_a,
                Operation::Add => num_b + num_a,
                Operation::Multiply => num_b * num_a,
                Operation::Divide => num_b / num_a,
                _ => { return Err("invalid operation!"); }
            })
        }
    }
    if number_stack.len() != 1 { return Err("stray numbers!"); }
    Ok(*number_stack.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_tokens_tests() {
        let test_tokens = vec!["1", "+", "5", "*", "(", "9", "/", "4", ")", "-", "4"];
        let expected_result = vec![
            Token::Number(1.0),
            Token::Op(Operation::Add),
            Token::Number(5.0),
            Token::Op(Operation::Multiply),
            Token::Op(Operation::ParenthesisOpen),
            Token::Number(9.0),
            Token::Op(Operation::Divide),
            Token::Number(4.0),
            Token::Op(Operation::ParenthesisClose),
            Token::Op(Operation::Subtract),
            Token::Number(4.0)
        ];
        let result = string_to_tokens(test_tokens).unwrap_or(vec![]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_simple_addition() {
        let test_tokens = vec!["3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "*", "2", "*", "3"];
        let tokens = string_to_tokens(test_tokens);
        assert_eq!(tokens.is_ok(), true, "string_to_tokens errored out!");

        let reverse_polish_notation = convert_tokens_to_reverse_polish_notation(tokens.unwrap());
        assert_eq!(reverse_polish_notation.is_ok(), true, "convert_tokens_to_reverse_polish_notation errored out!");

        let result = collapse_reverse_polish_notation(reverse_polish_notation.unwrap());
        assert_eq!(result.is_ok(), true, "collapse_reverse_polish_notation errored out!");
        println!("{}", result.unwrap());
        assert_eq!(result.unwrap(), -9.0, "wrong result returned!");
    }
}
