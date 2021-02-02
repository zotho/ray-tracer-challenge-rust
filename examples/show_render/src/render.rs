use image;

use rustic_ray::{
    Camera, World
};

use crate::{Opts, elapsed};

pub fn render(serialized_world: &str, camera: Camera, opts: &Opts) {
    elapsed!(
        let canvas = if opts.parallel {
            camera.render_parallel(serialized_world, opts.batch_size)
        } else {
            camera.render(&World::from_str(serialized_world))
        };
    );

    image::save_buffer(
        &opts.output,
        canvas.canvas_to_rgb_buffer().as_slice(),
        canvas.width as u32,
        canvas.height as u32,
        image::ColorType::Rgb8,
    ).unwrap();
}