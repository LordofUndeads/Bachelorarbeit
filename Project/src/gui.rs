struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed
}

use iced::{Alignment, Sandbox, Settings, Element, Button, Column, Text};

impl Sandbox for Counter{
    type Message = Message;

    fn new() -> Self {
        Self { value: 0}
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn view(&self) -> Element<Message> {
        //einfache spalte
        column![

            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
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
}
