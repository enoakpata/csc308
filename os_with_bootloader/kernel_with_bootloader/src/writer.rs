mod constants;
use core::{
    fmt::{self, Write},
    ptr,
};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    scroll_offset: usize,
    
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            scroll_offset: 0,  // New field
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_pos += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
        if self.y_pos >= self.height() {
            self.scroll();
        }
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.scroll_offset = 0;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            '\t' => {
                for _ in 0..4 { self.write_char(' '); }
            }
            c => {
                if self.x_pos + font_constants::CHAR_RASTER_WIDTH >= self.width() {
                    self.newline();
                }
                if self.y_pos + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING >= self.height() {
                    self.scroll();
                }
                self.write_rendered_char(get_char_raster(c));
                self.x_pos += font_constants::CHAR_RASTER_WIDTH;
    
                if self.x_pos + font_constants::CHAR_RASTER_WIDTH >= self.width() {
                    self.newline();
                }
            }
        }
    }

    fn scroll(&mut self) {
        let char_height = CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        let stride_bytes = self.info.stride * self.info.bytes_per_pixel;
        let scroll_bytes = char_height * stride_bytes;
        let height = self.height();
    
        // Move screen contents up by one row height
        self.framebuffer.copy_within(scroll_bytes.., 0);
    
        // Clear the last row by zeroing out the new empty space
        let clear_start = (height - char_height) * stride_bytes;
        self.framebuffer[clear_start..].fill(0);
    
        // Adjust cursor position
        self.y_pos -= char_height;
    }

    fn scroll_up(&mut self) {
        let char_height = CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        if self.scroll_offset >= char_height {
            self.scroll_offset -= char_height;
            self.redraw();
        }
    }
    
    fn scroll_down(&mut self) {
        let char_height = CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        if self.scroll_offset + self.height() < self.info.height {
            self.scroll_offset += char_height;
            self.redraw();
        }
    }
    

    fn redraw(&mut self) {
        let stride_bytes = self.info.stride * self.info.bytes_per_pixel;
        let start_offset = self.scroll_offset * stride_bytes;
        let visible_height = self.height();
        let total_height = self.info.height;
    
        if start_offset + (visible_height * stride_bytes) <= self.framebuffer.len() {
            self.framebuffer.copy_within(start_offset..(start_offset + visible_height * stride_bytes), 0);
        }
    }
    

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other);
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)].copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}