use iced::{button, Background, Button, Color, Column, Element, Sandbox, Settings, Text};

pub fn main() {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple counter - 计数")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment - 增加").color(Color {
                    r: 1.0, g: 1.0, b: 1.0, a: 1.0,
                }))
                    .on_press(Message::IncrementPressed)
                    .padding(12)
                    .border_radius(12)
                    .background(Background::Color(Color {
                        r: 0.11,
                        g: 0.42,
                        b: 0.87,
                        a: 1.0,
                    })),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement - 减少"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}