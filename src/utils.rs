use rusttype::{point, Font, Scale};

pub fn measure_text(font: &Font, text: &str, scale: Scale) -> (u32, u32) {
    let v_metrics = font.v_metrics(scale);
    let ascent = v_metrics.ascent;
    let descent = v_metrics.descent;
    let height = (ascent - descent).ceil() as u32;

    let width = font
        .layout(text, scale, point(0.0, 0.0))
        .last()
        .map(|g| {
            let bb = g.pixel_bounding_box();
            if let Some(bb) = bb {
                bb.max.x as f32
            } else {
                g.position().x
            }
        })
        .unwrap_or(0.0)
        .ceil() as u32;

    (width, height)
}
