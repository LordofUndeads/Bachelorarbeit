use std::{process::{exit, Termination, ExitCode}, };

use iced::{button, 
    Column, Container,  Element, Length, Row, Sandbox,
     Settings, Space, Text,  Rule, window, window::Position,  
};

mod dep;
use dep::{modules::{style, style::button, message::{PageMessage, Message},
    }, menu::draw_panel};

use dep::menu::{program_settings::ProgramSettings, tools::Tools, draw_panel::{DrawPanel, DrawState}, };




pub fn main() -> iced::Result {

    //PopUpLoader::init_card()
    Gui::run(Settings {
        antialiasing: true,
        window: window::Settings {
            resizable: false,
            position: Position::Centered,
            size: (1280, 720),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
pub struct Gui {
    pages: Pages, 
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Gui {
        Gui {
            pages: Pages::new(),
            
        }
    }

    fn title(&self) -> String {
        format!("{}", self.pages.title())
    }

    fn update(&mut self, event: Message) {
        match event {
           
            Message::PageMessage(step_msg) => 
            if step_msg == PageMessage::ConfirmPressed {
                self.pages.advance()
            }
            else{
                self.pages.update(step_msg, );
            }

            
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Gui {
            pages: steps,
            ..
        } = self;

        

        let content: Element<_> = Column::new()
            .max_width(1280)
            .max_height(720)
            .spacing(20)
            .padding(20)
            .push(steps.view().map(Message::PageMessage))
      
            .into();

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .width(Length::Fill)
            .style(style::WindowStyle::Light)
            .into()
    }
}



struct Pages {
    pages: Vec<Page>,
    current: usize,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            pages: vec![
                Page::Menu {
                    tools: Tools::new(),
                    progset: ProgramSettings::new(),
                    draw_panel: DrawPanel::new(),
                    confirm_button: button::State::new(),
                    undo_buffer: vec![],
                    action_buffer: vec![],
                
                },
                Page::Iteration,
                Page::Result,
             
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: PageMessage, ) {
        self.pages[self.current].update(msg, );
    }

    fn view(&mut self) -> Element<PageMessage> {
        self.pages[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    // fn go_back(&mut self) {
    //     if self.has_previous() {
    //         self.current -= 1;
    //     }
    // }

    // fn has_previous(&self) -> bool {
    //     self.current > 0
    // }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.pages.len()
            && self.pages[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.pages[self.current].title()
    }
}

enum Page {

    Menu { 
        tools: Tools,
        progset: ProgramSettings,
        draw_panel: DrawPanel,
        confirm_button: button::State,
        undo_buffer: Vec<PageMessage>,
        action_buffer: Vec<PageMessage>,
        
    },
    Iteration,
    Result,

}



impl<'a> Page {
    fn update(&mut self, msg: PageMessage) {
        match msg {
        
            PageMessage::AlgorithmSelected(_algorithm) => {
                    if let Page::Menu { progset, .. } = self {
                        progset.algorithm = Some(_algorithm);
                    }
                }

            PageMessage::HeuristicSelected(_heuristic) => {
                if let Page::Menu { progset, .. } = self {
                    progset.heuristic = Some(_heuristic);
                }
            }

            PageMessage::EdgeSwappingToggled(is_active) => {
                if let Page::Menu { progset, .. } = self {
                    progset.bools.edgeswapping_active = is_active;
                }
            }

            PageMessage::StepTrigToggled(is_active) => {
                if let Page::Menu { progset, .. } = self {
                    progset.bools.stepwise_active = is_active;
                }
            }

            PageMessage::DrawPressed => {
                if let Page::Menu { tools, draw_panel, .. } = self {
                    tools.draw_active = true;
                    tools.draw_hole_active = false;
                    

                    draw_panel.ignore_input = !tools.draw_active;
                }
            }

            PageMessage::DrawHolePressed => {
                if let Page::Menu { tools, draw_panel, .. } = self {
                    tools.draw_active = false;
                    tools.draw_hole_active = true;

                    draw_panel.ignore_input = !tools.draw_active;
                    draw_panel.polygon.set_pending_none();
                }
            }


            PageMessage::UndoPressed => {
                if let Page::Menu { tools, draw_panel, .. } = self {
                    tools.draw_active = false;
                    tools.draw_hole_active = false;
                    tools.redo_active = true;

                    if draw_panel.vertices != vec![] {
                        tools.undo_active = true
                    }
                    else { tools.undo_active = false};

                    draw_panel.ignore_input = !tools.draw_active;
                }
            }

            PageMessage::RedoPressed => {
                if let Page::Menu { tools, draw_panel, .. } = self {
                    tools.draw_active = false;
                    tools.draw_hole_active = false;
                    tools.undo_active = true;

                    if draw_panel.vertices != vec![] {
                        tools.redo_active = true
                    }
                    else { tools.redo_active = false};

                    draw_panel.ignore_input = !tools.draw_active;
                }
            }
           
            PageMessage::ClearPressed => {
                if let Page::Menu {  draw_panel, tools, ..} = self {
                    
                    tools.popup_clear_open = true;
                    tools.draw_active = true;
                    draw_panel.ignore_input = true;
                    
                }
            }

            PageMessage::PopUpClosed | PageMessage::RejectClear => {
                if let Page::Menu {  tools, draw_panel, ..} = self { 
                    
                    tools.popup_clear_open = false;
                    tools.clear_active = true;
                    tools.draw_active = true;
                    tools.draw_hole_active = true;
                    draw_panel.ignore_input = false;
                    

                }

            }
            
            PageMessage::AddLine(line) => {
                if let Page::Menu { draw_panel, tools, action_buffer, .. } = self {


                    tools.clear_active = true;
                    tools.undo_active = true;
                    
                    draw_panel.vertices.push(line.from);
                    draw_panel.vertices.push(line.to);
                    draw_panel.polygon.request_redraw();

                    action_buffer.push(PageMessage::AddLine(line));
                    
                }

            }

            PageMessage::ClearAll => {
                if let Page::Menu {  draw_panel, tools, action_buffer, undo_buffer, ..} = self { 
                    tools.popup_clear_open = false;

                    //reactivate buttons
                    tools.clear_active = false;
                    tools.draw_active = false;
                    tools.undo_active = false;
                    tools.redo_active = false;
                    tools.draw_hole_active = false;

                    //clearing the input
                    draw_panel.polygon = DrawState::default();
                    draw_panel.vertices.clear();

                    //clearing undo and redo
                    action_buffer.clear();
                    undo_buffer.clear();
                }
            }

            PageMessage::ConfirmPressed => {
                
            }

            // PageMessage::BackPressed => {
            //     self.pages.go_back();
            // }
            // PageMessage::NextPressed => {
            //     self.pages.advance();


        };
    }


    fn title(&self) -> &str {
        match self {
            Page::Menu { .. } => "Triangulation for Polygons - Menu",
            Page::Iteration => "Triangulation for Polygons - Algorithm Iteration",
            Page::Result => "Triangulation for Polygons - Result",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Page::Menu { draw_panel, .. } => if draw_panel.vertices != vec![] { true} else { false},
            Page::Iteration => true,
            Page::Result => false,

        }
    }

    fn view(&mut self) -> Element<PageMessage> {
        match self {
            Page::Menu { tools, progset, draw_panel, confirm_button, ..} =>
                Self::menu(tools, progset, draw_panel, confirm_button),

            Page::Iteration => Self::iteration(),
            Page::Result => Self::result(),

        }
        .into()
    }

    fn container(title: &str) -> Column<'a, PageMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }


    


    fn menu(tools: &'a mut Tools, progset: &'a mut ProgramSettings, draw_panel: &'a mut DrawPanel, confirm_button: &'a mut button::State) -> Column<'a, PageMessage> {
        
        let button_con = if draw_panel.vertices != vec![] {
            button(confirm_button, "Confirm").style(style::ButtonStyle::PrimaryLight).on_press(PageMessage::ConfirmPressed)
        }
        else {
            button(confirm_button, "Confirm").style(style::ButtonStyle::PrimaryLight)
        };

        let mut tool_menu: Column<PageMessage> = Column::new();
                                                
            if tools.popup_clear_open {
                tool_menu = tool_menu.height(Length::Units(400))
            } else { tool_menu = tool_menu.height(Length::Units(250))}
        
            tool_menu = tool_menu
            .width(Length::Fill)
            .push(Row::new()
                .push(Rule::vertical(2).style(style::RuleStyle::Light))
                .push(Tools::tool_menu(tools))
                .push(Rule::vertical(2).style(style::RuleStyle::Light)));

        let draw_panel: Column<PageMessage> = Column::new()
                                                .push(DrawPanel::draw_panel(draw_panel)) ;
        let setting_menu: Row<PageMessage> = ProgramSettings::prog_settings(progset);
        Self::container("")
        .max_width(1280)
        .max_height(720)
        .spacing(0)
        
        .push(Row::new()
            
            .push(draw_panel)
            .push(Space::with_width(Length::Units(20)))
            .push(tool_menu)
           )
        .push(Space::with_height(Length::Units(20)))
        .push(setting_menu)
        .push(Row::new()
            .push(Column::new().width(Length::Fill)
            .push(Space::with_height(Length::Units(20)))
                .push(Row::new()
                    .push(Space::with_width(Length::Fill))
                    .push(button_con))))
            
        
            
          
    }

    fn iteration() -> Column<'a, PageMessage> {
        Self::container("Placeholder Iteration")
        // let mut controls = Row::new();

        // if steps.has_previous() {
        //     controls = controls.push(
        //         button(back_button, "Back")
        //             .on_press(Message::BackPressed)
        //             .style(style::Button::Secondary),
        //     );
        // }

        // controls = controls.push(Space::with_width(Length::Fill));

        // if steps.can_continue() {
        //     controls = controls.push(
        //         button(next_button, "Next")
        //             .on_press(Message::NextPressed)
        //             .style(style::Button::Primary),
        //     );
        // }
    }

    fn result() -> Column<'a, PageMessage> {
        Self::container("Placeholder Result")
          
    }
}















