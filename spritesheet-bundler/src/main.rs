use std::collections::HashMap;
use std::fs::{create_dir, read_dir};
use std::path::Path;
use std::str::FromStr;
use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};
use image::{GenericImage, ImageBuffer, ImageReader};

const RENDER_DIR: &str = "render";
const OUTPUT_DIR: &str = "out/";

#[derive(Default)]
struct Container {
    width: u32,
    height: u32,
    frames: u32,
    animation_data: HashMap<String, u32>,
}

#[derive(Clone, Copy, Debug)]
enum AnimationEvents {
    HitboxStart,
}

impl ToString for AnimationEvents {
    fn to_string(&self) -> String {
        match self {
            AnimationEvents::HitboxStart => "SpawnHitboxEvent".to_string(),
        }
    }
}

impl FromStr for AnimationEvents {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        if s == "hitbox_start" {
            Ok(Self::HitboxStart)
        } else {
            Err(anyhow!("Pose marker is not a valid animation event, '{s}'"))
        }
    }
}

fn animation_parts(path: &PathBuf) -> Vec<String> {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('-')
        .map(|s| s.to_string())
        .collect()
}

fn animation_base_name(path: &PathBuf) -> String {
    animation_parts(path)[0].clone()
}

fn animation_direction(path: &PathBuf) -> u32 {
    animation_parts(path)[1]
        .strip_prefix('o')
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn animation_frame(path: &PathBuf) -> u32 {
    animation_parts(path)[2]
        .strip_suffix(".png")
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn animation_name(path: &PathBuf) -> String {
    let parts: Vec<&str> = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('-')
        .collect();

    parts[0].to_string() + "-" + parts[1]
}

fn main() {
    let path = &env::args().nth(1).expect("Can't find target folder");
    let target_path = Path::new(path);

    if !Path::new(OUTPUT_DIR).exists() {
        create_dir(OUTPUT_DIR).unwrap();
    }

    let mut container = Container::default();
    let mut images = Vec::new();

    for path in read_dir(target_path.join(RENDER_DIR)).expect("Given path is not correct") {
        let path = match path {
            Ok(r) => r,
            Err(err) => panic!("Something wrong with path, {}", err),
        };

        if images.is_empty() {
            let img = ImageReader::open(path.path())
                .expect("Can't open image file")
                .decode()
                .expect("Failed to decode image");
            container.width = img.width();
            container.height = img.height();
        }
        images.push(path.path());
    }
    images.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut output_images = HashMap::new();

    let mut current_index = 0;
    let mut current_animation = animation_name(&images[0]);

    for image in &images {
        let base_animation = animation_base_name(image);
        let animation = animation_name(image);

        if animation != current_animation {
            if current_index > container.frames {
                container.frames = current_index;
            }

            container
                .animation_data
                .insert(base_animation.clone(), current_index);
            current_animation = animation;
            current_index = 0;
        }
        current_index += 1;
    }

    for animation in container.animation_data.keys() {
        output_images.insert(
            animation,
            ImageBuffer::new(container.width * container.frames, container.height * 4),
        );
    }

    for image in &images {
        let img = image::open(image).unwrap();

        let x_index = animation_frame(image);
        let y_index = animation_direction(image);

        let out_img = output_images.get_mut(&animation_base_name(image)).unwrap();

        assert!(
            x_index * container.width < out_img.width(),
            "{}, {}",
            x_index * container.width,
            out_img.width()
        );

        out_img
            .copy_from(&img, x_index * container.width, y_index * container.height)
            .unwrap();
    }

    for (animation, img) in output_images {
        img.save(OUTPUT_DIR.to_string() + &animation + ".png")
            .unwrap();
    }
}
