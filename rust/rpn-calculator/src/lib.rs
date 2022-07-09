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
            CalculatorInput::Add => {
                let a1 = reg.pop()?;
                let a2 = reg.pop()?;
                reg.push(a2 + a1);
            }
            CalculatorInput::Subtract => {
                let s1 = reg.pop()?;
                let s2 = reg.pop()?;
                reg.push(s2 - s1);
            }
            CalculatorInput::Multiply => {
                let m1 = reg.pop()?;
                let m2 = reg.pop()?;
                reg.push(m2 * m1);
            }
            CalculatorInput::Divide => {
                let d1 = reg.pop()?;
                let d2 = reg.pop()?;
                reg.push(d2 / d1);
            }
        }
    }
    if reg.len() > 1 {
        return None;
    }
    reg.pop()
}
