use anim::{easing, Options, Timeline};
use iced::advanced::graphics::core::event::Status;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, renderer};
use iced::widget::svg;
use iced::window::{self, RedrawRequest};
use iced::{mouse, BorderRadius, Event, Subscription};
use iced::{Color, Element, Length, Rectangle, Size};
use once_cell::sync::Lazy;
use std::time::{Duration, Instant};

pub static LOADER: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_memory(include_bytes!("../../../icons/loader.svg").to_vec()));

pub struct Circle {
    radius: f32,
    timeline: Timeline<f32>,
    start_time: Instant,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
}

impl Circle {
    pub fn new(radius: f32, start_time: Instant) -> Self {
        let mut timeline: Timeline<_> = Options::new(0.0, 45.0)
            .duration(Duration::from_millis(450))
            .easing(easing::linear())
            .cycle()
            .into();
        timeline.begin();

        Self {
            timeline,
            radius,
            start_time,
        }
    }

    pub fn update(&mut self, message: Message) -> Subscription<Message> {
        match message {
            Message::Tick => {
                self.timeline.update();
                Subscription::none()
            }
        }
    }
}

pub fn circle(radius: f32, start_time: Instant) -> Circle {
    Circle::new(radius, start_time)
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

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: iced::Event,
        _layout: advanced::Layout<'_>,
        _cursor: advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn advanced::Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> Status {
        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            shell.request_redraw(RedrawRequest::NextFrame);
        }

        Status::Ignored
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
        if (Instant::now() - self.start_time) < Duration::from_millis(500) {
            return;
        }

        let size = self.timeline.value();
        let bounds = layout.bounds();
        let circle_size = 3.;
        let white = iced::Color {
            r: 255.,
            g: 255.,
            b: 255.,
            a: 0.3,
        };

        let center_x = bounds.x + bounds.width / 2. - circle_size / 2.;
        let center_y = bounds.y + bounds.height / 2. - circle_size / 2.;

        // calculate coordinates for eight points around a circle
        let angles = [0., 45., 90., 135., 180., 225., 270., 315.];
        let radians = angles
            .iter()
            .map(|x| x + size)
            .map(|angle| angle * std::f32::consts::PI / 180.);

        let coordinates = radians
            .map(|radians| {
                (
                    center_x + bounds.width / 2. * radians.cos(),
                    center_y + bounds.width / 2. * radians.sin(),
                )
            })
            .collect::<Vec<_>>();

        // renderer.fill_quad(
        //     renderer::Quad {
        //         bounds,
        //         border_radius: BorderRadius::from(0.),
        //         border_width: 0.0,
        //         border_color: Color::TRANSPARENT,
        //     },
        //     iced::Color {
        //         r: 255.,
        //         g: 0.,
        //         b: 0.,
        //         a: 1.,
        //     },
        // );

        // create and render 8 quads at the calculated points
        for (x, y) in coordinates {
            let rectangle = Rectangle {
                width: circle_size,
                height: circle_size,
                x,
                y,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: rectangle,
                    border_radius: BorderRadius::from(50.),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                white,
            );
        }
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
