use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl Canvas {

    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        Canvas {
            canvas,
            ctx,
            width,
            height,
        }
    }

    pub fn draw_grid(&self, cell_size: i32, color: &str) {

      self.ctx.begin_path();
      self.canvas.set_width((cell_size as u32 + 1) * self.width + 1); 
      self.canvas.set_height((cell_size as u32 + 1) * self.height + 1);

      // Vertical lines.
      for i in 0..self.width + 1 {
        let new_x = (i as i32 * (cell_size + 1) + 1) as f64;
        self.ctx.move_to(new_x, 0_f64);

        let new_y = ((cell_size + 1) * self.height as i32 + 1) as f64;
        self.ctx.line_to(new_x, new_y);
      }

      // Horizontal lines.
      for j in 0..self.height + 1 {
        let new_y = (j as i32 * (cell_size + 1) + 1) as f64;
        self.ctx.move_to(0_f64, new_y);


        let new_x = ((cell_size + 1) * self.width as i32 + 1) as f64;
        self.ctx.line_to(new_x, new_y);
      }

      self.ctx.set_stroke_style_color(color);
      self.ctx.stroke();
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
    }
}
