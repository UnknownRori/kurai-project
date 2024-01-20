use crate::window::Window;

pub trait Drawable {
    fn draw(window: &Window) -> impl std::future::Future<Output = ()> + Send;
}
