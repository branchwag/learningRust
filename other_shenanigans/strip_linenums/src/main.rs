use eframe::egui;
use arboard::Clipboard;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Strip Line Numbers",
        options,
        Box::new(|_cc| Ok(Box::<StripLinesApp>::default())),
    )
}

struct StripLinesApp {
    input_text: String,
    output_text: String,
}

impl Default for StripLinesApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
        }
    }
}

impl eframe::App for StripLinesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.vertical(|ui| {
                    ui.heading("Strip Line Numbers");
                    ui.add_space(10.0);
                    
                    // Input section
                    ui.label("Input (paste code with line numbers):");
                    egui::ScrollArea::vertical()
                        .id_salt("input_scroll")
                        .min_scrolled_height(200.0)
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.add_sized(
                                [ui.available_width(), 200.0],
                                egui::TextEdit::multiline(&mut self.input_text)
                                    .font(egui::TextStyle::Monospace)
                            ).context_menu(|ui| {
                                if ui.button("Cut").clicked() {
                                    if let Ok(mut clipboard) = Clipboard::new() {
                                        let _ = clipboard.set_text(&self.input_text);
                                    }
                                    self.input_text.clear();
                                    self.output_text.clear();
                                    ui.close_menu();
                                }
                                if ui.button("Copy").clicked() {
                                    if let Ok(mut clipboard) = Clipboard::new() {
                                        let _ = clipboard.set_text(&self.input_text);
                                    }
                                    ui.close_menu();
                                }
                                if ui.button("Paste").clicked() {
                                    if let Ok(mut clipboard) = Clipboard::new() {
                                        if let Ok(text) = clipboard.get_text() {
                                            self.input_text = text;
                                        }
                                    }
                                    ui.close_menu();
                                }
                            });
                        });
                    
                    // Process the input
                    self.output_text = strip_line_numbers(&self.input_text);
                    
                    ui.add_space(10.0);
                    
                    // Output section
                    ui.horizontal(|ui| {
                        ui.label("Output:");
                        if ui.button("Copy to Clipboard").clicked() {
                            if let Ok(mut clipboard) = Clipboard::new() {
                                let _ = clipboard.set_text(&self.output_text);
                            }
                        }
                    });
                    
                    egui::ScrollArea::vertical()
                        .id_salt("output_scroll")
                        .min_scrolled_height(200.0)
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.add_sized(
                                [ui.available_width(), 200.0],
                                egui::TextEdit::multiline(&mut self.output_text)
                                    .font(egui::TextStyle::Monospace)
                            ).context_menu(|ui| {
                                if ui.button("Copy").clicked() {
                                    if let Ok(mut clipboard) = Clipboard::new() {
                                        let _ = clipboard.set_text(&self.output_text);
                                    }
                                    ui.close_menu();
                                }
                                if ui.button("Select All").clicked() {
                                    ui.close_menu();
                                }
                            });
                        });
                    
                    ui.add_space(30.0);
                });
                ui.add_space(10.0);
            });
        });
    }
}

fn strip_line_numbers(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let code_start = line
                .chars()
                .position(|c| !c.is_ascii_digit())
                .unwrap_or(line.len());
            &line[code_start..]
        })
        .collect::<Vec<_>>()
        .join("\n")
}
