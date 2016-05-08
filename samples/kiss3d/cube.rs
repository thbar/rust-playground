extern crate kiss3d;
extern crate nalgebra as na;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

fn main() {
    let mut window = Window::new("Sample");
    
    let mut cubes: Vec<SceneNode> = vec![];
    for i in -1..2 {
        for j in -1..2 {
            for k in -1..2 {
                let mut c = window.add_cube(0.1, 0.1, 0.1);
                c.set_color(1.0 - (i as f32) * 0.3, 0.3 + (j as f32) * 0.3, 0.7 + (k as f32) * 0.2);
                c.append_translation(&Vector3::new(
                    i as f32 * 0.25,j as f32 * 0.25,k as f32 * 0.25));
                cubes.push(c);
            }
        }
    }

    window.set_light(Light::StickToCamera);

    while window.render() {
        for i in 0..3*3*3 {
            cubes[i].prepend_to_local_rotation(&Vector3::new(0.017, 0.012, 0.016));
        }
    }
}
