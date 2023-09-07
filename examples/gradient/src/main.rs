use iced::widget::{column, container, row, slider, text};
use iced::{
    gradient, Alignment, Background, BorderRadius, Color, Element, Length,
    Radians, Sandbox, Settings,
};

pub fn main() -> iced::Result {
    Gradient::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
struct Gradient {
    start: Color,
    end: Color,
    angle: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartChanged(Color),
    EndChanged(Color),
    AngleChanged(f32),
}

impl Sandbox for Gradient {
    type Message = Message;

    fn new() -> Self {
        Self {
            start: Color::WHITE,
            end: Color::new(0.0, 0.0, 1.0, 1.0),
            angle: 0.0,
        }
    }

    fn title(&self) -> String {
        String::from("Gradient")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartChanged(color) => self.start = color,
            Message::EndChanged(color) => self.end = color,
            Message::AngleChanged(angle) => self.angle = angle,
        }
    }

    fn view(&self) -> Element<Message> {
        let Self { start, end, angle } = *self;

        let gradient_box = container(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(move |_: &_| {
                let gradient = gradient::Linear::new(Radians(
                    angle + std::f32::consts::PI,
                ))
                .add_stop(0.0, start)
                .add_stop(1.0, end)
                .into();

                container::Appearance {
                    text_color: None,
                    background: Some(Background::Gradient(gradient)),
                    border_radius: BorderRadius::default(),
                    border_width: 0.0,
                    border_color: Color::new(0.0, 0.0, 0.0, 0.0),
                }
            });

        let angle_picker = row![
            text("Angle").width(64),
            slider(0.0..=std::f32::consts::PI * 2.0, self.angle, move |v| {
                Message::AngleChanged(v)
            })
            .step(0.01)
        ]
        .spacing(8)
        .padding(8)
        .align_items(Alignment::Center);

        column![
            color_picker("Start", self.start).map(Message::StartChanged),
            color_picker("End", self.end).map(Message::EndChanged),
            angle_picker,
            gradient_box
        ]
        .into()
    }
}

fn color_picker(label: &str, color: Color) -> Element<'_, Color> {
    row![
        text(label).width(64),
        slider(0.0..=1.0, color.r, move |r| { Color { r, ..color } })
            .step(0.01),
        slider(0.0..=1.0, color.g, move |g| { Color { g, ..color } })
            .step(0.01),
        slider(0.0..=1.0, color.b, move |b| { Color { b, ..color } })
            .step(0.01),
    ]
    .spacing(8)
    .padding(8)
    .align_items(Alignment::Center)
    .into()
}
