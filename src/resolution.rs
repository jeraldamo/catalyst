
#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    pub x: usize,
    pub y: usize,
}

impl Resolution {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn res_vga() -> Self {
        Self {x: 640, y: 480}
    }

    pub fn res_xga() -> Self {
        Self {x: 1024, y: 768}
    }

    pub fn res_hd() -> Self {
        Self {x: 1280, y: 720}
    }

    pub fn res_full_hd() -> Self {
        Self {x: 1920, y: 1080}
    }

    pub fn res_4k() -> Self {
        Self {x: 3840, y: 2160}
    }

    pub fn aspect(&self) -> f32 {
        x / y
    }
}