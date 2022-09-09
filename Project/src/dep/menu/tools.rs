use super::super::modules::{message::PageMessage, style, style::{button}};

use iced::{
     Alignment, button,  
    Column,  Length, Row,
     Space, Text,  Rule,
};

//Struct for Tools in Menu
#[derive(Debug, Clone)]
pub struct Tools {
    pub draw_button:        button::State,
    pub draw_active:        bool,
    pub draw_hole_button:   button::State,
    pub draw_hole_active:   bool,
    pub clear_button:       button::State,
    pub clear_active:       bool,
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
            redo_button:  button::State::new(),
            redo_active:  false,  //true if undone steps are available
            undo_button:  button::State::new(),
            undo_active:  false,  //true if content is available
        }
    }
    
    pub fn tool_menu(tools: &'a mut Tools) -> Column<'a, PageMessage>{

        //defining buttons with active and inactive style
        let mut button_d = Column::new();
        if tools.draw_active {
            button_d = button_d.push(button(&mut tools.draw_button, "Draw"))
        }
        else {
            button_d = button_d.push(button(&mut tools.draw_button, "Draw")
            .on_press(PageMessage::DrawPressed)
            .style(style::ButtonStyle::Secondary))
        };

        let mut button_dh = Column::new();
        if tools.draw_hole_active {
            button_dh = button_dh.push(button(&mut tools.draw_hole_button, "Draw Hole"))
        }
        else {
            button_dh = button_dh.push(button(&mut tools.draw_hole_button, "Draw Hole")
            .on_press(PageMessage::DrawHolePressed)
            .style(style::ButtonStyle::Secondary))
        };

        
        let mut button_c = Column::new();
        if !tools.clear_active {
            button_c = button_c.push(button(&mut tools.clear_button, "Clear"))
        }
        else {
            button_c = button_c.push(button(&mut tools.clear_button, "Clear")
            .on_press(PageMessage::ClearPressed)
            .style(style::ButtonStyle::Secondary))

        };

        let mut button_ud = Column::new();
        if tools.undo_active {
            button_ud = button_ud.push(button(&mut tools.undo_button, "Undo")
            .on_press(PageMessage::UndoPressed)
            .style(style::ButtonStyle::Secondary))
        }
        else {
            button_ud = button_ud.push(button(&mut tools.undo_button, "Undo"))
            
            
        };

        let mut button_rd = Column::new();
        if tools.undo_active {
            button_rd = button_rd.push(button(&mut tools.redo_button, "Redo")
            .on_press(PageMessage::RedoPressed)
            .style(style::ButtonStyle::Secondary))
        }
        else {
            button_rd = button_rd.push(button(&mut tools.redo_button, "Redo"))
            
            
        };

        //the actual layout of the tool menu
        Column::new()
            .spacing(10)
            .width(Length::Units(250))
            .height(Length::Units(250))
            .align_items(Alignment::Center)
            .padding(0)
            .push(Rule::horizontal(2).style(style::Rule::Ligth))
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
                .push(button_c)
               
            
            )
            .push(Space::with_height(Length::Fill))
            .push(Rule::horizontal(2).style(style::Rule::Ligth))

    }
}