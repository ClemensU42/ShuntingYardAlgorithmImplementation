#[derive(Debug, PartialEq)]
enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide,
    ParenthesisOpen,
    ParenthesisClose
}
#[derive(Debug, PartialEq)]
enum Token{
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

pub fn calculate(tokens: &Vec<Token>) -> Result<f64, &'static str>{
    Ok(1.0)
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
}
