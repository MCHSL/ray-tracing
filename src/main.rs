use std::rc::Rc;

use camera::{Camera, CameraConfig};
use glam::vec3;
use material::{Dielectric, Lambertian, Metal};
use object::{Sphere, World};
use ray::Color;
use show_image::{create_window, event};

use crate::math::{random, random_range};

mod camera;
mod material;
mod math;
mod object;
mod ray;
mod vector;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = World::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.add(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = random();
            let center = vec3(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            if material_choice < 0.8 {
                let albedo = Color::from(Color::random().0 * Color::random().0);
                let material = Lambertian::new(albedo);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                })
            } else if material_choice < 0.95 {
                let albedo = Color::random();
                let fuzz = random_range(0.0..0.5);
                let material = Metal::new(albedo, fuzz);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                })
            } else {
                let material = Dielectric::new(1.5);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                })
            }
        }
    }

    world.add(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric::new(1.5),
    });

    world.add(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    });

    world.add(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal::new(Color::new(0.7, 0.6, 0.5), 0.1),
    });

    let vfov = 20.0;
    let look_from = vec3(13.0, 2.0, 3.0);
    let look_at = vec3(0.0, 0.0, 0.0);

    let camera = Camera::new(CameraConfig {
        image_width: 800,
        samples_per_pixel: 50,
        defocus_angle: 0.6,
        vfov,
        look_from,
        look_at,
        ..Default::default()
    });
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
