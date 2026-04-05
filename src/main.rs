use eframe::egui;

fn caesar_cipher(text: &str, shift: i32) -> String {
    let shift = shift.rem_euclid(26);

    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = ((c as u8 - base + shift as u8) % 26) + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn xor(text: &str, keychar: char) -> String {
    text.bytes()
        .map(|c| (c ^ keychar as u8) as char)
        .collect()
}
#[derive(Default)]
struct AppState {
    // Caesar
    caesar_input: String,
    caesar_output: String,
    caesar_shift: i32,

    // XOR
    xor_input: String,
    xor_output: String,
    xor_key: char,
}
pub struct MyApp {
    state: AppState,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState {
                caesar_input: String::new(),
                caesar_output: String::new(),
                caesar_shift: 0,

                xor_input: String::new(),
                xor_output: String::new(),
                xor_key: 'A',
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("🔐 Cipher Master");
            ui.separator();
            ui.group(|ui| {
                ui.label("Caesar Cipher");

                ui.text_edit_singleline(&mut self.state.caesar_input);

                ui.add(
                    egui::Slider::new(&mut self.state.caesar_shift, -26..=26)
                        .text("Shift"),
                );

                if ui.button("Encrypt Caesar").clicked() {
                    self.state.caesar_output =
                        caesar_cipher(&self.state.caesar_input, self.state.caesar_shift);
                }

                ui.label("Output:");
                ui.label(&self.state.caesar_output);
            });

            ui.separator();
            
            ui.group(|ui| {
                ui.label("XOR Cipher");

                ui.text_edit_singleline(&mut self.state.xor_input);

                ui.horizontal(|ui| {
                    ui.label("Key:");

                    let mut key_string = self.state.xor_key.to_string();

                    if ui.text_edit_singleline(&mut key_string).changed() {
                        if let Some(c) = key_string.chars().next() {
                            self.state.xor_key = c;
                        }
                    }
                });

                if ui.button("Encrypt XOR").clicked() {
                    self.state.xor_output =
                        xor(&self.state.xor_input, self.state.xor_key);
                }

                ui.label("Output:");
                ui.label(&self.state.xor_output);
            });
        });
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cipher Master",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}