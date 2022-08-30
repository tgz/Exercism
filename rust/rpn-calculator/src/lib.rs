#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                }
                let v1 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                stack.push(v1 + v2)
            },
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                }
                let v1 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                stack.push(v1 * v2)
            },
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }
                let v1 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                stack.push(v2/v1)
            },
            CalculatorInput::Subtract => {
                if stack.len() < 2 {
                    return None;
                }
                let v1 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                stack.push(v2 - v1)
            },
            CalculatorInput::Value(v) => {
                stack.push(*v)
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    stack.pop()
}
