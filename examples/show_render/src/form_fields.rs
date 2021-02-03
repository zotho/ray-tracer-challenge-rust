use crate::view::View;
use rustic_ray::{Point, Vector};

#[derive(Debug, Default)]
pub struct FormFields {
    from: String,
    to: String,
    up: String,
    fov: String,
}

impl FormFields {
    pub fn from_view(view: &View) -> Self {
        FormFields {
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
