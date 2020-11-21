mod camera;
mod matrix3x3;
mod mesh;
mod scene;
mod triangle;
mod vec3d;

use camera::Camera;
use mesh::Mesh;
use scene::Scene;
use vec3d::Vec3d;

use ggez::{
    self, 
    event::KeyCode, 
    graphics::Vertex, 
    event, 
    graphics, 
    input::keyboard, 
    nalgebra as na, 
    timer::delta
};

/// Event Handler for a Mesh
///
/// NOTE: This structure/Object hierarchy is subject to change
impl event::EventHandler for Scene {
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
    /// NOTE: Controls may be subject to change
    ///
    /// # Arguments
    /// * `self` - The Mesh being updated
    /// * `ctx` - GGez's Context
    ///
    /// # Return
    /// A GameResult
    ///
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let time_factor = 1.0 / 100000000.0 * delta(ctx).as_nanos() as f32;

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.camera.rotate_up(time_factor * 4.5);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.camera.rotate_down(time_factor * 4.5);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.camera.rotate_right(time_factor * 4.5);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.camera.rotate_left(time_factor * 4.5);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.camera.position = self.camera.position
                + (self.camera.m_forward * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.camera.position = self.camera.position
                - (self.camera.m_forward * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.camera.position = self.camera.position
                - (self.camera.m_right * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.camera.position = self.camera.position
                + (self.camera.m_right * Vec3d::new(-1.0, -1.0, 1.0)).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.camera.position =
                self.camera.position + Vec3d::new(0.0, 1.0, 0.0).set_length(time_factor);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::LShift)
            || keyboard::is_key_pressed(ctx, KeyCode::LShift)
        {
            self.camera.position =
                self.camera.position - Vec3d::new(0.0, 1.0, 0.0).set_length(time_factor);
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

        for i in 0..self.mesh_vec.len() {
            self.mesh_vec[i].is_over = false;
            // Get the projected triangles.
            let tris = self
                .camera
                .get_projected_triangles(self.mesh_vec[i].clone(), size);

            let mut raw: Vec<Vertex> = vec![];
            // Draw all the triangles
            for j in 0..tris.len() {
                let mut tri = tris[j];
                let pt_list = tri.form_vertexlist();
                raw.push(pt_list[0]);
                raw.push(pt_list[1]);
                raw.push(pt_list[2]);

                let triangle = graphics::Mesh::from_triangles(
                    ctx,
                    &tri.form_pointlist(),
                    graphics::Color::new(tri.color.0, tri.color.1, tri.color.2, 1.0),
                )?;
                graphics::draw(ctx, &triangle, (na::Point2::new(0.0, 0.0),))?;
            }
            // let triangle = graphics::Mesh::from_raw (
            //     ctx,
            //     &raw,
            //     &vec![],
            //     None,
            // )?;
            // graphics::draw(ctx, &triangle, (na::Point2::new(0.0, 0.0),))?;
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
fn main() -> ggez::GameResult {
    // Create Window
    let cb = ggez::ContextBuilder::new("my-engine", "littleTitan");
    let (ctx, event_loop) = &mut cb.build()?;

    // Create Camera
    let camera: Camera = Camera::new(Vec3d::new(0.0, 0.0, -4.0), Vec3d::new(0.0, 0.0, 0.0));

    // Create Mesh
    let mut mesh = Mesh::new(Vec3d::new(0.0, 0.0, 0.0));
    mesh.from_file("m.stl"); //note do not attempt to use too big files yet

    let mut mesh1 = Mesh::new(Vec3d::new(0.0, 0.0, 0.0));
    mesh1.form_cube();

    let mut mesh2 = Mesh::new(Vec3d::new(0.0, 2.0, 0.0));
    mesh2.form_cube();

    // Create Scene
    let scene = &mut Scene::new(camera, vec![mesh])?;

    // Give Context and Mesh to GGez
    ggez::graphics::set_window_title(ctx, "My Engine");
    event::run(ctx, event_loop, scene)
}
