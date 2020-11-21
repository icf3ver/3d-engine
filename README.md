# My 3D Engine

A 3D engine made with ggez in rust.

WARNING: This program may potentially trigger seizures for people with photosensitive epilepsy. Viewer discretion is advised.

# v0.1.0
Basic Working algorithm for rendering 3d models.

# v0.2.0
Implements Multithreading
Better projection Algorithm
Crop triangles to fit screen
Better comments.

# v0.3.0
changed movement more intuitive.
Since on the 2d screen plane y is reversed I now compensate for that.
Better render culling

## Bugs targetted in this version
 - Big triangles glitch.
 - Clipping stack overflow error.
![overflow](img/clipping_err.png)

# Todo
 ## Bug Fix
 - Fix diagonal turning mirror error
 - Fix the big triangles glitch
 - Clipping stack overflow error
![overflow](img/clipping_err.png)
 - <!> Clean lighting <!>
 ## UI
  + Export Stl and Obj
  + Object manipulation
   - Rotation
   - Movement
   - Scale
  + Vertex manipulation
  + divide sides
  + background manipulation
  + Lock look
  + vertex mode
  + transparent mode