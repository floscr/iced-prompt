use anim::{easing, Options, Timeline};
use iced::advanced::layout::{self, Layout, Node};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{renderer, Clipboard, Shell};
use iced::{event, mouse, time, Event, Point, Subscription};
use iced::{Color, Element, Length, Rectangle, Size};
use std::time::{Duration, Instant};

pub struct Circle {
    radius: f32,
    start_time: Instant,
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

        Self {
            timeline,
            radius,
            start_time: Instant::now(),
        }
    }

    pub fn update(&mut self, message: Message) -> Subscription<Message> {
        match message {
            Message::Tick(now) => {
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

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            _ => event::Status::Ignored,
        }
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

// use iced::time;

// #[derive(Default)]
// pub struct Loading {
//     start_time: Instant,
//     is_visible: bool,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     Tick(Instant),
// }

// impl iced::widget::Component for Loading {
//     type Message = Message;
//     type Executor = iced::executor::Default;

//     fn update(&mut self, message: Self::Message) -> Command<Message> {
//         match message {
//             Message::Tick(now) => {
//                 if now.duration_since(self.start_time) >= Duration::from_secs(1) {
//                     self.is_visible = true;
//                 }

//                 Command::none()
//             }
//         }
//     }

//     fn view(&self) -> Element<Message> {
//         if self.is_visible {
//             Text::new("Loading").into()
//         } else {
//             Element::empty()
//         }
//     }

//     fn subscription(&self) -> Subscription<Message> {
//         let timer = time::every(Duration::from_secs(1)).map(Message::Tick);

//         Subscription::from_recipe(timer)
//     }
// }
