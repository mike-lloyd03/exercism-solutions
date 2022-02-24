#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut reg: Vec<i32> = Vec::new();
    for input in inputs {
        match *input {
            CalculatorInput::Value(v) => reg.push(v),
            _ => reg = vec![math(&reg, input)?],
        }
    }
    reg.pop()
}

fn math(reg: &[i32], op: &CalculatorInput) -> Option<i32> {
    if reg.len() == 2 {
        match *op {
            CalculatorInput::Add => Some(reg[0] + reg[1]),
            CalculatorInput::Subtract => Some(reg[0] - reg[1]),
            CalculatorInput::Multiply => Some(reg[0] * reg[1]),
            CalculatorInput::Divide => Some(reg[0] / reg[1]),
            CalculatorInput::Value(_) => None,
        }
    } else {
        None
    }
}
