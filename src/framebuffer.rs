use crate::image::*;

/// Heap-Allocated Framebuffer.
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub color: Vec<u8>,
}

impl FrameBuffer {
    /// Creates a new framebuffer. Normally created inside a Rasterizer during its initialization or when it is resized
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            width,
            height,
            color: vec![0; width * height * 4],
        }
    }

	/// Takes the framebuffer and saves it to an image. Good for one time captures.
    pub fn to_image(&self) -> Image {
        Image {
            buffer: self.color.clone(),
            width: self.width,
            height: self.height,
        }
    }

	/// Copies the framebuffer into an image buffer if they are the same size. Good for dynamic images that change frequently.
    pub fn to_image_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.clear();
        if self.color.len() == buffer.len() {
            buffer.copy_from_slice(self.color.as_slice());
        }
    }



    /// Copies framebuffer directly into another one, with an optional offset.
    pub fn blit_framebuffer(&mut self, fbuf_blit: &FrameBuffer, offset_x: usize, offset_y: usize) {
        let stride = 4;
        // We blit these directly into the color buffer because otherwise we'd just be drawing everything over again and we don't have to worry about depth
        
        // The color array is a 1D row of bytes, so we have to do this in sets of rows
        // Make sure this actually fits inside the buffer
        let extent_width: usize = offset_x + fbuf_blit.width;
        let extent_height: usize = offset_y + fbuf_blit.height;
    
        let src_height: usize = fbuf_blit.height;
        let dst_height: usize = self.height;
    
        // If this goes out of bounds at all we should not draw it. Otherwise it WILL panic.
        let not_too_big: bool = self.width * self.height < fbuf_blit.width * self.height;
        let not_out_of_bounds: bool = extent_width > self.width || extent_height > self.height;
        if not_too_big && not_out_of_bounds { 
            println!("ERROR - FRAMEBUFFER BLIT: Does not fit inside target buffer!"); 
            return;
        }
    
        // Lets get an array of rows so we can blit them directly into the color buffer
        let mut rows_src: Vec<&[u8]> = Vec::new();
    
        // Build a list of rows to blit to the screen.
        fbuf_blit.color.chunks_exact(fbuf_blit.width * stride).enumerate().for_each(|(_, row)| {
            rows_src.push(row);
        });
    
        let is_equal_size: bool = self.width == fbuf_blit.width && self.height == fbuf_blit.height;
    
        // Goes through each row of fbuf and split it twice into the slice that fits our rows_src. So we 
        self.color.chunks_exact_mut(self.width * stride).enumerate().for_each(|(i, row_dst)| {
            if i >= dst_height { return; }
            if i >= offset_y && i < offset_y + src_height { 
                if is_equal_size {
                    row_dst.copy_from_slice(rows_src[i]);
                } else {
                    // We need to cut the row into a section that we can just set equal to our row
                    // Make sure that we are actually in the bounds from our source buffer
                    if i >= offset_y && i < (offset_y + rows_src.len()) {
                        // [......|#######]
                        // Split at the stride distance to get the first end
                        let rightsect = row_dst.split_at_mut(offset_x * stride).1;
        
                        // [......|####|...]
                        // Get the second half but left
                        let section = rightsect.split_at_mut((extent_width - offset_x) * stride).0;
        
                        // I HAVE YOU NOW
                        section.copy_from_slice(rows_src[i-offset_y]);
                    }
                }
            }
        });
    }
}