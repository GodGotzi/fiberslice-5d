use image::{GenericImageView, ImageError};

/// A texture typically contains one or more images that share the same format.
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    /// Creates a new texture from a byte array.
    ///
    /// Arguments:
    ///
    /// * `device`: The wgpu device for which the texture will be generated.
    /// * `queue`: The wgpu queue for which the texture will be generated.
    /// * `bytes`: The byte array containing the image / texture.
    /// * `label`: The label of the new texture.
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, ImageError> {
        let image = image::load_from_memory(bytes)?;
        Ok(Self::from_image(device, queue, &image, Some(label)))
    }

    /// Creates a new texture from a [image::DynamicImage].
    ///
    /// Arguments:
    ///
    /// * `device`: The wgpu device for which the texture will be generated.
    /// * `queue`: The wgpu queue for which the texture will be generated.
    /// * `image`: The source [image::DynamicImage] of the new texture.
    /// * `label`: The label of the new texture.
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let rgba = image.as_rgba8().unwrap();
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    /// Creates a new depth texture.
    ///
    /// Arguments:
    ///
    /// * `device`: The wgpu device for which the texture will be generated.
    /// * `config`: The wgpu surface configuration for which the texture will be generated.
    /// * `sample_count`: This has to be the same as the number of samples used for _MSAA_.
    /// * `label`: The label of the texture.
    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        sample_count: u32,
        label: &str,
    ) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, /*  | wgpu::TextureUsages::TEXTURE_BINDING */
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Creates a framebuffer that can be used for multisample anti-aliasing.
    ///
    /// Arguments:
    ///
    /// * `device`: The wgpu device for which the texture will be generated.
    /// * `config`: The wgpu surface configuration for which the texture will be generated.
    /// * `sample_count`: The sample count used for _MSAA_. Valid values are `1` (no MSAA) or `4`.
    /// * `label`: The label of the texture.
    pub fn create_multisampled_framebuffer(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        sample_count: u32,
        label: &str,
    ) -> wgpu::TextureView {
        let multisampled_texture_extent = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
            size: multisampled_texture_extent,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some(label),
            view_formats: &[],
        };

        device
            .create_texture(multisampled_frame_descriptor)
            .create_view(&wgpu::TextureViewDescriptor::default())
    }
}
