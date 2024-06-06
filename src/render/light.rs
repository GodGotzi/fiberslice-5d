/// The light data is used to compute the scenes lighting in the shader.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    /// The position of the light within the scene in homogenous coordinates.
    ///
    /// Homogenous coordinates are used to fullfill the 16 byte alignment requirement.
    pub position: [f32; 4],

    /// The color of the light.
    ///
    /// The format is RGB (`[1.0, 1.0, 1.0]` is fully white) and the last item controls the
    /// strength of the ambient light.
    pub color: [f32; 4],
}
