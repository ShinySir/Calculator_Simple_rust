// calculator_logic.rs

pub struct CalculatorApp {
    pub first_operand: Option<f64>,
    pub second_operand: Option<f64>,
    pub operator: Option<char>,
    pub current_input: String,
}

impl CalculatorApp {

    pub fn append_digit(&mut self, c: char) {
        if c == '.' && self.current_input.contains('.') {
            return;
        }
        self.current_input.push(c);
    }

    pub fn set_operator(&mut self, op: char) {
        if let Ok(num) = self.current_input.parse::<f64>() {
            self.first_operand = Some(num);
            self.operator = Some(op);
            self.current_input.clear();
        }
    }


    pub fn calculate(&mut self) {
        if let (Some(a), Some(op)) = (self.first_operand, self.operator) {
            if let Ok(b) = self.current_input.parse::<f64>() {

                self.second_operand = Some(b); // store second number

                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' | 'x' => a * b,
                    '/' | '÷' => a / b,
                    _ => b,
                };

                self.current_input = result.to_string();
                self.first_operand = None;
                self.operator = None;
            }
        }
    }

    pub fn clear(&mut self) {
        self.current_input.clear();
        self.first_operand = None;
        self.second_operand = None;
        self.operator = None;
    }

    pub fn backspace(&mut self) {
        self.current_input.pop();
    }
}