use iced::{button, 
    Column, Container,  Element, Length, Row, Sandbox,
     Settings, Space, Text,  Rule, window, window::Position, Alignment, Point
};

mod dep;
use dep::{modules::{style, style::button, message::{PageMessage, Message},
    }, };

use dep::menu::{program_settings::ProgramSettings, tools::Tools, draw_panel::{DrawPanel, DrawState}, };

use dep::iteration::preview::{PreviewPanel,PreviewState};




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
    dark_mode: bool,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Gui {
        Gui {
            pages: Pages::new(),
            dark_mode: false,
            
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
            else if step_msg == PageMessage::DarkModeToggled(!self.dark_mode){
                self.dark_mode = !self.dark_mode;
                self.pages.update(step_msg, );
            }
            else{
                self.pages.update(step_msg, );
            }

            
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Gui {
            pages: steps,
            dark_mode,
            ..
        } = self;

        

        let content: Element<_> = Column::new()
            .max_width(1280)
            .max_height(720)
            .spacing(20)
            .padding(20)
            .push(steps.view().map(Message::PageMessage))
      
            .into();

        let mut container = Container::new(content)
            .height(Length::Fill)
            .center_y()
            .width(Length::Fill);
            
            if *dark_mode {
               container = container.style(style::WindowStyle::Dark);
            } 
            else { 
                container = container.style(style::WindowStyle::Light);}

        container.into()
           
    }
}



