use std::collections::HashMap;

use butterfly_soul_engine::{
  modules::context::{
    control::{Control, KeyEvent},
    render::{RenderFrame, Texture},
  },
  utils::{rect::Rect, vector::Vector, viewbox::ViewBox},
};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{HtmlCanvasElement, HtmlElement};

use crate::canvas::WebCanvas;

pub use butterfly_soul_engine::modules::context::Context;
pub mod canvas;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = butterflySoulEngine)]
  fn mount_control(el: &HtmlElement);
  #[wasm_bindgen(js_namespace = butterflySoulEngine)]
  fn control() -> JsValue;
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WebContext {
  canvas: Option<WebCanvas>,
  control: Option<()>,
}

#[wasm_bindgen]
impl WebContext {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Function::new_no_args(include_str!("glue.js"))
      .call0(&JsValue::undefined())
      .unwrap();

    WebContext {
      canvas: None,
      control: None,
    }
  }
  pub fn mount(&mut self, el: HtmlCanvasElement) -> Result<(), JsValue> {
    self.set_control(&el)?;
    self.set_canvas(&el)?;
    Ok(())
  }
  fn set_canvas(&mut self, el: &HtmlCanvasElement) -> Result<(), JsValue> {
    let ctx = el
      .get_context("2d")?
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let canvas = WebCanvas::new(el.clone(), ctx);
    self.canvas = Some(canvas);
    Ok(())
  }

  fn set_control(&mut self, el: &HtmlCanvasElement) -> Result<(), JsValue> {
    self.control = Some(());
    mount_control(&el);
    Ok(())
  }
}

impl Context for WebContext {
  fn render(&self, frame: RenderFrame) -> Option<()> {
    let canvas = self.canvas.clone()?;
    let ctx = canvas.context.clone();
    let el = canvas.element.clone();
    let canvas_size = Vector::new(el.width() as f32, el.height() as f32);

    // 重置成視口坐標系
    // x: +-viewport.width * 0.5 (>)
    // y: +-viewport.height * 0.5 (^)
    // 原點設在中心點
    // 偏移為視口位置
    {
      let viewport = frame.viewport();
      let scale = canvas_size / viewport.size();
      let (_sw, sh) = scale.unpack();
      let (ox, oy) =
        ((canvas_size / 2.) - viewport.position() * sh /* scale */ * Vector::new(1.,-1.)).unpack();
      ctx
        .set_transform(sh as f64, 0., 0., -sh as f64, ox as f64, oy as f64)
        .unwrap();
      // ctx
      //   .set_transform(sw as f64, 0., 0., -sh as f64, ox as f64, oy as f64)
      //   .unwrap();
      //draw
      draw(frame.get(), &canvas);
    };

    // 重置成單位坐標系
    // x: +-1.0 (>)
    // y: +-1.0 (^)
    // 原點設在中心點
    {
      let (cw, ch) = canvas_size.unpack();
      ctx
        .set_transform(
          ch as f64,
          0.,
          0.,
          -ch as f64,
          (cw / 2.) as f64,
          (ch / 2.) as f64,
        )
        // .set_transform(
        //   cw as f64,
        //   0.,
        //   0.,
        //   -ch as f64,
        //   (cw / 2.) as f64,
        //   (ch / 2.) as f64,
        // )
        .unwrap();
      //draw
      draw(frame.get_ui(), &canvas);
    };

    //
    //
    //
    //
    fn draw(list: &Vec<(Rect, Texture)>, canvas: &WebCanvas) {
      let ctx = canvas.context.clone();

      for (rect, texture) in list.iter() {
        ctx.save();

        //move to
        {
          let (x, y) = rect.position.unpack();
          ctx.translate(x as f64, y as f64).unwrap();
          ctx.rotate(rect.angle as f64).unwrap();
        }

        //draw
        {
          let (w, h) = rect.size.unpack();
          let (x, y, w, h) = ((w / -2.) as f64, (h / -2.) as f64, w as f64, h as f64);
          match texture {
            Texture::Color(color) => {
              ctx.set_fill_style(&JsValue::from(color));
              ctx.fill_rect(x, y, w, h);
            }
            Texture::Bitmap(bitmap) => {
              let bitmap = canvas.get_bitmap(bitmap).unwrap();
              ctx
                .draw_image_with_image_bitmap_and_dw_and_dh(bitmap, x, y, w, h)
                .unwrap();
            }
          }
        }

        ctx.restore();
      }
    }

    Some(())
  }
  fn control(&self) -> Option<Control> {
    if self.control.is_none() {
      return None;
    };
    let value: WebControl = serde_wasm_bindgen::from_value(control()).unwrap();
    let keys = value
      .keys
      .iter()
      .map(|(key, info)| KeyEvent {
        code: key.clone(),
        alt: info.alt,
        ctrl: info.ctrl,
        meta: info.meta,
        shift: info.shift,
        repeat: info.repeat,
      })
      .collect();

    Some(Control {
      keys,
      click: value.click,
      mouse: value.mouse,
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebControl {
  pub keys: HashMap<String, WebKeyEvent>,
  pub click: [Option<Vector>; 2],
  pub mouse: Vector,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebKeyEvent {
  pub alt: bool,
  pub ctrl: bool,
  pub meta: bool,
  pub shift: bool,
  pub repeat: bool,
}
