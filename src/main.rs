mod camera;
mod matrix3x3;
mod vec3d;
mod triangle;
mod mesh;

use ggez;
use ggez::{graphics, event};
use ggez::nalgebra as na;
use ggez::event::KeyCode;
use ggez::input::keyboard;

use camera::Camera;
use vec3d::Vec3d; 
use triangle::Triangle;
use mesh::Mesh;

use rand::prelude::*;

// Event handlers
impl event::EventHandler for Mesh {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.camera.rotation.x += 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.camera.rotation.x -= 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.camera.rotation.y += 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.camera.rotation.y -= 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.position.z += 1.0; // self.camera.position + self.camera.forward;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.position.z -= 1.0; //  = self.camera.position - self.camera.forward;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.position.x += 1.0; //  = self.camera.position + self.camera.right;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.position.x -= 1.0; //  = self.camera.position - self.camera.right;
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let size: (f32, f32) = ggez::graphics::drawable_size(ctx);

        let mut tris: Vec<Triangle> = Vec::new();

        tris = self.camera.get_projected_triangles(&self.clone(), size);

        for mut tri in tris {
            let triangle = graphics::Mesh::from_triangles (
                ctx,
                &tri.form_pointlist(),
                graphics::Color::new(tri.color.0, tri.color.1, tri.color.2, 1.0),
            )?;
            graphics::draw(ctx, &triangle, (na::Point2::new(0.0, 0.0),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

// Main
pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("my-engine", "littleTitan");
    let (ctx, event_loop) = &mut cb.build()?;
    let mut camera: Camera = Camera::new(Vec3d::new(0.0, 0.0, -6.0), Vec3d::new(0.0, 0.0, 0.0), 10000.0, 0.1, 130.0, ctx);
    //camera.forward.y_axis_rotation(45.0, camera.position.x, camera.position.z);
    let state = &mut Mesh::new(ctx, camera)?;
    ggez::graphics::set_window_title(ctx, "My Engine");
    //state.form_cube();
    state.from_file("models/xyz.stl");
    event::run(ctx, event_loop, state)
}