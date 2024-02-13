use num_traits::*;

use macroquad::{
    math::{vec2, Vec2},
    ui::{hash, root_ui, widgets::*, Id, Ui},
};

type F = Box<dyn FnOnce(&mut Ui) + 'static>;
pub struct TabMenu(String, Option<F>);

pub struct TabbedWidget {
    id: Id,
    tabs: Vec<TabMenu>,
    name: String,
    location: Vec2,
    size: Vec2,
}

impl TabbedWidget {
    /// INFO : Improve this silly thing so it can be flexible later
    pub fn draw(&mut self) {
        macroquad::ui::widgets::Window::new(self.id, self.location, self.size)
            .label(&self.name)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                let tabs_name = self
                    .tabs
                    .iter()
                    .map(|tab| tab.0.as_ref())
                    .collect::<Vec<_>>();

                let tab_widget_selected =
                    Tabbar::new(hash!(), vec2(self.size.x - 8., 12.), &tabs_name).ui(ui);
                match self.tabs.get_mut(
                    tab_widget_selected
                        .to_usize()
                        .expect("Failed to convert u32 to usize"),
                ) {
                    Some(a) => {
                        if let Some(func) = a.1.take() {
                            func(ui);
                        }
                    }
                    None => tracing::info!("Invalid tab index! : {}", &self.name),
                }
            });
    }
}

pub struct TabbedWidgetBuilder {
    id: Id,
    location: Vec2,
    size: Vec2,
    name: String,
    tabs: Vec<TabMenu>,
}

impl TabbedWidgetBuilder {
    #[inline]
    #[must_use]
    pub fn new(id: Id, name: String, size: Vec2) -> Self {
        Self {
            id,
            size,
            name,
            location: vec2(0., 0.),
            tabs: vec![],
        }
    }

    pub fn location(mut self, position: Vec2) -> Self {
        self.location = position;
        self
    }

    #[inline]
    #[must_use]
    pub fn add(mut self, name: String, ui: impl FnOnce(&mut Ui) + 'static) -> Self {
        self.tabs.push(TabMenu(name, Some(Box::new(ui))));
        self
    }

    pub fn build(self) -> TabbedWidget {
        TabbedWidget {
            id: self.id,
            name: self.name,
            location: self.location,
            size: self.size,
            tabs: self.tabs,
        }
    }
}
