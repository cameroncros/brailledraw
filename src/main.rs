mod drawing;

use clap::{Parser, Subcommand};
use crate::drawing::{Drawing, Mode};

use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily, Label, Sense, Widget};
use mouse_position::mouse_position::Mouse;
use mouse_position::mouse_position::Mouse::Position;

const CHAR_WIDTH: u32 = 16;
const CHAR_HEIGHT: u32 = 32;

impl eframe::App for Drawing {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.mode, Mode::None, "None");
                ui.radio_value(&mut self.mode, Mode::Draw, "Draw");
                ui.radio_value(&mut self.mode, Mode::Erase, "Erase");

                if ui.button("Save").clicked() {
                    self.draw();
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            ui.vertical(|ui| {
                for row in &mut self.data {
                    ui.horizontal(|ui| {
                        for cell in row.iter_mut() {
                            let resp = Label::new(cell.render().to_string()).sense(Sense::all()).ui(ui);
                            let rect = resp.rect;
                            if resp.hovered() {
                                ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                            }
                            match resp.hover_pos() {
                                Some(pos) => {
                                    if cell.set_pixel(
                                        &self.mode,
                                        (pos.x - rect.min.x) as u32, (pos.y - rect.min.y) as u32,
                                        (rect.max.x - rect.min.x) as u32, (rect.max.y - rect.min.y) as u32,
                                    ) {
                                        ctx.request_repaint();
                                    }
                                },
                                None => {},
                            }
                        }
                    });
                }
            });
        });
    }
}

#[derive(Subcommand)]
enum GuiMode {
    Terminal,
    Gui
}
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    mode: Option<GuiMode>
}

fn terminal_mode(mut drawing: Drawing) {
    loop {
        if let Position { x, y } = Mouse::get_mouse_position() {
            if x > 0 && y > 0 {
                drawing.update(x as u32, y as u32);
            }
            drawing.draw();
        } else {
            break;
        }
    }
}

fn gui_mode(drawing: Drawing) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BrailleDraw",
        options,
        Box::new(|cc| {
            let fonts = FontDefinitions::default();

            cc.egui_ctx.set_fonts(fonts);


            let mut fonts = FontDefinitions::default();

            // Install my own font (maybe supporting non-latin characters):
            fonts.font_data.insert("my_font".to_owned(),
                                   std::sync::Arc::new(
                                       // .ttf and .otf supported
                                       FontData::from_static(include_bytes!("Unicode-Braille.ttf")),
                                   )
            );

            // Put my font first (highest priority):
            fonts.families.get_mut(&FontFamily::Proportional).unwrap()
                .insert(1, "my_font".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::<Drawing>::new(drawing))
        }),
    ).unwrap();
}

fn main() {
    let args = Args::parse();

    let drawing = Drawing::new(24, 70);
    match args.mode {
        Some(GuiMode::Terminal) => {
            terminal_mode(drawing);
        }
        Some(GuiMode::Gui) | None => {
            gui_mode(drawing);
        }
    }

}