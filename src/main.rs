mod drawing;

use crate::drawing::Drawing;

use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily, Frame, Label, Pos2, RichText, Sense, Widget};
use egui_canvas::Canvas;

const CHAR_WIDTH: u32 = 16;
const CHAR_HEIGHT: u32 = 32;

impl eframe::App for Drawing {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            let response = ui.vertical(|ui| {
                for row in &mut self.data {
                    ui.horizontal(|ui| {
                        for cell in row.iter_mut() {
                            let resp = Label::new(cell.render().to_string()).sense(Sense::all()).ui(ui);
                            let rect = resp.rect;
                            match resp.hover_pos() {
                                Some(pos) => {
                                    println!("{}",pos);
                                    cell.set_pixel(
                                        (pos.x-rect.min.x) as u32, (pos.y-rect.min.y) as u32,
                                        (rect.max.x-rect.min.x) as u32,(rect.max.y-rect.min.y) as u32,
                                    );
                                    ctx.request_repaint();
                                },
                                None => {},
                            }
                        }
                    });
                }
            }).response;
            // response.sense.
            // match response.hover_pos() {
            //     None => {}
            //     Some(p) => {
            //         println!("hover pos: {:?}", p);
            //         self.update(p.x as u32, p.y as u32)
            //     }
            // }
        });
    }
}

fn main() {
    let mut drawing = Drawing::new(12, 60);
    // loop {
    //     if let Position { x, y } = Mouse::get_mouse_position() {
    //         if x > 0 && y > 0 {
    //             drawing.update(x as u32, y as u32);
    //         }
    //         drawing.draw();
    //     } else {
    //         break;
    //     }
    // }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BrailleDraw",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();

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
                .insert(0, "my_font".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::<Drawing>::new(Drawing::new(12, 60)))
        }),
    ).unwrap();
}