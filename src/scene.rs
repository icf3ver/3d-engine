use ggez::GameResult;

use crate::{Camera, Mesh};

/// The Scene to be rendered.
pub struct Scene {
    pub camera: Camera,
    pub mesh_vec: Vec<Mesh>,
}

impl Scene {
    /// Creates a new Mesh with a camera
    ///
    /// # Arguments
    /// * `camera` - The Camera.
    /// * `mesh_vec` - A vec of all the meshes to render.
    ///
    /// # Return
    /// A GameRusult<Scene> object
    ///
    pub fn new(camera: Camera, mesh_vec: Vec<Mesh>) -> GameResult<Scene> {
        Ok(Scene {
            camera: camera,
            mesh_vec: mesh_vec,
        })
    }
}
