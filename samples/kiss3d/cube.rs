/* sample taken from kiss3d, which I'm just trying to compile myself */
extern crate kiss3d;
extern crate nalgebra as na;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c      = window.add_cube(0.1, 0.1, 0.1);

    c.set_color(1.0, 0.3, 0.9);

    window.set_light(Light::StickToCamera);

    while window.render() {
        c.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.006));
    }
}
