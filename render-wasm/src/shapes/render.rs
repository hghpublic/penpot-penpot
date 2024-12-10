use skia_safe as skia;
use std::collections::HashMap;
use uuid::Uuid;

use super::{draw_image_in_container, Fill, Image, Kind, Shape};
use crate::math::Rect;
use crate::render::Renderable;

impl Renderable for Shape {
    fn blend_mode(&self) -> crate::render::BlendMode {
        self.blend_mode
    }

    fn opacity(&self) -> f32 {
        self.opacity
    }

    fn render(
        &self,
        surface: &mut skia_safe::Surface,
        images: &HashMap<Uuid, Image>,
    ) -> Result<(), String> {
        let mut transform = skia::Matrix::new_identity();
        let (translate_x, translate_y) = self.translation();
        let (scale_x, scale_y) = self.scale();
        let (skew_x, skew_y) = self.skew();
        transform.set_all(
            scale_x,
            skew_x,
            translate_x,
            skew_y,
            scale_y,
            translate_y,
            0.,
            0.,
            1.,
        );

        // Check transform-matrix code from common/src/app/common/geom/shapes/transforms.cljc
        let center = self.selrect.center();
        let mut matrix = skia::Matrix::new_identity();
        matrix.pre_translate(center);
        matrix.pre_concat(&transform);
        matrix.pre_translate(-center);

        surface.canvas().concat(&matrix);

        for fill in self.fills().rev() {
            render_fill(surface, images, fill, self.selrect, &self.kind);
        }

        let mut paint = skia::Paint::default();
        paint.set_blend_mode(self.blend_mode.into());
        paint.set_alpha_f(self.opacity);

        Ok(())
    }
}

fn render_fill(
    surface: &mut skia::Surface,
    images: &HashMap<Uuid, Image>,
    fill: &Fill,
    selrect: Rect,
    kind: &Kind,
) {
    match (fill, kind) {
        (Fill::Image(image_fill), kind) => {
            let image = images.get(&image_fill.id());
            if let Some(image) = image {
                draw_image_in_container(
                    surface.canvas(),
                    &image,
                    image_fill.size(),
                    kind,
                    &fill.to_paint(&selrect),
                );
            }
        }
        (_, Kind::Rect(rect)) => {
            surface.canvas().draw_rect(rect, &fill.to_paint(&selrect));
        }
        (_, Kind::Circle(rect)) => {
            surface.canvas().draw_oval(rect, &fill.to_paint(&selrect));
        }
        (_, Kind::Path(path)) => {
            surface
                .canvas()
                .draw_path(&path.to_skia_path(), &fill.to_paint(&selrect));
        }
    }
}
