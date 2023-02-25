use std::collections::VecDeque;
use std::io::{self, Write};


#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operation(char),
    LeftParen,
    RightParen,
}

fn main() {

    // start an infinite loop to keep the program running
    loop {
        // prompt the user to enter an expression
        print!("Enter an expression: ");

        // flush the output buffer to ensure the prompt is displayed immediately
        io::stdout().flush().unwrap();

        // read the user's input from the command line
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // tokenize the user's input
        let tokens = match tokenize(&input) {
            Ok(tokens) => tokens, // if tokenization succeeds, store the tokens in the 'tokens' variable
            Err(err) => { // if tokenization fails, print an error message and start the loop again
                println!("Error: {}\n", err);
                continue;
            },
        };

        // convert the tokens into postfix notation
        let postfix = match shunting_yard(tokens) {
            Ok(postfix) => postfix, // if the conversion succeeds, store the postfix notation in the 'postfix' variable
            Err(err) => { // if the conversion fails, print an error message and start the loop again
                println!("Error: {}\n", err);
                continue;
            },
        };

        // evaluate the postfix notation and compute the result
        let result = match evaluate(postfix) {
            Ok(result) => result, // if the evaluation succeeds, store the result in the 'result' variable
            Err(err) => { // if the evaluation fails, print an error message and start the loop again
                println!("Error: {}\n", err);
                continue;
            },
        };

        // print the result to the command line
        println!("Result: {}\n", result);
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String>  {
    // Function to tokenize a mathematical expression and convert it into a vector of tokens
    // Takes a string `input` as an argument and returns a `Result` with either a vector of tokens or a string error message

    // Create an empty vector to store the tokens
    let mut tokens = Vec::new();
    // Create an empty string to store the current token
    let mut current_token = String::new();

    // Loop through each character in the input string
    for c in input.chars() {
        // If the character is a digit or a decimal point, add it to the current token string
        if c.is_digit(10) || c == '.' {
            current_token.push(c);
        }
        // If the character is a parenthesis or an operator
        else if c == '(' || c == ')' || c == '+' || c == '-' || c == '*' || c == '/' {
            // If the current token string is not empty, convert it to a number and add it to the vector of tokens
            if !current_token.is_empty() {
                // Use the `parse()` method to convert the current token string to a number
                let num = current_token.parse().map_err(|_| "Invalid number entered.")?;
                // Add the number token to the vector of tokens
                tokens.push(Token::Number(num));
                // Clear the current token string
                current_token.clear();
            }
            // Add the parenthesis or operator token to the vector of tokens
            match c {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '+' | '-' | '*' | '/' => tokens.push(Token::Operation(c)),
                _ => {}
            }
        }
        // If the character is not a digit, decimal point, parenthesis, or operator, skip it
        else {
            continue;
        }
    }

    // If the current token string is not empty, convert it to a number and add it to the vector of tokens
    if !current_token.is_empty() {
        let num = current_token.parse().map_err(|_| "Invalid number entered.")?;
        tokens.push(Token::Number(num));
    }

    // If no tokens were found, return an error message
    if tokens.is_empty() {
        return Err("No tokens found.".into());
    }

    // Return the vector of tokens as a `Result`
    Ok(tokens)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<VecDeque<Token>, String> {
    let mut output_queue = VecDeque::new();
    let mut operator_stack = Vec::new();

    for token in tokens {
        match token {
            // add numbers to the output queue
            Token::Number(_) => output_queue.push_back(token), 
            // push left parentheses to the operator stack
            Token::LeftParen => operator_stack.push(token), 
            // when a right parenthesis is found, pop operators off the stack and add them to the output queue until a left parenthesis is found
            Token::RightParen => { 
                while let Some(top) = operator_stack.last() {
                    if let Token::LeftParen = top {
                        operator_stack.pop();
                        break;
                    } else {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                }
            }
            // when an operator is found, compare it to the top operator on the stack, and pop operators off the stack and add them to the output queue until an operator with lower precedence is found, or a left parenthesis is found
            Token::Operation(op) => { 
                while let Some(top) = operator_stack.last() {
                    if let Token::Operation(top_op) = top {
                        if (precedence(op) <= precedence(*top_op)) && (*top_op != '(') {
                            output_queue.push_back(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                // push the operator onto the stack
                operator_stack.push(token); 
            }
        }
    }
    // pop any remaining operators off the stack and add them to the output queue
    while let Some(token) = operator_stack.pop() { 
        // if a left parenthesis is found, there are mismatched parentheses
        if let Token::LeftParen = token { 
            return Err("Mismatched parentheses".to_string());
        }
        output_queue.push_back(token);
    }

    Ok(output_queue)
}


fn evaluate(output_queue: VecDeque<Token>) -> Result<f64, String> {
    let mut stack = Vec::new();

    // Loop through each token in the output queue
    for token in output_queue {
        match token {
            // If the token is a number, push it onto the stack
            Token::Number(num) => stack.push(num),
            // If the token is an operator, pop two numbers from the stack, apply the operator to them,
            // and push the result onto the stack
            Token::Operation(op) => {
                let b = stack.pop().expect("Invalid expression.");
                let a = stack.pop().expect("Invalid expression.");

                // Apply the operator to the two numbers
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => return Err("Invalid operation entered.".into()),
                };

                // Push the result onto the stack
                stack.push(result);
            },
            // If the token is neither a number nor an operator, return an error
            _ => return Err("Invalid token in output queue".into()),
        }
    }

    // The final result should be the only element remaining on the stack
    // If the stack is empty, return an error
    stack.pop().ok_or("Invalid expression.".into())
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}
