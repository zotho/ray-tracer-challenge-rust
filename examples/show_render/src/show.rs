use macroquad::prelude::*;
use megaui_macroquad::{
    draw_megaui, draw_window,
    megaui::{
        self, hash,
    },
    WindowParams,
};

use rustic_ray::{
    Point, Vector, 
};

use crate::view::View;

#[derive(Debug, Default)]
struct WindowFields {
    from: String,
    to: String,
    up: String,
    fov: String,
}

impl WindowFields {
    pub fn from_view(view: &View) -> Self {
        WindowFields {
            from: serde_json::to_string(&view.from).unwrap(),
            to: serde_json::to_string(&view.to).unwrap(),
            up: serde_json::to_string(&view.up).unwrap(),
            fov: view.fov.to_string(),
        }
    }

    pub fn to_view(&self, view: &mut View) -> Result<(), serde_json::Error> {
        let from: Point = serde_json::from_str(&self.from)?;
        let to: Point = serde_json::from_str(&self.to)?;
        let up: Vector = serde_json::from_str(&self.up)?;
        let fov: f64 = self.fov.parse().unwrap();
        if from != view.from || to != view.to || up != view.up || fov != view.fov {
            view.from = from;
            view.to = to;
            view.up = up;
            view.fov = fov;
            view.need_update = true;
        }
        Ok(())
    }

    pub fn as_slice(&mut self) -> [(&str, &mut String); 4] {
        [
            ("From:", &mut self.from),
            ("To:  ", &mut self.to),
            ("Up:  ", &mut self.up),
            ("FOV: ", &mut self.fov),
        ]
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
    let mut win_fields = WindowFields::from_view(&view);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

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
            }
        );

        if win_fields.to_view(&mut view).is_err() {
            win_fields = WindowFields::from_view(&view);
        }

        view.update();

        draw_megaui();
        next_frame().await
    }
}

pub fn show() {
    main()
}