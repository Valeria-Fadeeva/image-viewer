use fltk::{
    app::*,
    enums::{Color, FrameType},
    frame::Frame,
    group::{Pack, PackType},
    image::*,
    prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let xywh = screen_xywh(0);
    let (screen_w, screen_h) = (xywh.2, xywh.3);

    let default_image = "img.jpg";

    let panel_height = 68;
    let small_element_height = 30;
    let padding = 5;
    let bgcolor = 0x1b1b1b;
    let fgcolor = 0xaaaaaa;
    let font_size = 12;

    let panel_height_f: f32 = panel_height as f32;
    let screen_h_f: f32 = screen_h as f32;
    let half_h_f: f32 = (screen_h_f - panel_height_f) / 2.0;
    let half_h: i32 = half_h_f.round() as i32;

    let screen_w_f: f32 = screen_w as f32;
    let half_w_f: f32 = screen_w_f / 2.0;
    let half_w: i32 = half_w_f.round() as i32;

    let half_half_w_f: f32 = half_w_f / 2.0;
    let half_half_w: i32 = half_half_w_f.round() as i32;

    let w_1_of_3_f: f32 = screen_w_f * 0.33;
    let w_1_of_3: i32 = w_1_of_3_f.round() as i32;

    let w_2_of_3_f: f32 = w_1_of_3_f * 2.0;
    let w_2_of_3: i32 = w_2_of_3_f.round() as i32;

    //                                                                 x, y, w, h
    let mut frame1: (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut status_frame1: (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut frame2: (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut status_frame2: (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut frame3: (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut status_frame3: (i32, i32, i32, i32) = (0, 0, 0, 0);

    let loy = "h33";

    // one_image_layout
    let one_image_layout = (
        0,
        0,
        screen_w,
        screen_h - small_element_height - panel_height,
    );

    let one_image_status_layout = (
        0,
        screen_h - panel_height - small_element_height,
        screen_w,
        small_element_height,
    );

    if loy == "v" {
        // three_image_layout vertical
        frame1 = (0, 0, screen_w, half_h - small_element_height);

        status_frame1 = (
            0,
            half_h - small_element_height,
            screen_w,
            small_element_height,
        );

        frame2 = (0, half_h + 1, half_w, half_h - small_element_height);

        status_frame2 = (
            0,
            screen_h - panel_height - small_element_height,
            half_w,
            small_element_height,
        );

        frame3 = (
            half_w + 1,
            half_h + 1,
            half_w,
            half_h - small_element_height,
        );

        status_frame3 = (
            half_w + 1,
            screen_h - panel_height - small_element_height,
            half_w,
            small_element_height,
        );
    } else if loy == "h" {
        // three_image_layout horizontal
        frame1 = (0, 0, half_w, screen_h - panel_height - small_element_height);

        status_frame1 = (
            0,
            screen_h - panel_height - small_element_height,
            half_w,
            small_element_height,
        );

        frame2 = (
            half_w + 1,
            0,
            half_half_w - 1,
            screen_h - panel_height - small_element_height,
        );

        status_frame2 = (
            half_w + 1,
            screen_h - panel_height - small_element_height,
            half_half_w - 1,
            small_element_height,
        );

        frame3 = (
            half_w + half_half_w + 1,
            0,
            half_half_w - 1,
            screen_h - panel_height - small_element_height,
        );

        status_frame3 = (
            half_w + half_half_w + 1,
            screen_h - panel_height - small_element_height,
            half_half_w - 1,
            small_element_height,
        );
    } else if loy == "h33" {
        // three_image_layout horizontal 33%
        frame1 = (
            0,
            0,
            w_1_of_3,
            screen_h - panel_height - small_element_height,
        );

        status_frame1 = (
            0,
            screen_h - panel_height - small_element_height,
            w_1_of_3,
            small_element_height,
        );

        frame2 = (
            w_1_of_3 + 1,
            0,
            w_1_of_3 - 1,
            screen_h - panel_height - small_element_height,
        );

        status_frame2 = (
            w_1_of_3 + 1,
            screen_h - panel_height - small_element_height,
            w_1_of_3 - 1,
            small_element_height,
        );

        frame3 = (
            w_2_of_3 + 2,
            0,
            w_1_of_3,
            screen_h - panel_height - small_element_height,
        );

        status_frame3 = (
            w_2_of_3 + 2,
            screen_h - panel_height - small_element_height,
            w_1_of_3,
            small_element_height,
        );
    }

    let application = App::default();
    let theme = ColorTheme::new(color_themes::DARK_THEME);
    theme.apply();

    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    let widget_scheme = WidgetScheme::new(SchemeType::Fluent);
    widget_scheme.apply();

    let mut win = Window::default()
        .with_size(screen_w, screen_h - panel_height)
        .center_screen()
        .with_label("Просмотр результата");

    if args.len() > 2 && args.len() <= 4 {
        let mut vpack1 = Pack::new(frame1.0, frame1.1, frame1.2, frame1.3, "");
        let mut main_frame = Frame::default()
            .with_size(frame1.2, frame1.3)
            .with_label("");
        let mut main_frame_status_line = Frame::default()
            .with_size(status_frame1.2, status_frame1.3)
            .with_label("Status");
        vpack1.end();
        vpack1.set_type(PackType::Vertical);

        let mut vpack2 = Pack::new(frame2.0, frame2.1, frame2.2, frame2.3, "");
        let mut sub_left_frame = Frame::default()
            .with_size(frame2.2, frame2.3)
            .with_label("");
        let mut sub_left_frame_status_line = Frame::default()
            .with_size(status_frame2.2, status_frame2.3)
            .with_label("Status");
        vpack2.end();
        vpack2.set_type(PackType::Vertical);

        let mut vpack3 = Pack::new(frame3.0, frame3.1, frame3.2, frame3.3, "");
        let mut sub_right_frame = Frame::default()
            .with_size(frame3.2, frame3.3)
            .with_label("");
        let mut sub_right_frame_status_line = Frame::default()
            .with_size(status_frame3.2, status_frame3.3)
            .with_label("Status");
        vpack3.end();
        vpack3.set_type(PackType::Vertical);

        let mut filepath0: &str = if args.len() > 1 {
            &args[1]
        } else {
            default_image
        };
        filepath0 = filepath0.trim();
        let mut img0 = SharedImage::load(&std::path::Path::new(filepath0)).unwrap();

        let mut filepath1: &str = if args.len() > 2 {
            &args[2]
        } else {
            default_image
        };
        filepath1 = filepath1.trim();
        let mut img1 = SharedImage::load(&std::path::Path::new(filepath1)).unwrap();

        let mut filepath2: &str = if args.len() > 3 {
            &args[3]
        } else {
            default_image
        };
        filepath2 = filepath2.trim();
        let mut img2 = SharedImage::load(&std::path::Path::new(filepath2)).unwrap();

        if loy == "v" {
            img0.scale(
                screen_w - padding * 2,
                half_h - padding * 2 - small_element_height,
                true,
                true,
            );
            img1.scale(
                half_w - padding * 2,
                half_h - padding * 2 - small_element_height,
                true,
                true,
            );
            img2.scale(
                half_w - padding * 2,
                half_h - padding * 2 - small_element_height,
                true,
                true,
            );
        } else if loy == "h" {
            img0.scale(
                half_w - padding * 2,
                screen_h - padding * 2 - small_element_height,
                true,
                true,
            );
            img1.scale(
                half_half_w - padding * 2,
                screen_h - padding * 2 - small_element_height,
                true,
                true,
            );
            img2.scale(
                half_half_w - padding * 2,
                screen_h - padding * 2 - small_element_height,
                true,
                true,
            );
        } else if loy == "h33" {
            img0.scale(
                w_1_of_3 - padding * 2,
                screen_h - padding * 2 - small_element_height,
                true,
                true,
            );
            img1.scale(
                w_1_of_3 - padding * 2,
                screen_h - padding * 2 - small_element_height,
                true,
                true,
            );
            img2.scale(
                w_1_of_3 - padding * 2,
                screen_h - padding * 2 - small_element_height,
                true,
                true,
            );
        }

        main_frame.set_image(Some(img0));
        let arg0 = format!("{}", filepath0);
        main_frame.set_frame(FrameType::BorderBox);
        main_frame_status_line.set_frame(FrameType::BorderBox);
        main_frame_status_line.set_label(&*arg0);
        main_frame_status_line.set_color(Color::from_u32(bgcolor));
        main_frame_status_line.set_label_color(Color::from_u32(fgcolor));
        main_frame_status_line.set_label_size(font_size);

        sub_left_frame.set_image(Some(img1));
        let arg1 = format!("{}", filepath1);
        sub_left_frame.set_frame(FrameType::BorderBox);
        sub_left_frame_status_line.set_frame(FrameType::BorderBox);
        sub_left_frame_status_line.set_label(&*arg1);
        sub_left_frame_status_line.set_color(Color::from_u32(bgcolor));
        sub_left_frame_status_line.set_label_color(Color::from_u32(fgcolor));
        sub_left_frame_status_line.set_label_size(font_size);

        sub_right_frame.set_image(Some(img2));
        let arg2 = format!("{}", filepath2);
        sub_right_frame.set_frame(FrameType::BorderBox);
        sub_right_frame_status_line.set_frame(FrameType::BorderBox);
        sub_right_frame_status_line.set_label(&*arg2);
        sub_right_frame_status_line.set_color(Color::from_u32(bgcolor));
        sub_right_frame_status_line.set_label_color(Color::from_u32(fgcolor));
        sub_right_frame_status_line.set_label_size(font_size);
    } else {
        let mut vpack0 = Pack::new(
            one_image_layout.0,
            one_image_layout.1,
            one_image_layout.2,
            one_image_layout.3,
            "",
        );
        //vpack.set_spacing(0);
        let mut main_frame = Frame::default()
            .with_size(one_image_layout.2, one_image_layout.3)
            .with_label("");
        let mut main_frame_status_line = Frame::default()
            .with_size(one_image_status_layout.2, one_image_status_layout.3)
            .with_label("Status");
        vpack0.end();
        vpack0.set_type(PackType::Vertical);

        let mut filepath0: &str = if args.len() > 1 {
            &args[1]
        } else {
            default_image
        };
        filepath0 = filepath0.trim();
        let mut img0 = SharedImage::load(&std::path::Path::new(filepath0)).unwrap();
        img0.scale(
            screen_w,
            screen_h - small_element_height - panel_height,
            true,
            true,
        );

        main_frame.set_image(Some(img0));
        let mut arg: &str = if args.len() > 1 { filepath0 } else { &args[0] };
        arg = arg.trim();
        let arg0 = format!("{}", arg);
        main_frame.set_frame(FrameType::BorderBox);
        main_frame_status_line.set_frame(FrameType::BorderBox);
        main_frame_status_line.set_label(&*arg0);
        main_frame_status_line.set_color(Color::from_u32(bgcolor));
        main_frame_status_line.set_label_color(Color::from_u32(fgcolor));
        main_frame_status_line.set_label_size(font_size);
    }

    win.make_resizable(true);
    win.end();
    win.show();

    application.run().unwrap();
}
