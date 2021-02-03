use macroquad::prelude::*;
use megaui_macroquad::{
    draw_megaui, draw_window,
    megaui::{self, hash},
    WindowParams,
};

use crate::{form_fields::FormFields, view::View};

#[derive(Debug, Default)]
pub struct Inputs {
    mouse_x: f32,
    mouse_y: f32,
    mouse_dx: f32,
    mouse_dy: f32,
}

impl Inputs {
    pub fn new() -> Self {
        let (mouse_x, mouse_y) = mouse_position();
        Inputs {
            mouse_x,
            mouse_y,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
        }
    }

    pub fn update(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_dx = mouse_x - self.mouse_x;
        self.mouse_dy = mouse_y - self.mouse_y;
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Show".to_owned(),
        fullscreen: false,
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut view = View::new();
    let mut win_fields = FormFields::from_view(&view);
    let mut inputs = Inputs::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        inputs.update();

        let sw = screen_width();
        let sh = screen_height();

        let ix = sw / 2.0 - view.width() / 2.0;
        let iy = sh / 2.0 - view.height() / 2.0;

        clear_background(WHITE);

        draw_texture(view.texture, ix, iy, WHITE);

        draw_window(
            hash!(),
            vec2(20., 20.),
            vec2(300., 130.),
            WindowParams {
                label: "Settings (You can change only numbers)".to_string(),
                close_button: false,
                ..Default::default()
            },
            |ui| {
                ui.label(None, "Camera:");
                for (i, (name, value)) in win_fields.as_slice().iter_mut().enumerate() {
                    ui.label(None, name);
                    ui.same_line(0.0);
                    megaui::widgets::InputText::new(hash!(hash!(), i))
                        .size(megaui::Vector2::new(250.0, 19.0))
                        .filter_numbers()
                        .ui(ui, value);
                }
            },
        );

        if win_fields.to_view(&mut view).is_err() {
            win_fields = FormFields::from_view(&view);
        }

        if is_key_pressed(KeyCode::Left) {
            view.rotate_y(std::f64::consts::PI / 6.0);
            win_fields = FormFields::from_view(&view);
        }
        if is_key_pressed(KeyCode::Right) {
            view.rotate_y(-std::f64::consts::PI / 6.0);
            win_fields = FormFields::from_view(&view);
        }
        if inputs.mouse_dx != 0.0 {
            view.rotate_y(-inputs.mouse_dx as f64 / 100.0);
            win_fields = FormFields::from_view(&view);
        }

        view.update();

        draw_megaui();
        next_frame().await
    }
}

pub fn show() {
    main()
}
