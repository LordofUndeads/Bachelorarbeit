use iced::{button, Alignment, Element, Sandbox, Settings, Button, Column, Text,};
#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,

}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed
}
pub fn main() -> iced::Result {
    
    Counter::run(Settings::default())
    
}


    impl Sandbox for Counter{
        
        type Message = Message;

        fn new() -> Self {
           Self::default()
        }

        fn title(&self) -> String{
            String::from("Counter - Iced")
        }

        fn view(&mut self) -> Element<Message> {
            //einfache spalte
            Column::new()
            .push(Button::new(&mut self.increment_button, Text::new("+")).on_press(Message::IncrementPressed),)
            .push(Text::new(self.value.to_string()).size(50),)
            .push(Button::new(&mut self.decrement_button, Text::new("-")).on_press(Message::DecrementPressed),)
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
