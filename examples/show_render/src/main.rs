use std::f64::consts::PI;

use clap::Clap;

use rustic_ray::{
    Camera, Point, Transformation, Vector, 
};

mod elapsed;
mod new_world;
mod render;
mod show;
mod view;

#[derive(Clap)]
pub struct Opts {
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long, default_value = "out.png")]
    output: String,
    #[clap(short, long, default_value = "400")]
    hsize: usize,
    #[clap(short, long, default_value = "400")]
    vsize: usize,
    #[clap(short, long)]
    parallel: bool,
    #[clap(short, long, default_value = "100")]
    batch_size: usize,
    #[clap(short, long)]
    show: bool,
}

pub fn load_world(filename: Option<&str>) -> String {
    if let Some(filename) = filename {
        std::fs::read_to_string(filename).unwrap()
    } else {
        let world = new_world::new_world();
        let serialized_world = serde_json::to_string_pretty(&world).unwrap();
        std::fs::write("default_world.json", &serialized_world).unwrap();
        serialized_world
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut camera = Camera::new(opts.hsize, opts.vsize, PI / 4.0);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 1.5, -8.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let serialized_world = load_world(opts.input.as_deref());

    if opts.show {
        show::show();
    } else {
        render::render(&serialized_world, camera, &opts);
    }
}