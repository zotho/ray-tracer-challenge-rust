use std::f64::consts::PI;

use clap::Clap;
use macroquad::prelude::*;

use rustic_ray::{Camera, Point, Transformation, Vector};

use crate::{elapsed, load_world, Opts};

pub struct View {
    pub texture: Texture2D,
    pub from: Point,
    pub to: Point,
    pub up: Vector,
    pub fov: f64,
    pub opts: Opts,
    pub serialized_world: String,
    pub need_update: bool,
}

impl View {
    pub fn rotate_y(&mut self, angle: f64) {
        self.need_update = true;

        let dir = self.to - self.from;
        let new_dir = dir.rotate_y(angle);
        self.to = self.from + new_dir;
    }

    pub fn new() -> Self {
        let opts: Opts = Opts::parse();

        let fov = PI / 4.0;
        let mut camera = Camera::new(opts.hsize, opts.vsize, fov);

        let from = Point::new(0.0, 1.5, -8.0);
        let to = Point::new(0.0, 1.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        camera.transform = Transformation::view_transform(from, to, up);

        let serialized_world = load_world(opts.input.as_deref());

        let canvas = camera.render_parallel(&serialized_world, opts.batch_size);
        let bytes = canvas.canvas_to_rgba_buffer();

        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        let texture = Texture2D::from_rgba8(ctx, canvas.width as u16, canvas.height as u16, &bytes);

        View {
            texture,
            from,
            to,
            up,
            fov,
            opts,
            serialized_world,
            need_update: false,
        }
    }

    pub fn update(&mut self) {
        if !self.need_update {
            return;
        }

        let mut camera = Camera::new(self.opts.hsize, self.opts.vsize, self.fov);
        camera.transform = Transformation::view_transform(self.from, self.to, self.up);

        elapsed!(
            let canvas = camera.render_parallel(&self.serialized_world, self.opts.batch_size);
        );
        let bytes = canvas.canvas_to_rgba_buffer();

        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        self.texture.update(
            ctx,
            &Image {
                bytes,
                width: canvas.width as u16,
                height: canvas.height as u16,
            },
        );
        self.need_update = false;
    }

    pub fn width(&self) -> f32 {
        self.texture.width()
    }

    pub fn height(&self) -> f32 {
        self.texture.height()
    }
}
