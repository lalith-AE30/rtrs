#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ImageInfo {
    pub image_width: u32,
    pub image_height: u32,
}

#[allow(dead_code)]
impl ImageInfo {
    pub fn from_dim(image_width: u32, image_height: u32) -> Self {
        Self {
            image_width,
            image_height,
        }
    }

    pub fn from_aspect(image_height: u32, aspect_ratio: f64) -> Self {
        Self {
            image_height,
            image_width: (image_height as f64 * aspect_ratio) as u32,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.image_width as f64 / self.image_height as f64
    }
}

impl Default for ImageInfo {
    fn default() -> Self {
        Self {
            image_width: 400,
            image_height: 225,
        }
    }
}
