use iced::{button, 
    Column, Container,  Element, Length, Row, Sandbox,
     Settings, Space, Text,  Rule, window, window::Position, Alignment, Point, 
};
use num_traits::cast::ToPrimitive;
mod dep;
use dep::{modules::{style, style::button, message::{PageMessage, Message},
    },  };

use dep::menu::{program_settings::ProgramSettings, tools::Tools, draw_panel::{DrawPanel, DrawState}, };

use dep::iteration::preview::PreviewPanel;

use dep::result::result_view::ResultPanel;


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
            if step_msg == PageMessage::ConfirmPressed || step_msg == PageMessage::EndPressed 
            {
                self.pages.advance()
            }
            else if step_msg == PageMessage::RepeatPressed {
                self.pages.return_to_menu()
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
                    undo_performed: false,
                    dark_mode: false
                
                },
                Page::Iteration {
                    preview_panel: PreviewPanel::new(),
                    dark_mode: false, 
                    prevoius_button: button::State::new(),
                    next_button: button::State::new(),
                    end_button: button::State::new(),
                    current_step: 1,
                    
                },
                Page::Result {
                    result_panel: ResultPanel::new(),
                    repeat_button: button::State::new(),
                    exit_button: button::State::new(),
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
            
            match self.current_page {
                //Menu Page
                0 => {
                    //Offsets example:
                    //x: width of draw_panel = 600         | y: height of draw_panel = 400
                    //   width of preview_panel = 1280     |    height of preview_panel = 500
                    // => offset_x = (1280 - 600)/ 2 = 340 | => offset_y = (500 - 400)/2 = 50
                    if let Some(offset_x) = ((self.pages[1].get_panel_width() - self.pages[0].get_panel_width())/2).to_f32(){
                        if let Some(offset_y) = ((self.pages[1].get_panel_height() - self.pages[0].get_panel_height())/2).to_f32() {
                            buffer = Page::buffer_move_center(buffer, offset_x, offset_y);
                        }
                    }
                    
                }
                //Iteration Page
                1 => {
                    
                    if let Some(offset_x) = ((self.pages[2].get_panel_width() - self.pages[1].get_panel_width())/2).to_f32(){
                        if let Some(offset_y) = ((self.pages[2].get_panel_height() - self.pages[1].get_panel_height())/2).to_f32() {
                            buffer = Page::buffer_move_center(buffer, offset_x, offset_y);
                        }
                    }
                }

                //other values do not accure but can be added by adding new pages
                // 2 is skipped because there is an extra function tho its can_continue() is false to prevent getting out of bounds
                _ => {}
            }
            //If the current Page is Result (idx == 2), next page will be the Menu (idx == 0)
            //no buffer has to be copyed
            self.pages[self.current_page +1].set_vertex_buffer(buffer);
            

            //advance to next page
            self.current_page += 1;

            
            
        }
    }
    //Function that sets the Page from Result back to Menu
    fn return_to_menu(&mut self) {
        self.pages[1].reset_iteration();
        self.pages[2].reset_iteration();
        self.current_page = 0;
    }


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
        undo_performed: bool,
        dark_mode: bool,
    },
    Iteration {
        preview_panel: PreviewPanel,
        prevoius_button: button::State,
        next_button: button::State,
        end_button: button::State,
        dark_mode: bool,
        current_step: usize,
        
    },
    Result {
        result_panel: ResultPanel,
        repeat_button: button::State,
        exit_button: button::State,
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
                if let Page::Menu { tools, draw_panel, undo_buffer, action_buffer , 
                                undo_performed, .. } = self {
                    
                    
                    tools.redo_active = true;

                    //check if the last action done was closing the polygon
                    if draw_panel.closed == true {
                       
                        draw_panel.closed = false;
                        draw_panel.ignore_input = false;

                    }
                    //marks that an undo action was done
                    //if a new vertex is drawn now, the undo buffer will be cleared, so that redo can not replicate points that where there before the
                    //new draw action
                    *undo_performed = true;
                    //undo last draw of vertex and pup action into buffer for undone action so they can be redone
                    let action_buffer_idx = action_buffer.len() - 1;
                   
                    undo_buffer.push(action_buffer[action_buffer_idx]);
                    action_buffer.pop();
                    draw_panel.vertices.pop();

                    draw_panel.polygon.request_redraw();

                    //check if more actions can be undone
                    if *action_buffer == vec![] {
                        tools.undo_active = false;
                    }

                    

                    //set the draw state to the last position so the drawing can be continued
                    if draw_panel.vertices != vec![]{
                        draw_panel.polygon.set_pending_waitinput(draw_panel.vertices[draw_panel.vertices.len() -1]);
                    }
                    else {
                        draw_panel.polygon.set_pending_none();
                    }

                    
                }
            }

            PageMessage::RedoPressed => {
                if let Page::Menu { tools, draw_panel, undo_buffer, action_buffer,
                            undo_performed, .. } = self {
                    
                    tools.undo_active = true;
                    *undo_performed = false;
                    //redo last undone action and push it back to the action buffer
                    let undo_buffer_idx = undo_buffer.len() - 1;
                   
                    action_buffer.push(undo_buffer[undo_buffer_idx]);
                    
                    if let PageMessage::AddPoint(vertex) = undo_buffer[undo_buffer_idx] {
                        draw_panel.vertices.push(vertex);
                    }
                    
                    undo_buffer.pop();

                    draw_panel.polygon.request_redraw();


                    //check if more actions can be redone
                    if *undo_buffer == vec![] {
                        tools.redo_active = false;
                    }

                    //set the draw state to the last position so the drawing can be continued
                    draw_panel.polygon.set_pending_waitinput(draw_panel.vertices[draw_panel.vertices.len() -1]);
                    
                    
                    
                }
            }
           
            PageMessage::ClearPressed => {
                if let Page::Menu {  draw_panel, tools, undo_performed ,..} = self {
                    
                    *undo_performed = false;
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
            
            PageMessage::AddPoint(vertex) => {
                if let Page::Menu { draw_panel, tools, action_buffer, undo_performed,
                                    undo_buffer,.. } = self {


                    tools.clear_active = true;
                    tools.undo_active = true;
                    
                    //if the last action performed was UNDO then clear the undo buffer
                    //preventing replicating the points from before the UNDO with REDO after new points got added on top
                    if *undo_performed {
                        undo_buffer.clear();
                        tools.redo_active = false;
                    }

                    Page::push_vertex_to_buffer(vertex, &mut draw_panel.vertices);            
                    
                    
                    draw_panel.polygon.request_redraw();

                    action_buffer.push(PageMessage::AddPoint(vertex));
                    
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

                if let Page::Iteration { preview_panel, current_step , ..} = self {
                    
                    *current_step -= 1;

                    preview_panel.polygon.request_redraw()
                }
            }
            PageMessage::NextPressed => {
                
                if let Page::Iteration { preview_panel, current_step , ..} = self {
                    
                    *current_step += 1;

                    preview_panel.polygon.request_redraw()
                }

            }

           PageMessage::EndPressed | PageMessage::RepeatPressed => {
                //These Messages get handeled in the GUI struct
           }


           PageMessage::ExitPressed => {
                //Exits tge Program
                std::process::exit(0)
           }

           

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
            Page::Iteration { .. } => true,
            Page::Result { .. } => false,

        }
    }

    fn view(&mut self) -> Element<PageMessage> {
        match self {
            Page::Menu { tools, progset, draw_panel, confirm_button, dark_mode ,..} 
                => Self::menu(tools, progset, draw_panel, confirm_button, dark_mode),

            Page::Iteration { preview_panel, dark_mode, prevoius_button, next_button, 
               end_button, current_step,  }
                => Self::iteration( preview_panel, prevoius_button, next_button, end_button, *dark_mode, *current_step, ),
            Page::Result { result_panel, exit_button, repeat_button ,dark_mode }
                => Self::result(result_panel, repeat_button, exit_button , *dark_mode),

        }
        .into()
    }
    //Function to provide easy creation of a container that holds all displayed content of a page
    fn container(title: &str) -> Column<'a, PageMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    //function that returns the vertex buffer of a page
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

    //function that sets the vertex buffer of a page to a given buffer
    fn set_vertex_buffer(&mut self, vertex_buffer:  Vec<Point>)  {
        match self {
            Page::Menu { .. } => {
            }
            Page::Iteration { preview_panel, .. } => {
               preview_panel.vertices = vertex_buffer;
            }
            Page::Result { result_panel, .. } => {
                result_panel.vertices =  vertex_buffer;
            }
        }
    }

    //Function that moves all vertices in a buffer with a given offset so that the resulting polygon is placed
    //in the center of the new canvas
    fn buffer_move_center(buffer: Vec<Point>, offset_x: f32, offset_y: f32) -> Vec<Point> {
        
        let mut output: Vec<Point> = vec![];

        for mut vertex in buffer {
            vertex.x = vertex.x + offset_x;
            vertex.y = vertex.y + offset_y;
            output.push(vertex);
        }
        return output;
    }

    //Function that adds a Point to the vertex buffer, it checks if the given vertex is already given as the first vertex, 
    //so that there is no double vertex at the start/end of the polygon
    //TODO check that NONE of the vertices are doubled
    fn push_vertex_to_buffer(vertex: Point, buffer: &mut Vec<Point>) {
        if buffer.len() > 0 {
            if vertex != buffer[buffer.len() -1] {
                buffer.push(vertex);
            }
        }
        else {
            buffer.push(vertex);
        }
    }

    //Function thet returns the width of a canvas
    fn get_panel_width(&self) -> u16 {
        match self {
            Page::Menu { draw_panel, .. } => {
                draw_panel.panel_width
            }
            Page::Iteration { preview_panel, .. } => {
                preview_panel.panel_width
            }
            Page::Result { result_panel, .. } => {
                result_panel.panel_width
            }
        }
    }
    //Function that returns the width of a canvas
    fn get_panel_height(&self) -> u16 {
        match self {
            Page::Menu { draw_panel, .. } => {
                draw_panel.panel_height
            }
            Page::Iteration { preview_panel, .. } => {
                preview_panel.panel_height
            }
            Page::Result { result_panel, .. } => {
                result_panel.panel_height
            }
        }
    }

    //Function that resets the step_count and line buffer of iteration, so the algorithm can be performed again
    fn reset_iteration(&mut self) {
        match self {
            
            Page::Iteration { current_step, preview_panel, .. } => {

                    let one: usize = 1;
                    *current_step = one;

                    preview_panel.polygon.request_redraw();
                
                //diagonals = vec![];
                
            }
            Page::Result { result_panel, .. } => {
                
                result_panel.polygon.request_redraw();
            
            }
            _ => {}
        }
    }

    //function that defines the look of the menu with a draw poanel and drawing tools as well as several options for the algorithm
    fn menu(tools: &'a mut Tools, progset: &'a mut ProgramSettings, draw_panel: &'a mut DrawPanel, confirm_button: &'a mut button::State,
            dark_mode: &mut bool,) -> Column<'a, PageMessage> {
        
        let mut button_con = button(confirm_button, "Confirm");
        
        //should be draw_panel.closed but this does not get read when it reached true due to unknown reason
        //idea: need a nother action performed to refresh view of the menu page
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

    //Function that defines the look of the Iteration Page, where the algorithm is performed
    fn iteration(preview_panel: &'a mut PreviewPanel, previous_button: &'a mut button::State, next_button: &'a mut button::State, 
                end_button: &'a mut button::State, dark_mode: bool, current_step: usize, ) 
        -> Column<'a, PageMessage> {
        
        // caculation max number of steps for the triangulation
        // polygon with n edges alsways consists of n-2 triangles
        // one triangle per step means n-2 steps
        let max_steps = preview_panel.vertices.len() - 2;
        
        //init all buttons with messages
        let mut button_prev = button(previous_button, "Previous");
            if current_step > 1 {
                button_prev = button_prev.on_press(PageMessage::PreviousPressed);
            }
        
        let mut button_next = button(next_button, "Next").on_press(PageMessage::NextPressed);
        let mut button_end = button(end_button, "End").on_press(PageMessage::EndPressed);
        let preview = Row::new().align_items(Alignment::Center)
                .push(PreviewPanel::preview_panel(preview_panel));
        
        //button styles depending if dark mode active or not
        if dark_mode {
            button_prev = button_prev.style(style::ButtonStyle::SecondaryDark);
            button_next = button_next.style(style::ButtonStyle::PrimaryDark);
            button_end = button_end.style(style::ButtonStyle::PrimaryDark);

        }else {
            button_prev = button_prev.style(style::ButtonStyle::SecondaryLight);
            button_next = button_next.style(style::ButtonStyle::PrimaryLight);
            button_end = button_end.style(style::ButtonStyle::PrimaryLight);
        }
        
        // text to display triangulation progress
        let stepcount = format!("Iteration Step {}/{}", current_step, max_steps);

        // navigation bar below the preview panel
        let mut controls = Row::new().spacing(450).align_items(Alignment::Center);

  
            controls = controls.push(button_prev);
     
            controls = controls.push(Text::new(stepcount).size(22));

        // if all steps have been completed one can continue to the result page
            if current_step == max_steps {
                controls = controls.push(button_end);
            }
            else {
                controls = controls.push(button_next);
            }
            
        
        Self::container("").push(preview).push(controls)
        
    }

    //Function that defines the look of the Result Page
    fn result(result_panel: &'a mut ResultPanel, repeat_button: &'a mut button::State, exit_button: &'a mut button::State,dark_mode: bool) 
    -> Column<'a, PageMessage> {
       
        let result = Row::new().align_items(Alignment::Center).push(ResultPanel::result_panel(result_panel));
        
        //init all buttons with messages
        let mut button_rep = button(repeat_button, "Return \n to Menu").on_press(PageMessage::RepeatPressed);
        let mut button_exit = button(exit_button, "Exit").on_press(PageMessage::ExitPressed);
        
        //button styles depending if dark mode active or not
        if dark_mode {
            button_exit = button_exit.style(style::ButtonStyle::SecondaryDark);
            button_rep = button_rep.style(style::ButtonStyle::PrimaryDark);
           

        }else {
            button_exit = button_exit.style(style::ButtonStyle::SecondaryLight);
            button_rep = button_rep.style(style::ButtonStyle::PrimaryLight);
        }


        let mut controls = Row::new().spacing(450).align_items(Alignment::Center);

            controls = controls.push(button_exit)
                               .push(Space::with_width(Length::Fill))
                               .push(button_rep);

        Self::container("").push(result).push(controls)
    }

    
}