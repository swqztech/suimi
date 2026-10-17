use crate::generate::{self, Mode, PasswordOptions, Strength};
use eframe::egui;

pub struct SuimiApp {
    mode: Mode,
    password_opts: PasswordOptions,
    username_length: u32,
    output: String,
    strength: Strength,
}

impl SuimiApp {
    pub fn new() -> Self {
        let password_opts = PasswordOptions::default();
        let strength = generate::password_strength(&password_opts);
        let output = generate::generate_password(&password_opts);
        Self {
            mode: Mode::Password,
            password_opts,
            username_length: 12,
            output,
            strength,
        }
    }

    fn roll_once(&mut self) {
        self.output = match self.mode {
            Mode::Password => generate::generate_password(&self.password_opts),
            Mode::Username => generate::generate_username(self.username_length),
            Mode::Nickname => generate::generate_nickname(),
        };
        if self.mode == Mode::Password {
            self.strength = generate::password_strength(&self.password_opts);
        }
    }

    fn roll_six(&mut self) {
        if self.mode == Mode::Password {
            self.output = generate::generate_passwords(&self.password_opts, 6);
            self.strength = generate::password_strength(&self.password_opts);
        } else {
            let lines: Vec<String> = (0..6)
                .map(|_| match self.mode {
                    Mode::Username => generate::generate_username(self.username_length),
                    Mode::Nickname => generate::generate_nickname(),
                    Mode::Password => unreachable!(),
                })
                .collect();
            self.output = lines.join("\n");
        }
    }

}

impl eframe::App for SuimiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::paint_background(ctx);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(22.0, 18.0)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    header(ui);
                    ui.add_space(14.0);
                    if let Some(m) = mode_tabs(ui, self.mode) {
                        self.mode = m;
                        self.roll_once();
                    }
                    ui.add_space(16.0);
                    output_box(ui, &self.output);
                    if self.mode == Mode::Password {
                        ui.add_space(8.0);
                        strength_bar(ui, &self.strength);
                    }
                    ui.add_space(18.0);

                    match self.mode {
                        Mode::Password => password_controls(ui, &mut self.password_opts),
                        Mode::Username => username_controls(ui, &mut self.username_length),
                        Mode::Nickname => nickname_hint(ui),
                    }

                    if self.mode == Mode::Password {
                        self.strength = generate::password_strength(&self.password_opts);
                    }

                    ui.add_space(20.0);
                    let output_copy = self.output.clone();
                    let mut want_roll = false;
                    let mut want_six = false;
                    action_row(ui, ctx, &output_copy, &mut want_roll, &mut want_six);
                    if want_roll {
                        self.roll_once();
                    }
                    if want_six {
                        self.roll_six();
                    }
                    ui.add_space(16.0);
                    footer(ui);
                });
            });
    }
}

mod theme {
    use eframe::egui;

    pub const BG: egui::Color32 = egui::Color32::from_rgb(26, 38, 42);
    pub const PANEL: egui::Color32 = egui::Color32::from_rgb(18, 28, 32);
    pub const GOLD: egui::Color32 = egui::Color32::from_rgb(212, 175, 95);
    pub const GOLD_DIM: egui::Color32 = egui::Color32::from_rgb(140, 118, 72);
    pub const TEXT: egui::Color32 = egui::Color32::from_rgb(210, 218, 222);
    pub const MUTED: egui::Color32 = egui::Color32::from_rgb(120, 135, 142);
    pub const GREEN: egui::Color32 = egui::Color32::from_rgb(72, 196, 122);
    pub const GREEN_BTN: egui::Color32 = egui::Color32::from_rgb(58, 178, 108);
    pub const PILL_ON: egui::Color32 = egui::Color32::from_rgb(45, 62, 68);
    pub const PILL_OFF: egui::Color32 = egui::Color32::from_rgb(32, 44, 48);
    pub const BORDER: egui::Color32 = egui::Color32::from_rgb(55, 72, 78);

    pub fn paint_background(ctx: &egui::Context) {
        let mut visuals = ctx.style().visuals.clone();
        visuals.dark_mode = true;
        visuals.panel_fill = BG;
        visuals.window_fill = BG;
        visuals.extreme_bg_color = PANEL;
        visuals.faint_bg_color = PILL_OFF;
        visuals.widgets.noninteractive.bg_fill = PILL_OFF;
        visuals.widgets.inactive.bg_fill = PILL_OFF;
        visuals.widgets.hovered.bg_fill = PILL_ON;
        visuals.widgets.active.bg_fill = PILL_ON;
        visuals.selection.bg_fill = GOLD.linear_multiply(0.35);
        visuals.widgets.noninteractive.fg_stroke.color = TEXT;
        visuals.widgets.inactive.fg_stroke.color = TEXT;
        visuals.override_text_color = Some(TEXT);
        ctx.set_visuals(visuals);
    }
}

fn header(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("随密")
                .size(36.0)
                .color(theme::GOLD)
                .strong(),
        );
        ui.label(
            egui::RichText::new("掷一次，拿一把新钥匙")
                .size(14.0)
                .color(theme::TEXT),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("本地生成 · 不上传 · 系统安全随机源")
                .size(11.0)
                .color(theme::MUTED),
        );
    });
}

fn mode_tabs(ui: &mut egui::Ui, mode: Mode) -> Option<Mode> {
    let mut picked = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if tab_button(ui, "密码", mode == Mode::Password) {
            picked = Some(Mode::Password);
        }
        if tab_button(ui, "用户名", mode == Mode::Username) {
            picked = Some(Mode::Username);
        }
        if tab_button(ui, "网名", mode == Mode::Nickname) {
            picked = Some(Mode::Nickname);
        }
    });
    picked.filter(|m| *m != mode)
}

