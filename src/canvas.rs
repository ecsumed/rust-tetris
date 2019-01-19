use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    width: i32,
    height: i32,
	cell_size: i32,
}

impl Canvas {

    pub fn new(attr_id: &str, width: i32, height: i32, cell_size: i32) -> Canvas {
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
            cell_size,
        }
    }

    pub fn draw_grid(&self, color: &str) {

      self.ctx.begin_path();
      self.canvas.set_width(((self.cell_size + 1) * self.width + 1) as u32); 
      self.canvas.set_height(((self.cell_size + 1) * self.height + 1) as u32);

      // Vertical lines.
      for i in 0..self.width + 1 {
        let new_x = (i as i32 * (self.cell_size + 1) + 1) as f64;
        self.ctx.move_to(new_x, 0_f64);

        let new_y = ((self.cell_size + 1) * self.height as i32 + 1) as f64;
        self.ctx.line_to(new_x, new_y);
      }

      // Horizontal lines.
      for j in 0..self.height + 1 {
        let new_y = (j as i32 * (self.cell_size + 1) + 1) as f64;
        self.ctx.move_to(0_f64, new_y);


        let new_x = ((self.cell_size + 1) * self.width as i32 + 1) as f64;
        self.ctx.line_to(new_x, new_y);
      }

      self.ctx.set_stroke_style_color(color);
      self.ctx.stroke();
    }

    pub fn draw_block(&self, x: u32, y: u32, color: &str) {
        self.ctx.set_fill_style_color(color);

        self.ctx.fill_rect(
            f64::from(x as i32 * (self.cell_size + 1) + 1),
            f64::from(y as i32 * (self.cell_size + 1) + 1),
            f64::from(self.cell_size),
            f64::from(self.cell_size),
        );
	}

	pub fn clear(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
    }
}
