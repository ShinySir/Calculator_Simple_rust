mod operations;
use operations::CalculatorApp;
use eframe::*;
use egui::{CentralPanel, IconData, Vec2, Button};
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

//function to change the font to lexend
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Your main Lexend font
    fonts.font_data.insert(
        "lexend".to_owned(),
        Arc::new(FontData::from_static(include_bytes!("../assets/LexendDeca-Regular.ttf"))),
    );

    // Fallback font with ⌫
    fonts.font_data.insert(
        "noto_symbols".to_owned(),
        Arc::new(FontData::from_static(include_bytes!("../assets/NotoSansSymbols2-Regular.ttf"))),
    );

    // Proportional font: Lexend first, then fallback
    fonts.families.insert(
        FontFamily::Proportional,
        vec!["lexend".to_owned(), "noto_symbols".to_owned()],
    );

    // Monospace font: same
    fonts.families.insert(
        FontFamily::Monospace,
        vec!["lexend".to_owned(), "noto_symbols".to_owned()],
    );

    ctx.set_fonts(fonts);
}


//function of where the ui sits
impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);

        CentralPanel::default().show(ctx, |ui| {
            let button_size = Vec2::new(100.0, 75.0); // fixed size for all buttons
            ui.label(":3");

            ui.columns(2, |columns| {
                // Left column: buttons
                columns[0].vertical(|ui| {
                    // First row
                    ui.horizontal(|ui| {
                        if ui.add_sized(button_size, Button::new("Clear")).clicked() {
                            println!("Clear");
                            self.clear();
                        }
                        if ui.add_sized(button_size, Button::new("⌫")).clicked() {
                            println!("Backspace");
                            self.backspace();
                        }
                        ui.label(&self.current_input);
                    });

                    // Second row
                    ui.horizontal(|ui| {
                        if ui.add_sized(button_size, Button::new("7")).clicked() {
                            println!("7");
                            self.append_digit('7');
                        }
                        if ui.add_sized(button_size, Button::new("8")).clicked() {
                            println!("8");
                            self.append_digit('8');
                        }
                        if ui.add_sized(button_size, Button::new("9")).clicked() {
                            println!("9");
                            self.append_digit('9');
                        }
                        if ui.add_sized(button_size, Button::new("x")).clicked() {
                            println!("x");
                            self.set_operator('x');
                        }
                    });

                    // Third row
                    ui.horizontal(|ui| {
                        if ui.add_sized(button_size, Button::new("4")).clicked() {
                            println!("4");
                            self.append_digit('4');
                        }
                        if ui.add_sized(button_size, Button::new("5")).clicked() {
                            println!("5");
                            self.append_digit('5');
                        }
                        if ui.add_sized(button_size, Button::new("6")).clicked() {
                            println!("6");
                            self.append_digit('6');
                        }
                        if ui.add_sized(button_size, Button::new("-")).clicked() {
                            println!("-");
                            self.set_operator('-');
                        }
                    });

                    // Fourth row
                    ui.horizontal(|ui| {
                        if ui.add_sized(button_size, Button::new("1")).clicked() {
                            println!("1");
                            self.append_digit('1');
                        }
                        if ui.add_sized(button_size, Button::new("2")).clicked() {
                            println!("2");
                            self.append_digit('2');
                        }
                        if ui.add_sized(button_size, Button::new("3")).clicked() {
                            println!("3");
                            self.append_digit('3');
                        }
                        if ui.add_sized(button_size, Button::new("+")).clicked() {
                            println!("+");
                            self.set_operator('+');
                        }
                    });

                    // Fifth row
                    ui.horizontal(|ui| {
                        if ui.add_sized(button_size, Button::new("÷")).clicked() {
                            println!("÷");
                            self.set_operator('÷');
                        }
                        if ui.add_sized(button_size, Button::new("0")).clicked() {
                            println!("0");
                            self.append_digit('0');
                        }
                        if ui.add_sized(button_size, Button::new(".")).clicked() {
                            println!(".");
                            self.append_digit('.');
                        }
                        if ui.add_sized(button_size, Button::new("=")).clicked() {
                            println!("=");
                            self.calculate();
                        }
                    });

                    
                });

                // Right column: display
                columns[1].vertical(|ui| {
                    ui.label("");ui.label("");
                });
            });
        });
    }
}

//this is the main function
fn main() -> eframe::Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        
        viewport: egui::ViewportBuilder::default()
            .with_icon(load_icon())
            .with_app_id("Simple_Calculator"),
        ..Default::default()
    };


    run_native(
        "Simple Calculator",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(CalculatorApp {
                first_operand: None,
                second_operand: None,
                operator: None,
                current_input: String::new(),
            }))
        }),
    )
}

fn load_icon() -> IconData {
    // Example: Load and convert a PNG file to IconData
    let png_bytes = include_bytes!("../assets/icon.png").as_slice();
    eframe::icon_data::from_png_bytes(png_bytes)
        .expect("Failed to convert to IconData")
}