fn tab_button(ui: &mut egui::Ui, label: &str, selected: bool) -> bool {
    let stroke = if selected {
        egui::Stroke::new(1.5_f32, theme::GOLD)
    } else {
        egui::Stroke::new(1.0_f32, theme::BORDER)
    };
    let fill = if selected {
        theme::PILL_ON
    } else {
        theme::PILL_OFF
    };
    let text_color = if selected { theme::GOLD } else { theme::MUTED };
    let btn = egui::Button::new(egui::RichText::new(label).color(text_color).size(13.0))
        .fill(fill)
        .stroke(stroke)
        .min_size(egui::vec2(72.0, 30.0));
    ui.add(btn).clicked()
}

fn output_box(ui: &mut egui::Ui, text: &str) {
    egui::Frame::none()
        .fill(theme::PANEL)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .inner_margin(egui::Margin::symmetric(14.0, 16.0))
        .rounding(10.0)
        .show(ui, |ui| {
            ui.set_min_height(56.0);
            ui.label(
                egui::RichText::new(if text.is_empty() { "—" } else { text })
                    .monospace()
                    .size(15.0)
                    .color(egui::Color32::from_rgb(230, 236, 240)),
            );
        });
}

fn strength_bar(ui: &mut egui::Ui, strength: &Strength) {
    ui.horizontal(|ui| {
        let bar_w = ui.available_width() - 120.0;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(bar_w.max(80.0), 6.0), egui::Sense::hover());
        let painter = ui.painter();
        painter.rect_filled(rect, 3.0, egui::Color32::from_rgb(35, 48, 52));
        let fill_w = rect.width() * strength.ratio;
        let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(fill_w, rect.height()));
        painter.rect_filled(fill_rect, 3.0, theme::GREEN);
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(format!("{} · 约 {:.0} bit", strength.label, strength.bits))
                .size(12.0)
                .color(theme::MUTED),
        );
    });
}

fn password_controls(ui: &mut egui::Ui, opts: &mut PasswordOptions) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("长度").size(13.0).color(theme::MUTED));
        ui.add_space(8.0);
        ui.add(
            egui::Slider::new(&mut opts.length, 6..=64)
                .show_value(false)
                .trailing_fill(true),
        );
        ui.label(
            egui::RichText::new(format!("{}", opts.length))
                .size(14.0)
                .color(theme::GOLD)
                .strong(),
        );
    });
    ui.add_space(12.0);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
        pill_toggle(ui, "小写 a-z", &mut opts.lowercase);
        pill_toggle(ui, "大写 A-Z", &mut opts.uppercase);
        pill_toggle(ui, "数字 0-9", &mut opts.digits);
        pill_toggle(ui, "符号 !@#", &mut opts.symbols);
        pill_toggle(ui, "排除易混 0OIl1", &mut opts.exclude_ambiguous);
    });
}

fn username_controls(ui: &mut egui::Ui, length: &mut u32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("长度").size(13.0).color(theme::MUTED));
        ui.add_space(8.0);
        ui.add(egui::Slider::new(length, 4..=32).show_value(false).trailing_fill(true));
        ui.label(
            egui::RichText::new(format!("{}", length))
                .size(14.0)
                .color(theme::GOLD)
                .strong(),
        );
    });
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new("以小写字母开头，可含数字与下划线")
            .size(11.0)
            .color(theme::MUTED),
    );
}

fn nickname_hint(ui: &mut egui::Ui) {
    ui.label(
        egui::RichText::new("从内置词库随机组合中文网名，可带数字后缀")
            .size(11.0)
            .color(theme::MUTED),
    );
}

fn pill_toggle(ui: &mut egui::Ui, label: &str, value: &mut bool) {
    let on = *value;
    let stroke = if on {
        egui::Stroke::new(1.2_f32, theme::GOLD_DIM)
    } else {
        egui::Stroke::new(1.0_f32, theme::BORDER)
    };
    let fill = if on { theme::PILL_ON } else { theme::PILL_OFF };
    let color = if on { theme::GOLD } else { theme::MUTED };
    let btn = egui::Button::new(egui::RichText::new(label).size(12.0).color(color))
        .fill(fill)
        .stroke(stroke)
        .rounding(16.0);
    if ui.add(btn).clicked() {
        *value = !*value;
    }
}

fn secondary_button(label: &str, min_w: f32) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(label).size(13.0).color(theme::TEXT))
        .fill(theme::PILL_OFF)
        .stroke(egui::Stroke::new(1.0_f32, theme::BORDER))
        .min_size(egui::vec2(min_w, 36.0))
}

fn action_row(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    output: &str,
    want_roll: &mut bool,
    want_six: &mut bool,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        let roll_btn = egui::Button::new(
            egui::RichText::new("再掷一次")
                .size(14.0)
                .color(egui::Color32::WHITE)
                .strong(),
        )
        .fill(theme::GREEN_BTN)
        .min_size(egui::vec2(100.0, 36.0));
        if ui.add(roll_btn).clicked() {
            *want_roll = true;
        }

        if ui
            .add(secondary_button("复制", 72.0))
            .clicked()
            && !output.is_empty()
        {
            ctx.copy_text(output.to_string());
        }

        if ui.add(secondary_button("一次六个", 88.0)).clicked() {
            *want_six = true;
        }
    });
}

fn footer(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("Passroll · 字符从勾选集合中均匀抽取")
                .size(11.0)
                .color(theme::MUTED),
        );
    });
}
