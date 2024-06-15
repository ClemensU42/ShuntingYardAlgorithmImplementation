
#[derive(Debug)]
enum Operation{
    Add,
    Subtract,
    Multiply,
    Divide
}
#[derive(Debug)]
enum Token{
    Op(Operation),
    Number(f64),
}

pub fn string_to_tokens(strings: Vec<String>) -> Result<Vec<Token>, &'static str>{
    Ok(vec![])
}

pub fn calculate(tokens: &Vec<Token>) -> Result<f64, &'static str>{
    Ok(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_tokens_tests() {

        //assert_eq!(result, 4);
    }
}
