use std::collections::HashMap;
use std::fs::{self, create_dir, read_dir, read_to_string};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};
use image::ImageReader;

const RENDER_DIR: &str = "render";
const METADATA_FILE: &str = "metadata.csv";

const OUTPUT_DIR: &str = "out/";
const OUTPUT_RON_FILE: &str = "out.trickfilm.ron";
const OUTPUT_ASSET_MACRO_FILE: &str = "out-asset.txt";

const FRAME_RATE: f32 = 18.0;

#[derive(Default)]
struct Container {
    width: u32,
    height: u32,
    max_frames: u32,
    animation_frames: HashMap<String, u32>,
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

/// Name of the animation as it appears in blender.
///
/// For example `idle-o3-0013.png` will return `idle`.
fn animation_base_name(path: &PathBuf) -> String {
    animation_parts(path)[0].clone()
}

/// Orientation index of the animation.
///
/// For example `idle-o3-0013.png` will return `3`.
fn animation_direction(path: &PathBuf) -> u32 {
    animation_parts(path)[1]
        .strip_prefix('o')
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

/// Name + orientation of the given animation.
///
/// For example `idle-o3-0013.png` will return `idle-o3`.
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

/// Trickfilm string data for the given animation.
/// This contains all of the animation data, meaning all of the indices, times, events.
fn animation_format(
    start_index: u32,
    end_index: u32,
    animation: &str,
    metadata: &HashMap<String, Vec<(AnimationEvents, usize)>>,
) -> String {
    let duration = (end_index - start_index) as f32 / FRAME_RATE;

    let mut events_str = String::new();
    let key = animation
        .split('-')
        .next()
        .expect("Failed to get stem name of animation, unexpected format");
    if let Some(events) = metadata.get(key) {
        events_str += "\n\t\tevents: {\n";

        for (event, frame) in events {
            events_str += &format!("\t\t\t{}: ", frame);
            events_str += "{\n";
            events_str += &format!(
                "\t\t\t\t\"rantoni::assets::events::{}\": (\n",
                event.to_string()
            );

            match event {
                AnimationEvents::HitboxStart => {
                    events_str += &format!("\t\t\t\t\tmsg: \"{}\",\n", animation);
                }
            };

            events_str += "\t\t\t\t)\n";
            events_str += "\t\t\t},\n";
        }

        events_str += "\t\t},";
    }

    format!(
        "\t\"{animation}\": (
\t\tkeyframes: KeyframesRange((start: {start_index}, end: {end_index})),
\t\tduration: {duration},{events_str}
\t),
"
    )
}

fn write_trickfilm_file(images: &Vec<PathBuf>, container: &Container, path: &Path) {
    let metadata = read_metadata(&path.join(METADATA_FILE));
    let mut content = String::new();
    content += "{\n";

    let mut start_index = 0;
    let mut current_index = 0;

    let mut current_animation = animation_name(&images[0]);
    let mut current_base_animation = animation_base_name(&images[0]);

    for image in images {
        let animation_name = animation_name(image);
        let animation_base_name = animation_base_name(image);

        if animation_name != current_animation {
            content += &animation_format(start_index, current_index, &current_animation, &metadata);
            current_animation = animation_name;
            start_index = animation_direction(image)
                * container
                    .animation_frames
                    .get(&animation_base_name)
                    .expect("animation base name not present in anim data");
            current_index = start_index;
        }

        if animation_base_name != current_base_animation {
            start_index = 0;
            current_index = 0;
            current_base_animation = animation_base_name;
        }

        current_index += 1;
    }
    content += &animation_format(start_index, current_index, &current_animation, &metadata);

    content += "}";
    fs::write(OUTPUT_DIR.to_string() + OUTPUT_RON_FILE, content).unwrap();
}

fn write_assets_macro(
    images: &Vec<PathBuf>,
    container: &Container,
    name: &str,
    trickfilm_path: &str,
) {
    let layout_content = format!(
        "#[asset(texture_atlas_layout(tile_size_x = {}, tile_size_y = {}, columns = {}, rows = {}))]\npub {}_layout: Handle<TextureAtlasLayout>,\n",
        container.width,
        container.height,
        container.max_frames,
        8,
        name
    );
    let mut animation_content = "\n\n#[asset(paths(\n".to_string();

    let mut current_animation = animation_name(&images[0]);
    for image in images {
        let animation_name = animation_name(&image);
        if animation_name != current_animation {
            animation_content += &format!("\"{}#{}\",\n", trickfilm_path, current_animation);
            current_animation = animation_name;
        }
    }
    animation_content += &format!("\"{}#{}\",\n", trickfilm_path, current_animation);
    animation_content += "),collection(typed))]";

    fs::write(
        OUTPUT_DIR.to_string() + OUTPUT_ASSET_MACRO_FILE,
        layout_content + &animation_content,
    )
    .unwrap();
}

fn read_metadata(file: &Path) -> HashMap<String, Vec<(AnimationEvents, usize)>> {
    let mut hash_map: HashMap<String, Vec<(AnimationEvents, usize)>> = HashMap::new();
    let raw_metadata = read_to_string(file).expect("Can't open metadata file");
    if raw_metadata.is_empty() {
        return HashMap::new();
    }

    let contents = raw_metadata
        .strip_suffix('\n')
        .expect("Malformed metadata file, supposed to end on a newline")
        .to_string();
    for line in contents.split('\n') {
        let parts: Vec<&str> = line.split(',').collect();
        let key = parts[0].to_string();

        let mut value = match hash_map.get(&key) {
            Some(v) => v.to_vec(),
            None => Vec::new(),
        };

        value.push((
            AnimationEvents::from_str(parts[1])
                .expect("Failed to convert string to AnimationEvent"),
            parts[2]
                .parse::<usize>()
                .expect("Failed to parse string to usize"),
        ));

        hash_map.insert(key, value);
    }
    hash_map
}

fn add_frame_data_to_container(container: &mut Container, images: &Vec<PathBuf>) {
    let mut current_index = 0;
    let mut current_animation = animation_name(&images[0]);

    for image in images {
        let base_animation = animation_base_name(image);
        let animation = animation_name(image);

        if animation != current_animation {
            if current_index > container.max_frames {
                container.max_frames = current_index;
            }

            container
                .animation_frames
                .insert(base_animation.clone(), current_index);
            current_animation = animation;
            current_index = 0;
        }
        current_index += 1;
    }
}

fn get_images_and_container(parent_dir: &PathBuf) -> (Vec<PathBuf>, Container) {
    let mut container = Container::default();
    let mut images = Vec::new();

    for path in read_dir(parent_dir.join(RENDER_DIR)).expect("Given path is not correct") {
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

    add_frame_data_to_container(&mut container, &images);

    (images, container)
}

fn main() {
    let name = &env::args()
        .nth(1)
        .expect("must give name of the asset as second argument");
    let trickfilm_path = &env::args()
        .nth(2)
        .expect("must give relative trickfilm path to the asset as third argument");

    let mut parent_dir =
        env::current_dir().expect("Current dir is not valid, failed to get current dir");
    assert!(parent_dir.pop(), "parent dir must exist");

    if !Path::new(OUTPUT_DIR).exists() {
        create_dir(OUTPUT_DIR).unwrap();
    }

    let (images, container) = get_images_and_container(&parent_dir);

    for image in images.iter().filter(|f| {
        f.to_str()
            .expect("string convert from PathBuf shouldn't fail")
            .ends_with("-o0-0000.png")
    }) {
        let image_names = animation_base_name(image) + "-o?-*.png";
        println!("{}", image_names);
        Command::new("magick")
            .arg("montage")
            .arg("-background")
            .arg("none")
            .arg(parent_dir.join(RENDER_DIR).join(&image_names))
            .arg("-tile")
            .arg("19x")
            .arg("-geometry")
            .arg("+0+0")
            .arg("-limit")
            .arg("width")
            .arg(format!("{}", container.max_frames * container.width))
            .arg(OUTPUT_DIR.to_string() + &animation_base_name(image) + ".png")
            .status()
            .unwrap();
    }

    write_trickfilm_file(&images, &container, &parent_dir);
    write_assets_macro(&images, &container, name, trickfilm_path);

    print_stats(&container);
}

fn print_stats(container: &Container) {
    let naive_size = container.width
        * container.height
        * 4
        * 8
        * container.max_frames
        * container.animation_frames.len() as u32
        / 1000_000;

    let mut total_frames = 0;
    for frames in container.animation_frames.values() {
        total_frames += (frames * 8).div_ceil(container.max_frames) * container.max_frames;
    }
    let current_size = container.width * container.height * 4 * total_frames / 1000_000;

    println!(
        "\n{} MB - Total size of textures with naiive implementation",
        naive_size
    );

    println!(
        "{} MB - Total size of textures with current implementation",
        current_size
    );

    println!("{} MB - Total difference", naive_size - current_size);
    println!(
        "{:.1} % - Less size",
        100.0 - current_size as f32 * 100.0 / naive_size as f32
    );
}
