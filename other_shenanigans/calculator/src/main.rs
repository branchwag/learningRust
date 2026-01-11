use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default()))),
    )
}

#[derive(Default)]
struct CalculatorApp {
    display: String,
    current_value: f64,
    previous_value: f64,
    operation: Option<Operation>,
    new_number: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl CalculatorApp {
    fn handle_number(&mut self, digit: &str) {
        if self.new_number {
            self.display = digit.to_string();
            self.new_number = false;
        } else {
            if self.display == "0" {
                self.display = digit.to_string();
            } else {
                self.display.push_str(digit);
            }
        }
    }

    fn handle_decimal(&mut self) {
        if self.new_number {
            self.display = "0.".to_string();
            self.new_number = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn handle_operation(&mut self, op: Operation) {
        if let Ok(value) = self.display.parse::<f64>() {
            if let Some(prev_op) = self.operation {
                self.previous_value = self.calculate(self.previous_value, value, prev_op);
                self.display = format_number(self.previous_value);
            } else {
                self.previous_value = value;
            }
            self.operation = Some(op);
            self.new_number = true;
        }
    }

    fn handle_equals(&mut self) {
        if let (Some(op), Ok(value)) = (self.operation, self.display.parse::<f64>()) {
            let result = self.calculate(self.previous_value, value, op);
            self.display = format_number(result);
            self.current_value = result;
            self.operation = None;
            self.new_number = true;
        }
    }

    fn calculate(&self, a: f64, b: f64, op: Operation) -> f64 {
        match op {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => {
                if b != 0.0 {
                    a / b
                } else {
                    f64::NAN
                }
            }
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.previous_value = 0.0;
        self.operation = None;
        self.new_number = false;
    }

    fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.new_number = false;
    }

    fn backspace(&mut self) {
        if !self.new_number && self.display.len() > 1 {
            self.display.pop();
        } else {
            self.display = "0".to_string();
        }
    }

    fn negate(&mut self) {
        if let Ok(mut value) = self.display.parse::<f64>() {
            value = -value;
            self.display = format_number(value);
        }
    }
}

fn format_number(n: f64) -> String {
    if n.is_nan() {
        "Error".to_string()
    } else if n.is_infinite() {
        "Error".to_string()
    } else if n.fract() == 0.0 && n.abs() < 1e10 {
        format!("{}", n as i64)
    } else {
        // Remove trailing zeros
        let mut s = format!("{:.10}", n);
        while s.contains('.') && (s.ends_with('0') || s.ends_with('.')) {
            if s.ends_with('.') {
                s.pop();
                break;
            }
            s.pop();
        }
        s
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);

                // Display
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.display)
                            .font(egui::TextStyle::Heading)
                            .desired_width(265.0)
                            .interactive(false)
                            .frame(true),
                    );
                });

                ui.add_space(20.0);

                // Button grid
                ui.vertical(|ui| {
                    // Row 1: Clear buttons and operations
                    ui.horizontal(|ui| {
                        if ui.add_sized([65.0, 50.0], egui::Button::new("C")).clicked() {
                            self.clear();
                        }
                        if ui
                            .add_sized([65.0, 50.0], egui::Button::new("CE"))
                            .clicked()
                        {
                            self.clear_entry();
                        }
                        if ui
                            .add_sized([65.0, 50.0], egui::Button::new("Backspace"))
                            .clicked()
                        {
                            self.backspace();
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("÷")).clicked() {
                            self.handle_operation(Operation::Divide);
                        }
                    });

                    ui.add_space(5.0);

                    // Row 2: 7, 8, 9, ×
                    ui.horizontal(|ui| {
                        if ui.add_sized([65.0, 50.0], egui::Button::new("7")).clicked() {
                            self.handle_number("7");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("8")).clicked() {
                            self.handle_number("8");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("9")).clicked() {
                            self.handle_number("9");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("×")).clicked() {
                            self.handle_operation(Operation::Multiply);
                        }
                    });

                    ui.add_space(5.0);

                    // Row 3: 4, 5, 6, -
                    ui.horizontal(|ui| {
                        if ui.add_sized([65.0, 50.0], egui::Button::new("4")).clicked() {
                            self.handle_number("4");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("5")).clicked() {
                            self.handle_number("5");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("6")).clicked() {
                            self.handle_number("6");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("-")).clicked() {
                            self.handle_operation(Operation::Subtract);
                        }
                    });

                    ui.add_space(5.0);

                    // Row 4: 1, 2, 3, +
                    ui.horizontal(|ui| {
                        if ui.add_sized([65.0, 50.0], egui::Button::new("1")).clicked() {
                            self.handle_number("1");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("2")).clicked() {
                            self.handle_number("2");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("3")).clicked() {
                            self.handle_number("3");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("+")).clicked() {
                            self.handle_operation(Operation::Add);
                        }
                    });

                    ui.add_space(5.0);

                    // Row 5: ±, 0, ., =
                    ui.horizontal(|ui| {
                        if ui.add_sized([65.0, 50.0], egui::Button::new("±")).clicked() {
                            self.negate();
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("0")).clicked() {
                            self.handle_number("0");
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new(".")).clicked() {
                            self.handle_decimal();
                        }
                        if ui.add_sized([65.0, 50.0], egui::Button::new("=")).clicked() {
                            self.handle_equals();
                        }
                    });
                });
            });
        });
    }
}