struct Pages {
    pages: Vec<Page>,
    current_page: usize,
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
                    dark_mode: false
                
                },
                Page::Iteration {
                    preview_panel: PreviewPanel::new(),
                    dark_mode: false, 
                    prevoius_button: button::State::new(),
                    next_button: button::State::new(),
                    current: 1,
                    
                },
                Page::Result {
                    dark_mode: false,
                },
             
            ],
            current_page: 0,
        }
    }

    fn update(&mut self, msg: PageMessage, ) {
        self.pages[self.current_page].update(msg, );
    }

    fn view(&mut self) -> Element<PageMessage> {
        self.pages[self.current_page].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {

            //copying vertices from one page to the next page for preview purposes
            
            let mut buffer = self.pages[self.current_page].get_vertex_buffer();
            self.pages[self.current_page +1].set_vertex_buffer(&mut buffer);
            

            //advance to netx page
            self.current_page += 1;
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
        self.current_page + 1 < self.pages.len()
            && self.pages[self.current_page].can_continue()
    }

    fn title(&self) -> &str {
        self.pages[self.current_page].title()
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
        dark_mode: bool,
    },
    Iteration {
        preview_panel: PreviewPanel,
        prevoius_button: button::State,
        next_button: button::State,
        dark_mode: bool,
        current: i16,
        
    },
    Result {
        dark_mode: bool,
    },

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
                //conversion to grid and execute algorithm with heuristc and settings
               
            }

            PageMessage::DarkModeToggled(_dark_mode) => {
                if let Page::Menu {progset, ..} = self {
                    
                    progset.bools.dark_mode = _dark_mode;
                    
                }
            }

            PageMessage::PreviousPressed => {
               
            }
            PageMessage::NextPressed => {
                
                if let Page::Iteration { preview_panel, current , ..} = self {
                    
                    *current = *current + 1;

                    preview_panel.polygon.request_redraw()
                }

            }

            //end button instead of next button if stepcount == max_steps

        };
    }


    fn title(&self) -> &str {
        match self {
            Page::Menu { .. } => "Triangulation for Polygons - Menu",
            Page::Iteration { .. }=> "Triangulation for Polygons - Algorithm Iteration",
            Page::Result { .. }=> "Triangulation for Polygons - Result",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Page::Menu { draw_panel, .. } => if draw_panel.vertices != vec![] { true} else { false},
            Page::Iteration { .. }=> true,
            Page::Result { .. }=> false,

        }
    }

    fn view(&mut self) -> Element<PageMessage> {
        match self {
            Page::Menu { tools, progset, draw_panel, confirm_button, dark_mode ,..} 
                => Self::menu(tools, progset, draw_panel, confirm_button, dark_mode),

            Page::Iteration { preview_panel, dark_mode, prevoius_button, next_button, 
                current: current_step,  }
                => Self::iteration( preview_panel, prevoius_button, next_button, *dark_mode, *current_step, ),
            Page::Result { .. }=> Self::result(),

        }
        .into()
    }

    fn container(title: &str) -> Column<'a, PageMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn get_vertex_buffer(&mut self) -> Vec<Point> {
        match self {
            Page::Menu { draw_panel, .. } => {
                (*draw_panel.vertices).to_vec()
            }
            Page::Iteration { preview_panel, .. } => {
                (*preview_panel.vertices).to_vec()
            }
            Page::Result { .. } => {
                vec![]
            }
        }
    }

    fn set_vertex_buffer(&mut self, vertex_buffer: &mut Vec<Point>)  {
        match self {
            Page::Menu { .. } => {
            }
            Page::Iteration { preview_panel, .. } => {
               preview_panel.vertices.append(vertex_buffer);
            }
            Page::Result { .. } => {
                //result_view.vertices = vertex_buffer
            }
        }
    }


    fn menu(tools: &'a mut Tools, progset: &'a mut ProgramSettings, draw_panel: &'a mut DrawPanel, confirm_button: &'a mut button::State,
            dark_mode: &mut bool,) -> Column<'a, PageMessage> {
        
        let mut button_con = button(confirm_button, "Confirm");
        if draw_panel.vertices != vec![] {
            button_con = button_con.on_press(PageMessage::ConfirmPressed)
        }
        
        if *dark_mode {
            button_con = button_con.style(style::ButtonStyle::PrimaryDark);
        }
        else {
            button_con = button_con.style(style::ButtonStyle::PrimaryLight);
        }

        let mut tool_menu: Column<PageMessage> = Column::new();
                                                
            if tools.popup_clear_open {
                tool_menu = tool_menu.height(Length::Units(400))
            } else { tool_menu = tool_menu.height(Length::Units(250))}
        
            tool_menu = tool_menu
            .width(Length::Fill)
            .push(Row::new()
                .push(Rule::vertical(2).style(style::RuleStyle::Light))
                .push(Tools::tool_menu(tools, progset.bools.dark_mode))
                .push(Rule::vertical(2).style(style::RuleStyle::Light)));

        let draw_panel: Column<PageMessage> = Column::new()
                                                .push(DrawPanel::draw_panel(draw_panel, dark_mode)) ;
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

    fn iteration(preview_panel: &'a mut PreviewPanel, previous_button: &'a mut button::State, next_button: &'a mut button::State, dark_mode: bool,
                current_step: i16, ) 
        -> Column<'a, PageMessage> {
        
        let max_steps = preview_panel.vertices.len() - 2;

        let mut button_prev = button(previous_button, "Back").on_press(PageMessage::PreviousPressed);
        let mut button_next = button(next_button, "Next").on_press(PageMessage::NextPressed);
        
        let preview = Row::new().push(PreviewPanel::preview_panel(preview_panel, dark_mode));

        if dark_mode {
            button_prev = button_prev.style(style::ButtonStyle::SecondaryDark);
            button_next = button_next.style(style::ButtonStyle::PrimaryDark);

        }else {
            button_prev = button_prev.style(style::ButtonStyle::SecondaryLight);
            button_next = button_next.style(style::ButtonStyle::PrimaryLight);
        }
        
        let stepcount = format!("Iteration Step {}/{}", current_step, max_steps);

        let mut controls = Row::new().spacing(450).align_items(Alignment::Center);

        //if steps.has_previous() {
            controls = controls.push(button_prev);
        //}
            controls = controls.push(Text::new(stepcount).size(22));
        //controls = controls.push(Space::with_width(Length::Fill));

        //if steps.can_continue() {
            controls = controls.push(button_next);
        //}
        
        Self::container("").push(preview).push(controls)
        
    }

    fn result() -> Column<'a, PageMessage> {
        Self::container("Placeholder Result")
          
    }
}















