// Importing the VecDeque data structure from the standard library

use std::collections::VecDeque;

// Define an enum to represent different arithmetic operators
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Function to apply an operator to two operands and return the result
fn apply_operator(op: Operator, a: i64, b: i64) -> i64 {
    match op {
        Operator::Add => a + b,
        Operator::Subtract => b - a, // Fixed: Subtract a from b
        Operator::Multiply => b * a,
        Operator::Divide => b / a,
    }
}

// Function to evaluate an expression string and return the result
fn evaluate_expression(expression: &str) -> i64 {
    // Define two queues: one for operands and one for operators
    let mut operands = VecDeque::new();
    let mut operators = VecDeque::new();
    // Buffer to store digits of multi-digit numbers
    let mut num_buffer = String::new();

    // Iterate over each character in the expression
    for token in expression.chars() {
        match token {
            // If the token is a digit, add it to the number buffer
            '0'..='9' => num_buffer.push(token),
            // If the token is an arithmetic operator ('a', 'b', 'c', 'd')
            'a' | 'b' | 'c' | 'd' => {
                // If there are digits in the buffer, parse them and add to operands queue
                if !num_buffer.is_empty() {
                    operands.push_back(num_buffer.parse::<i64>().unwrap());
                    num_buffer.clear(); // Clear the buffer
                }
                // Map the operator token to the corresponding Operator enum variant and push to operators queue
                operators.push_back(match token {
                    'a' => Operator::Add,
                    'b' => Operator::Subtract,
                    'c' => Operator::Multiply,
                    'd' => Operator::Divide,
                    _ => unreachable!(), // This should never happen
                });
            }
            // If the token is 'e', add an additional '+' operator to the operators queue
            'e' => {
                operators.push_back(Operator::Add);
            }
            // If the token is 'f', apply operators until reaching an '+' operator
            'f' => {
                while let Some(op) = operators.pop_back() {
                    if op == Operator::Add {
                        break; // Exit the loop if we encounter an addition operator
                    }
                    // Pop the last two operands, apply the operator, and push the result back to operands queue
                    if let Some(b) = operands.pop_back() {
                        if let Some(a) = operands.pop_back() {
                            let result = apply_operator(op, a, b);
                            operands.push_back(result);
                        }
                    }
                }
            }
            _ => unreachable!(), // This should never happen
        }
    }

    // If there are digits left in the buffer, parse them and add to operands queue
    if !num_buffer.is_empty() {
        operands.push_back(num_buffer.parse::<i64>().unwrap());
    }

    // Apply remaining operators to the operands queue until operators queue is empty
    while let Some(op) = operators.pop_front() {
        if let Some(b) = operands.pop_front() {
            if let Some(a) = operands.pop_front() {
                let result = apply_operator(op, a, b);
                operands.push_front(result);
            }
        }
    }

    // Return the final result from the operands queue, or 0 if the queue is empty
    operands.pop_front().unwrap_or(0)
}

// Unit tests for the evaluate_expression function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression1() {
        assert_eq!(evaluate_expression("3a2c4"), 20);
    }

    #[test]
    fn test_evaluate_expression2() {
        assert_eq!(evaluate_expression("32a2d2"), 17);
    }

    #[test]
    fn test_evaluate_expression3() {
        assert_eq!(evaluate_expression("500a10b66c32"), 14208);
    }

    #[test]
    fn test_evaluate_expression4() {
        println!("{:?}", evaluate_expression("3ae4c66fb32")); 
        assert_eq!(evaluate_expression("3ae4c66fb32"), 235);

    }

    #[test]
    fn test_evaluate_expression5() {
        println!("{:?}", evaluate_expression("3c4d2aee2a4c41fc4f")); 
        assert_eq!(evaluate_expression("3c4d2aee2a4c41fc4f"), 990);
    }
}
