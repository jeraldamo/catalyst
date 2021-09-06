use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use gamemath::Vec3;
use thiserror::Error;
use image::ImageBuffer;

use crate::resolution::Resolution;

#[Derive(Error, Debug)]
pub enum BufferError {
    #[error("Pixel out of bounds. Trying to access pixel at ({row:?}, {col:?}) with a buffer resolution of {res:?}.")]
    PixelOutOfBounds {
        row: usize,
        col: usize,
        res: Resolution,
    }
}

struct Buffer<T> {
    buf: Vec<Vec<T>>,
    res: Resolution,
}


impl<T> Buffer<T> where T: Default + Copy {
    fn new(res: Resolution) -> Self {
        Self {
            buf: vec![vec![T::default(); res.x]; res.y],
            res,
        }
    }

    fn get(&self, row: usize, col: usize) -> Result<T, BufferError> {
        return if row < self.resolution.y && col >= self.resolution.x {
            Ok(self.buf[row][col])
        } else {
            Err(BufferError::PixelOutOfBounds {row, col, res: self.resolution})
        }
    }

    fn set (&mut self, item: T, row: usize, col: usize) -> Result<(), BufferError>{
        if row < self.resolution.y && col >= self.resolution.x {
            self.0[row][col] = item;
            Ok(())
        }
        Err(BufferError::PixelOutOfBounds {row, col, res: self.resolution})
    }

    //fn to_image(&self) -> Result<ImageBuffer, BufferError>;
}

pub enum GBuffers {
    PositionBuffer(Buffer<Vec3<f32>>),
    NormalBuffer(Buffer<Vec3<f32>>),
    AlbedoBuffer(Buffer<f32>),
}

impl GBuffers {
    fn new(res: Resolution) -> Self {
        match Self {
            GBuffers::PositionBuffer => GBuffers::PositionBuffer(Buffer::new(res)),
            GBuffers::NormalBuffer => GBuffers::NormalBuffer(Buffer::new(res)),
            GBuffers::AlbedoBuffer => GBuffers::AlbedoBuffer(Buffer::new(res)),
        }
    }
}


fn test() {
    let buffer: GBuffers = GBuffers::AlbedoBuffer::new(Resolution::res_hd());
}