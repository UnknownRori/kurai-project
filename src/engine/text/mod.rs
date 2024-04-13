use macroquad::prelude::*;

// TODO : Make this thing much more flexible
pub fn draw_text_ex2(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    is_centered: bool,
    font: Option<&Font>,
) {
    let (font_size, font_scale, font_scale_aspect) = camera_font_scale(font_size);
    let measure = if is_centered {
        let len = measure_text(text, font, font_size, font_scale);
        (len.width / 2., 0.)
    } else {
        (0., 0.)
    };

    draw_text_ex(
        text,
        x - measure.0,
        y,
        TextParams {
            color,
            font,
            font_size,
            font_scale,
            font_scale_aspect,
            ..Default::default()
        },
    )
}
