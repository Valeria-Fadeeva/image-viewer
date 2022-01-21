//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_variables)]

extern crate dotenv;
use dotenv::dotenv;
use std::env;
use fltk::{
    app::*,
    enums::{Color, FrameType},
    frame::Frame,
    image::*,
    prelude::*,
    window::Window,
};
use fltk_flex::Flex;
use fltk_theme::{
    color_themes,
    ColorTheme,
    SchemeType,
    ThemeType,
    WidgetScheme,
    WidgetTheme
};

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    /*let default_image: String = env::var("DEFAULT_IMAGE").unwrap();
    let default_image: &str = default_image.as_str();*/

    let padding: i32 = 5;

    let bgcolor: u32 = 0x1b1b1b;
    let fgcolor: u32 = 0xaaaaaa;

    let text_font_size: i32 = 12;

    let title_and_panel_h: i32 = 68;

    let small_element_h: i32 = 30;

    let xywh: (i32, i32, i32, i32) = screen_xywh(0);
    let (screen_w, screen_h) = (xywh.2, xywh.3);

    let pack_full_h = screen_h - title_and_panel_h;

    let application = App::default();

    let theme = ColorTheme::new(color_themes::DARK_THEME);
    theme.apply();

    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let widget_scheme = WidgetScheme::new(SchemeType::Fluent);
    widget_scheme.apply();

    let mut win = Window::default()
        .with_size(screen_w, pack_full_h)
        .center_screen()
        .with_label("Просмотр результата");

        let flex = Flex::default().with_size(screen_w, pack_full_h).row();

            let mut flex1 = Flex::default().size_of_parent().column();
            let mut frame1 = Frame::default().with_label("Empty Data");
            let mut frame1_status = Frame::default().with_label("Status");
            flex1.set_size(&mut frame1_status, small_element_h);
            flex1.end();

            frame1.set_frame(FrameType::BorderBox);
            frame1_status.set_frame(FrameType::BorderBox);
            frame1_status.set_color(Color::from_u32(bgcolor));
            frame1_status.set_label_color(Color::from_u32(fgcolor));
            frame1_status.set_label_size(text_font_size);

            if args.len() > 1 {
                let filepath1: &str = &args[1].trim();
                let mut img1 = SharedImage::load(&std::path::Path::new(filepath1)).unwrap();
                if args.len() > 2 {
                    frame1.set_size(frame1.w() / 3, frame1.h());
                }
                img1.scale(frame1.w() - padding * 2, frame1.h(), true, true);
                frame1.set_image(Some(img1));
                frame1.set_label("");
                let status1 = format!("{}", filepath1);
                frame1_status.set_label(&*status1);
            }

            if args.len() > 2 && args.len() <= 4 {
                let mut flex2 = Flex::default().size_of_parent().column();
                let mut frame2 = Frame::default().with_label("Empty Data");
                let mut frame2_status = Frame::default().with_label("Status");
                flex2.set_size(&mut frame2_status, small_element_h);
                flex2.end();

                frame2.set_frame(FrameType::BorderBox);
                frame2_status.set_frame(FrameType::BorderBox);
                frame2_status.set_color(Color::from_u32(bgcolor));
                frame2_status.set_label_color(Color::from_u32(fgcolor));
                frame2_status.set_label_size(text_font_size);

                if args.len() > 2 {
                    let filepath2: &str = &args[2].trim();
                    let mut img2 = SharedImage::load(&std::path::Path::new(filepath2)).unwrap();
                    frame2.set_size(frame2.w() / 3, frame2.h());
                    img2.scale(frame2.w() - padding * 2, frame2.h(), true, true);
                    frame2.set_image(Some(img2));
                    frame2.set_label("");
                    let status2 = format!("{}", filepath2);
                    frame2_status.set_label(&*status2);
                }

                let mut flex3 = Flex::default().size_of_parent().column();
                let mut frame3 = Frame::default().with_label("Empty Data");
                let mut frame3_status = Frame::default().with_label("Status");
                flex3.set_size(&mut frame3_status, small_element_h);
                flex3.end();

                frame3.set_frame(FrameType::BorderBox);
                frame3_status.set_frame(FrameType::BorderBox);
                frame3_status.set_color(Color::from_u32(bgcolor));
                frame3_status.set_label_color(Color::from_u32(fgcolor));
                frame3_status.set_label_size(text_font_size);

                if args.len() > 3 {
                    let filepath3: &str = &args[3].trim();
                    let mut img3 = SharedImage::load(&std::path::Path::new(filepath3)).unwrap();
                    frame3.set_size(frame3.w() / 3, frame3.h());
                    img3.scale(frame3.w() - padding * 2, frame3.h(), true, true);
                    frame3.set_image(Some(img3));
                    frame3.set_label("");
                    let status3 = format!("{}", filepath3);
                    frame3_status.set_label(&*status3);
                }
            }

        flex.end();

    win.end();
    win.make_resizable(true);
    win.show();

    application.run().unwrap();
}
