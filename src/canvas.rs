use std::collections::HashMap;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap};

#[derive(Debug,Clone)]
pub struct WebCanvas {
  pub element: HtmlCanvasElement,
  pub context: CanvasRenderingContext2d,
  pub bitmaps: HashMap<String,ImageBitmap>
}

impl WebCanvas {
  pub fn new(element: HtmlCanvasElement, context: CanvasRenderingContext2d) -> Self {
    let bitmaps = HashMap::new();
    let result = Self { element, context, bitmaps };
    result.reset();
    result
  }
  pub fn reset(&self) {
    self.element.set_width(self.element.offset_width() as u32);
    self.element.set_height(self.element.offset_height() as u32);
    self.context.set_image_smoothing_enabled(false);
  }
  pub fn get_bitmap(&self,src: &String) -> Option<&ImageBitmap> {
    self.bitmaps.get(src)
  }
}
