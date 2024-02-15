use std::env;

use camera::{Camera, CameraConfig};
use glam::vec3;
use material::{Dielectric, Lambertian, Light, Metal};
use math::{random, random_range};
use object::{ObjectCollection, Sphere};
use ray::Color;
use show_image::{create_window, event};
use texture::ImageTexture;

use crate::{bvh::BVHNode, texture::CheckerTexture};

mod aabb;
mod bvh;
mod camera;
mod common;
mod material;
mod math;
mod object;
mod ray;
mod texture;
mod vector;

fn random_balls() -> (Camera, BVHNode) {
    let mut world = ObjectCollection::new();

    let checkers =
        CheckerTexture::with_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new(checkers);

    world.add(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = random();
            let center = vec3(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            if material_choice < 0.8 {
                let albedo = Color::random();
                world.add(Sphere::new(center, 0.2, Lambertian::solid_color(albedo)))
            } else if material_choice < 0.95 {
                let albedo = Color::random();
                let fuzz = random_range(0.0..0.5);
                let material = Metal::solid_color(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, material))
            } else {
                let material = Dielectric::new(1.5);
                world.add(Sphere::new(center, 0.2, material))
            }
        }
    }

    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));

    world.add(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::solid_color(Color::new(0.4, 0.2, 0.1)),
    ));

    world.add(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        0.1,
        Light::solid_color(Color::new(0.7, 0.6, 0.5)), //Metal::solid_color(Color::new(0.7, 0.6, 0.5), 0.1),
    ));

    let world = BVHNode::new(world.objects());

    let vfov = 20.0;
    let look_from = vec3(13.0, 2.0, 3.0);
    let look_at = vec3(0.0, 0.0, 0.0);

    let camera = Camera::new(CameraConfig {
        image_width: 800,
        samples_per_pixel: 1024,
        defocus_angle: 0.6,
        vfov,
        look_from,
        look_at,
        ..Default::default()
    });

    (camera, world)
}

fn earth() -> (Camera, BVHNode) {
    let mut world = ObjectCollection::new();
    world.add(Sphere::new(
        vec3(0., 1.0, 0.0),
        1.0,
        Lambertian::new(ImageTexture::from_file("earthmap.jpg")),
    ));

    let world = BVHNode::new(world.objects());

    let vfov = 45.0;
    let look_from = vec3(3., 2.0, -1.0);
    let look_at = vec3(0.0, 1.0, 0.0);

    let camera = Camera::new(CameraConfig {
        image_width: 2560,
        samples_per_pixel: 2048,
        defocus_angle: 0.0,
        vfov,
        look_from,
        look_at,
        ..Default::default()
    });

    (camera, world)
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = env::args().nth(1).unwrap();
    let (camera, world) = match scene.as_str() {
        "balls" => random_balls(),
        "earth" => earth(),
        _ => panic!("Nonexistent scene"),
    };
    let image = camera.render(&world);

    image.save("image.png").unwrap();

    // Create a window with default options and display the image.
    let window = create_window("balls", Default::default()).unwrap();
    window.set_image("balls", image).unwrap();

    for event in window.event_channel().map_err(|e| e.to_string())? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if !event.is_synthetic
                && event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                println!("Escape pressed!");
                break;
            }
        }
    }

    Ok(())
}
