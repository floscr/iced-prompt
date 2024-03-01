use anim::{easing, Options, Timeline};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::widget::svg;
use iced::{mouse, Subscription};
use iced::{Color, Element, Length, Rectangle, Size};
use once_cell::sync::Lazy;
use std::time::{Duration, Instant};

pub static LOADER: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_memory(include_bytes!("../../../icons/loader.svg").to_vec()));

pub struct Circle {
    radius: f32,
    timeline: Timeline<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick(Instant),
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        let mut timeline: Timeline<_> = Options::new(0.0, 3.0)
            .duration(Duration::from_secs(2))
            .easing(easing::sine_ease())
            .into();
        timeline.begin();

        Self { timeline, radius }
    }

    pub fn update(&mut self, message: Message) -> Subscription<Message> {
        match message {
            Message::Tick(_) => {
                self.timeline.update();
                Subscription::none()
            }
        }
    }
}

pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}

impl<Message, Renderer> Widget<Message, Renderer> for Circle
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let size = self.timeline.value();

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: size.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Renderer> From<Circle> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(circle: Circle) -> Self {
        Self::new(circle)
    }
}
