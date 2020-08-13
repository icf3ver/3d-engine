mod camera;
mod matrix4x4;
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
            self.camera.rotation.y = (1.0 + self.camera.rotation.y) % 360.0;

            println!("{}", self.camera.rotation.y);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.camera.rotation.y -= 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.position.z += 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.position.z -= 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.position.x += 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.position.x -= 1.0;
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let size: (f32, f32) = ggez::graphics::drawable_size(ctx);

        let mut mesh_transformed: Mesh = self.get_mesh_relative_camera();

        for tri in &mesh_transformed.tris{
            let look: Vec3d = Vec3d::new(
                (tri.verticies.0.x + tri.verticies.1.x + tri.verticies.2.x)/3.0,
                (tri.verticies.0.y + tri.verticies.1.y + tri.verticies.2.y)/3.0,
                (tri.verticies.0.z + tri.verticies.1.z + tri.verticies.2.z)/3.0);
            let look_len = f64::sqrt((look.x * look.x + look.y * look.y + look.z * look.z).into());
            let lx = (look.x as f64) / look_len;
            let ly = (look.y as f64) / look_len;
            let lz = (look.z as f64) / look_len;
            
            let dot_product: f64 = (tri.normal.x as f64 * lx) + (tri.normal.y as f64 * ly) + (tri.normal.z as f64 * lz);
            if dot_product < 0.0{
                let mut tri_projected: Triangle = Triangle::new(Vec3d{x: 0.0, y: 0.0, z: 0.0}, Vec3d{x: 0.0, y: 0.0, z: 0.0}, Vec3d{x: 0.0, y: 0.0, z: 0.0});

                tri_projected.verticies.0 = mesh_transformed.proj_matrix.multiply_vector(&tri.verticies.0);
                tri_projected.verticies.1 = mesh_transformed.proj_matrix.multiply_vector(&tri.verticies.1);
                tri_projected.verticies.2 = mesh_transformed.proj_matrix.multiply_vector(&tri.verticies.2);

                tri_projected.verticies.0.x += 1.0; tri_projected.verticies.0.y += 1.0;
                tri_projected.verticies.1.x += 1.0; tri_projected.verticies.1.y += 1.0;
                tri_projected.verticies.2.x += 1.0; tri_projected.verticies.2.y += 1.0;

                tri_projected.verticies.0.x *= 0.5 * size.0; 
                tri_projected.verticies.0.y *= 0.5 * size.1;
                tri_projected.verticies.1.x *= 0.5 * size.0;
                tri_projected.verticies.1.y *= 0.5 * size.1;
                tri_projected.verticies.2.x *= 0.5 * size.0;
                tri_projected.verticies.2.y *= 0.5 * size.1;

                let triangle = graphics::Mesh::from_triangles (
                    ctx,
                    &tri_projected.form_pointlist(),
                    graphics::Color::new(look.x as f32, look.y as f32, look.z as f32, 1.0), //crude fix replace 1.0 <- 0.5 to fix layering error
                )?;
                graphics::draw(ctx, &triangle, (na::Point2::new(0.0, 0.0),))?;
            }
            
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

// Main
pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("my-engine", "littleTitan");
    let (ctx, event_loop) = &mut cb.build()?;
    let camera: Camera = Camera{position: Vec3d::new(0.0, 0.0, -4.0), rotation: Vec3d::new(0.0, 0.0, 0.0)};
    let state = &mut Mesh::new(ctx, camera)?;
    ggez::graphics::set_window_title(ctx, "My Engine");
    state.from_file("models/xyz.stl");
    event::run(ctx, event_loop, state)
}