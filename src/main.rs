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
use ggez::timer::delta;

use camera::Camera;
use vec3d::Vec3d;
use mesh::Mesh;

/// Event Handler for a Mesh
/// 
/// NOTE: This structure/Object hierarchy is subject to change
impl event::EventHandler for Mesh {
    /// Game Loop
    /// <p>
    /// User Controls
    ///  - UP_ARROW     rotatate around player local x axis 
    ///                 -> look up
    ///  - DOWN_ARROW   rotatate around player local x axis 
    ///                 -> look down
    ///  - RIGHT_ARROW  rotatate around global y axis 
    ///                 -> look right
    ///  - LEFT_ARROW   rotatate around global y axis 
    ///                 -> look left
    /// 
    ///  - W_KEY        Move Forward
    ///  - S_KEY        Move Backward
    ///  - D_KEY        Move Right
    ///  - A_KEY        Move Left
    /// 
    /// NOTE: Controls are subject to change
    /// 
    /// # Arguments
    /// * `self` - The Mesh being updated
    /// * `ctx` - GGez's Context
    /// 
    /// # Return
    /// A GameResult
    /// 
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let time_factor = 1.0/100000000.0 * delta(ctx).as_nanos() as f32;
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.camera.rotation.x += time_factor * 1.5;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.camera.rotation.x -= time_factor * 1.5;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.camera.rotation.y -= time_factor * 1.5;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.camera.rotation.y += time_factor * 1.5;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.position = self.camera.position + (self.camera.forward * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.position = self.camera.position - (self.camera.forward * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.position = self.camera.position - (self.camera.right * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.position = self.camera.position + (self.camera.right * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        
        Ok(())
    }

    /// Project the mesh and render it.
    /// 
    /// # Arguments
    /// * `self` - The Mesh being updated
    /// * `ctx` - GGez's Context
    /// 
    /// # Return
    /// A GameResult
    /// 
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into()); // clear

        let size: (f32, f32) = ggez::graphics::drawable_size(ctx);
        
        // Get the projected triangles.
        let tris = self.camera.get_projected_triangles(self.clone(), size);

        // Draw all the triangles
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

/// Main
///  - Generates a Context and an event loop
///  - Creates a Camera with a position and a rotaion
///  - Creates a Mesh with the Camera
///  - Loads in a file or the cube 
///  - Creates the window 
///  - Starts the eventloop for mesh
/// 
/// # Return
/// A GameResult
/// 
pub fn main() -> ggez::GameResult {
    // Create Window
    let cb = ggez::ContextBuilder::new("my-engine", "littleTitan");
    let (ctx, event_loop) = &mut cb.build()?;

    // Create Camera
    let camera: Camera = Camera::new(Vec3d::new(0.0, 0.0, -4.0), Vec3d::new(0.0, 0.0, 0.0));

    // Create Mesh
    let mesh = &mut Mesh::new(camera)?;
    mesh.from_file("models/xyz.stl");

    // Give Context and Mesh to GGez
    ggez::graphics::set_window_title(ctx, "My Engine");
    event::run(ctx, event_loop, mesh)
}