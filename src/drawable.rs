use crate::window::Window;

pub trait Drawable {
    async fn draw(window: &Window);
}
