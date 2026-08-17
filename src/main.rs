#![windows_subsystem = "windows"]

use eframe::egui;

// ----------------------------------------------------------------------------
// Clipboard Helper Functions
// ----------------------------------------------------------------------------

fn set_clipboard_text(text: &str) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text);
    }
}

fn get_clipboard_text() -> String {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        cb.get_text().unwrap_or_default()
    } else {
        String::new()
    }
}

// ----------------------------------------------------------------------------
// String & Formatting Helpers
// ----------------------------------------------------------------------------

fn add_commas(s: &str) -> String {
    if s == "Error" || s.is_empty() {
        return s.to_string();
    }
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let is_neg = int_part.starts_with('-');
    let digits = if is_neg { &int_part[1..] } else { int_part };

    let mut formatted_rev = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted_rev.push(',');
        }
        formatted_rev.push(c);
    }
    let mut result: String = formatted_rev.chars().rev().collect();
    if is_neg {
        result.insert(0, '-');
    }
    if parts.len() > 1 {
        result.push('.');
        result.push_str(parts[1]);
    }
    result
}

fn format_history_with_commas(hist: &str) -> String {
    if hist.is_empty() {
        return String::new();
    }
    hist.split(' ')
        .map(|token| {
            if token.chars().any(|c| c.is_ascii_digit()) {
                add_commas(token)
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_result(val: f64) -> String {
    if val.is_infinite() || val.is_nan() {
        return "Error".to_string();
    }
    let s = format!("{:.10}", val);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() {
        "0".to_string()
    } else {
        s.to_string()
    }
}

fn eval_op(first: f64, op: &str, second: f64) -> f64 {
    match op {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => {
            if second == 0.0 {
                f64::NAN
            } else {
                first / second
            }
        }
        _ => second,
    }
}

// ----------------------------------------------------------------------------
// Calculator Application State
// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct CalculatorApp {
    current_input: String,
    history_text: String,
    first_number: f64,
    pending_operator: Option<String>,
    active_key_pressed: String,
    clear_on_next_input: bool,
    is_repeat_mode: bool,
    last_operator: String,
    last_operand: f64,
}

impl CalculatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn handle_button_click(&mut self, btn: &str) {
        match btn {
            "C" => {
                self.current_input.clear();
                self.history_text.clear();
                self.first_number = 0.0;
                self.pending_operator = None;
                self.clear_on_next_input = false;
                self.is_repeat_mode = false;
                self.last_operator.clear();
                self.last_operand = 0.0;
            }
            "+/-" => {
                if !self.current_input.is_empty() && self.current_input != "Error" {
                    if self.current_input.starts_with('-') {
                        self.current_input.remove(0);
                    } else {
                        self.current_input.insert(0, '-');
                    }
                }
            }
            "%" => {
                if let Ok(val) = self.current_input.parse::<f64>() {
                    let res = val / 100.0;
                    self.current_input = format_result(res);
                }
            }
            "+" | "-" | "*" | "/" => {
                let val = self.current_input.parse::<f64>().unwrap_or(0.0);
                if let Some(op) = &self.pending_operator {
                    if !self.clear_on_next_input {
                        let res = eval_op(self.first_number, op, val);
                        self.first_number = res;
                        self.current_input = format_result(res);
                    }
                } else {
                    self.first_number = val;
                }
                self.pending_operator = Some(btn.to_string());
                self.history_text = format!("{} {}", format_result(self.first_number), btn);
                self.clear_on_next_input = true;
                self.is_repeat_mode = false;
            }
            "=" => {
                if let Some(op) = self.pending_operator.clone() {
                    let second = if self.clear_on_next_input && !self.is_repeat_mode {
                        self.first_number
                    } else {
                        self.current_input.parse::<f64>().unwrap_or(0.0)
                    };

                    let res = eval_op(self.first_number, &op, second);

                    self.history_text = format!(
                        "{} {} {} =",
                        format_result(self.first_number),
                        op,
                        format_result(second)
                    );
                    self.last_operator = op;
                    self.last_operand = second;
                    self.first_number = res;
                    self.current_input = format_result(res);
                    self.pending_operator = None;
                    self.clear_on_next_input = true;
                    self.is_repeat_mode = true;
                } else if self.is_repeat_mode && !self.last_operator.is_empty() {
                    let second = self.last_operand;
                    let res = eval_op(self.first_number, &self.last_operator, second);
                    self.history_text = format!(
                        "{} {} {} =",
                        format_result(self.first_number),
                        self.last_operator,
                        format_result(second)
                    );
                    self.first_number = res;
                    self.current_input = format_result(res);
                    self.clear_on_next_input = true;
                }
            }
            "." => {
                if self.clear_on_next_input {
                    self.current_input = "0.".to_string();
                    self.clear_on_next_input = false;
                } else if !self.current_input.contains('.') {
                    if self.current_input.is_empty() {
                        self.current_input.push_str("0.");
                    } else {
                        self.current_input.push('.');
                    }
                }
            }
            _ => {
                if self.clear_on_next_input {
                    self.current_input = btn.to_string();
                    self.clear_on_next_input = false;
                } else if self.current_input == "0" {
                    self.current_input = btn.to_string();
                } else {
                    self.current_input.push_str(btn);
                }
                self.is_repeat_mode = false;
            }
        }
    }

    fn handle_keyboard_events(&mut self, ctx: &egui::Context) {
        self.active_key_pressed.clear();

        let mut copy_requested = false;

        ctx.input(|i| {
            // OS Copy Event অথবা Ctrl+C / Cmd+C ডিটেক্ট করা
            for event in &i.events {
                if matches!(event, egui::Event::Copy) {
                    copy_requested = true;
                }
            }

            if i.modifiers.command && i.key_pressed(egui::Key::C) {
                copy_requested = true;
            }
        });

        // কপির অনুরোধ আসলে রেজাল্ট বা ইনপুট ক্লিপবোর্ডে পাঠানো
        if copy_requested {
            let to_copy = if !self.current_input.is_empty() {
                self.current_input.clone()
            } else {
                format_result(self.first_number)
            };
            set_clipboard_text(&to_copy);
            ctx.output_mut(|o| o.copied_text = to_copy);
        }

        ctx.input(|i| {
            // Paste (Ctrl + V)
            if i.modifiers.command && i.key_pressed(egui::Key::V) {
                let mut pasted = get_clipboard_text();
                pasted.retain(|c| c.is_ascii_digit() || c == '.' || c == '-');
                if !pasted.is_empty() {
                    if self.clear_on_next_input {
                        self.current_input.clear();
                        self.clear_on_next_input = false;
                    }
                    self.current_input = pasted;
                    self.is_repeat_mode = false;
                }
                return;
            }

            // BackSpace
            if i.key_pressed(egui::Key::Backspace)
                && !self.current_input.is_empty()
                && self.current_input != "Error"
            {
                self.current_input.pop();
                self.is_repeat_mode = false;
            }

            // Escape / 'c'
            if i.key_pressed(egui::Key::Escape) {
                self.active_key_pressed = "C".to_string();
                self.handle_button_click("C");
                return;
            }

            // Text Events
            for event in &i.events {
                if let egui::Event::Text(text) = event {
                    for ch in text.chars() {
                        let btn = match ch {
                            '0'..='9' | '.' => Some(ch.to_string()),
                            '+' | '-' | '*' | '/' => Some(ch.to_string()),
                            '=' | '\r' | '\n' => Some("=".to_string()),
                            'c' | 'C' => Some("C".to_string()),
                            '%' => Some("%".to_string()),
                            _ => None,
                        };

                        if let Some(b) = btn {
                            self.active_key_pressed = b.clone();
                            self.handle_button_click(&b);
                        }
                    }
                }
            }

            // Enter
            if i.key_pressed(egui::Key::Enter) {
                self.active_key_pressed = "=".to_string();
                self.handle_button_click("=");
            }
        });
    }
}

// ----------------------------------------------------------------------------
// GUI Render (eframe::App Implementation)
// ----------------------------------------------------------------------------

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visual = egui::Visuals::dark();
        visual.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 25, 25);
        visual.widgets.inactive.bg_fill = egui::Color32::from_rgb(51, 51, 51);
        visual.widgets.hovered.bg_fill = egui::Color32::from_rgb(66, 66, 66);
        visual.widgets.active.bg_fill = egui::Color32::from_rgb(82, 82, 82);
        visual.window_rounding = egui::Rounding::ZERO;
        ctx.set_visuals(visual);

        self.handle_keyboard_events(ctx);

        // চারপাশে বামে-ডানে-উপরে ১০px এবং নিচে ১২px বা ১৫px প্যাডিং
        let panel_frame = egui::Frame::none()
            .fill(egui::Color32::from_rgb(25, 25, 25))
            .inner_margin(egui::Margin {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 12.0, // নিচের বর্ডারের গ্যাপ বাড়াতে এই ভ্যালুটি প্রয়োজনমতো বাড়াতে পারেন
            });

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let max_width = ui.available_width();

            // 1. History Display
            let formatted_hist = format_history_with_commas(&self.history_text);
            let (_, hist_rect) = ui.allocate_space(egui::vec2(max_width, 24.0));

            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(hist_rect), |ui| {
                ui.set_clip_rect(hist_rect);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(formatted_hist)
                                .size(18.0)
                                .color(egui::Color32::from_rgb(153, 153, 153)),
                        )
                            .selectable(false)
                            .wrap_mode(egui::TextWrapMode::Truncate),
                    );
                });
            });

            ui.add_space(4.0);

            // 2. Main Input/Result Display
            let raw_disp = if self.current_input.is_empty() {
                "0"
            } else {
                &self.current_input
            };
            let formatted_disp = add_commas(raw_disp);

            let (_, disp_rect) = ui.allocate_space(egui::vec2(max_width, 50.0));

            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(disp_rect), |ui| {
                ui.set_clip_rect(disp_rect);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(formatted_disp)
                                .size(38.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        )
                            .selectable(false)
                            .wrap_mode(egui::TextWrapMode::Truncate),
                    );
                });
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            // 3. Calculator Buttons
            let button_rows = [
                vec!["C", "+/-", "%", "/"],
                vec!["7", "8", "9", "*"],
                vec!["4", "5", "6", "-"],
                vec!["1", "2", "3", "+"],
                vec!["0", ".", "="],
            ];

            let spacing = 4.0;
            let bottom_gap = 8.0; // বাটন সারির নিচে বাড়তি গ্যাপ
            let available_height = ui.available_height() - bottom_gap;

            let btn_width = ((max_width - (spacing * 3.0)) / 4.0).max(10.0);
            let btn_height = ((available_height - (spacing * 4.0)) / 5.0).max(10.0);

            let row_count = button_rows.len();
            for (idx, row) in button_rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;
                    for btn_text in row {
                        let is_operator = matches!(*btn_text, "=" | "+" | "-" | "*" | "/");
                        let is_keyboard_pressed = *btn_text == self.active_key_pressed;

                        let mut button = egui::Button::new(
                            egui::RichText::new(*btn_text).size(22.0).strong(),
                        );

                        if is_keyboard_pressed {
                            let fill = if *btn_text == "=" {
                                egui::Color32::from_rgb(31, 151, 245)
                            } else {
                                egui::Color32::from_rgb(82, 82, 82)
                            };
                            button = button.fill(fill);
                        } else if is_operator {
                            if *btn_text == "=" {
                                button = button.fill(egui::Color32::from_rgb(0, 120, 215));
                            } else {
                                button = button.fill(egui::Color32::from_rgb(38, 38, 38));
                            }
                        }

                        let width = if *btn_text == "0" {
                            (btn_width * 2.0) + spacing
                        } else {
                            btn_width
                        };

                        if ui.add_sized([width, btn_height], button).clicked() {
                            self.handle_button_click(btn_text);
                        }
                    }
                });

                if idx < row_count - 1 {
                    ui.add_space(spacing);
                }
            }
        });
    }
}


fn load_icon() -> Option<egui::IconData> {
    let icon_bytes = include_bytes!("../icon.ico"); // Reusing your .ico file
    let image = image::load_from_memory(icon_bytes).ok()?.to_rgba8();
    let (width, height) = image.dimensions();

    Some(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

// ----------------------------------------------------------------------------
// Main Entry Point
// ----------------------------------------------------------------------------

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([352.0, 520.0]) // (440 / 1.25 = 352, 650 / 1.25 = 520)
        .with_min_inner_size([320.0, 480.0])   // মিনিমাম উইন্ডো সাইজ
        .with_title("Calculator");

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Calculator",
        options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}