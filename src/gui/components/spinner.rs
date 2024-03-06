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
        let bounds = layout.bounds();

        let circle_size = 3.;

        let center_x = bounds.x + bounds.width / 2. - circle_size / 2.;
        let center_y = bounds.y + bounds.height / 2. - circle_size / 2.;

        let button_1 = Rectangle {
            width: circle_size,
            height: circle_size,
            x: center_x,
            y: bounds.y,
        };

        let button_2 = Rectangle {
            width: circle_size,
            height: circle_size,
            x: bounds.x + bounds.width - circle_size,
            y: center_y,
        };

        let button_3 = Rectangle {
            width: circle_size,
            height: circle_size,
            x: center_x,
            y: bounds.y + bounds.height - circle_size,
        };

        let button_4 = Rectangle {
            width: circle_size,
            height: circle_size,
            x: bounds.x + bounds.width - circle_size,
            y: bounds.y + bounds.height - circle_size,
        };

        let white = iced::Color {
            r: 255.,
            g: 255.,
            b: 255.,
            a: 0.3,
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: button_1,
                border_radius: BorderRadius::from(50.),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            white,
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: button_2,
                border_radius: BorderRadius::from(50.),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            white,
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: button_3,
                border_radius: BorderRadius::from(50.),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            white,
        );

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
