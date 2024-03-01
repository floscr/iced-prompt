// For now, to implement a custom native widget you will need to add
// `iced_native` and `iced_wgpu` to your dependencies.
//
// Then, you simply need to define your widget type and implement the
// `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
//
// Of course, you can choose to make the implementation renderer-agnostic,
// if you wish to, by creating your own `Renderer` trait, which could be
// implemented by `iced_wgpu` and other renderers.
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Color, Element, Length, Rectangle, Size};

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
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
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: self.radius.into(),
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
