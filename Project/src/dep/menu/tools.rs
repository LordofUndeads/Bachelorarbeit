use super::super::modules::{message::{PageMessage}, style, style::{button, ButtonStyle}};

use iced::{
     Alignment, button,  
    Column,  Length, Row,
     Space, Text,  Rule, Button, alignment::{Vertical, Horizontal}
};

use iced_aw::Card;

//Struct for Tools in Menu
#[derive(Debug, Clone)]
pub struct Tools {
    pub draw_button:        button::State,
    pub draw_active:        bool,
    pub draw_hole_button:   button::State,
    pub draw_hole_active:   bool,
    pub clear_button:       button::State,
    pub clear_active:       bool,
    pub popup_clear_open:   bool,
    pub button_yes:         button::State,
    pub button_no:          button::State,
    pub redo_button:        button::State,
    pub redo_active:        bool,
    pub undo_button:        button::State,
    pub undo_active:        bool,
}

impl<'a> Tools {
    pub fn new() -> Self {
        Tools {
            draw_button: button::State::new(),
            draw_active: false, //true if button clicked
            draw_hole_button: button::State::new(),
            draw_hole_active: false, //true if button clicked
            clear_button: button::State::new(),
            clear_active: false, //only true if content is available
            popup_clear_open: false, //dialogue popup to confirm if the draw panel should be cleared
            button_yes: button::State::new(),
            button_no: button::State::new(),

            redo_button:  button::State::new(),
            redo_active:  false,  //true if undone steps are available
            undo_button:  button::State::new(),
            undo_active:  false,  //true if content is available
        }
    }
    
    pub fn tool_menu(tools: &'a mut Tools, dark_mode: bool) -> Column<'a, PageMessage>{

        //defining buttons with active and inactive style and Dark/Light Mode
        let mut button_d = button(&mut tools.draw_button, "Draw");
        if !tools.draw_active {
            button_d = button_d.on_press(PageMessage::DrawPressed);
        }

        let mut button_dh = button(&mut tools.draw_hole_button, "Draw Hole");
        if !tools.draw_hole_active {
            button_dh = button_dh.on_press(PageMessage::DrawHolePressed);
        }
        let mut button_ud = button(&mut tools.undo_button, "Undo");
        if tools.undo_active {
            button_ud = button_ud.on_press(PageMessage::UndoPressed);
        }

        let mut button_rd = button(&mut tools.redo_button, "Redo");
        if tools.undo_active {
            button_rd = button_rd.on_press(PageMessage::RedoPressed)
        }

        let mut button_c = button(&mut tools.clear_button, "Clear");
        if tools.clear_active {
            button_c = button_c.on_press(PageMessage::ClearPressed)
        }
        
        if dark_mode {
            button_d = button_d.style(style::ButtonStyle::SecondaryDark);
            button_dh = button_dh.style(style::ButtonStyle::SecondaryDark);
            button_ud = button_ud.style(style::ButtonStyle::SecondaryDark);
            button_rd = button_rd.style(style::ButtonStyle::SecondaryDark);
            button_c = button_c.style(style::ButtonStyle::SecondaryDark);
        }
        else {
            button_d = button_d.style(style::ButtonStyle::SecondaryLight);
            button_dh = button_dh.style(style::ButtonStyle::SecondaryLight);
            button_ud = button_ud.style(style::ButtonStyle::SecondaryLight);
            button_rd = button_rd.style(style::ButtonStyle::SecondaryLight);
            button_c = button_c.style(style::ButtonStyle::SecondaryLight);
        }

        let mut clear_dialogue = Column::new();
        if !tools.clear_active {
            
            if dark_mode {
                button_c = button_c.style(style::ButtonStyle::SecondaryDark);
            }
            else {
                button_c = button_c.style(style::ButtonStyle::SecondaryLight);
            }
            clear_dialogue = clear_dialogue.push(button_c);
        }
        else {
            //either a button or a popup card depending on the popup_clear_open boolean
            if tools.popup_clear_open {
                let mut clear_card = Card::new(
                    Text::new("Clear the Draw Panel?").size(20),
                    Column::new()
                        .padding(5)
                        .spacing(5)
                        .push(Row::new()
                            .align_items(Alignment::Start)
                            .push(Column::new()
                            .push(Text::new("Are you sure that you want to clear the whole Draw Panel?").size(20))
                            .push(Text::new("This can not be undone!").size(20))))
                        .push(Row::new()
                            .padding(5)
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(Button::new(&mut tools.button_no, Text::new("No, \n don't clear!")
                                        .size(16).vertical_alignment(Vertical::Center).horizontal_alignment(Horizontal::Center))
                                .width(Length::Units(90))
                                .height(Length::Units(40))
                                .on_press(PageMessage::RejectClear)
                                .style(ButtonStyle::Red))
                            
                            .push(Button::new(&mut tools.button_yes, Text::new("Yes, \n clear all!")
                                        .size(16).vertical_alignment(Vertical::Center).horizontal_alignment(Horizontal::Center))
                                .width(Length::Units(90))
                                .height(Length::Units(40))
                                .on_press(PageMessage::ClearAll)
                                .style(ButtonStyle::Green)))
                        
                )
                .height(Length::Units(190))
                .width(Length::Units(300))
                .on_close(PageMessage::PopUpClosed)
                .style(style::PopUpStyle::Light);

                if dark_mode {
                    clear_card = clear_card.style(style::PopUpStyle::Dark);
                }
                else {
                    clear_card = clear_card.style(style::PopUpStyle::Light);
                    
                }
                clear_dialogue = clear_dialogue.push(clear_card);
                
            }
            else {
                clear_dialogue = clear_dialogue.push(button_c);
            }
            
            
        };

       

        //the actual layout of the tool menu
        let mut content = Column::new();

        if tools.popup_clear_open {content = content.height(Length::Units(400))}
            else {content = content.height(Length::Units(250))}
            
        content = content
            .spacing(10)
            .max_height(400)
            .height(Length::Units(400))
            .width(Length::Units(250))
            .align_items(Alignment::Center)
            .padding(0)
            .push(Rule::horizontal(2).style(style::RuleStyle::Light))
            .push(Text::new("Drawing Tools").size(20))
            .push(Row::new()
                .spacing(10)
                .padding(10)
                .push(button_d)
                .push(button_dh)
               
            )
            .push(Row::new().spacing(10)
                .padding(10)
                .push(button_ud)
                .push(button_rd)
            
            )
            .push(Row::new().spacing(10).align_items(Alignment::Center)
                .padding(10)
                .push(clear_dialogue)
               
            
            )
            .push(Space::with_height(Length::Fill))
            .push(Rule::horizontal(2).style(style::RuleStyle::Light));

            

            return content;

    }
}