use iced::{Sandbox, Settings, executor, Command, Container, Renderer, window};
use iced::{button, Button, Element, Column, Text, Row, Radio};
use iced::{alignment::Horizontal, Alignment, Length, Space};
use iced::{Vector};

pub struct Loader;

impl Loader {
    pub fn init_gui() -> iced::Result {
        Program::run(Settings::default())
    }
}

//The Program with its GUI

pub struct Program {
    pages: Gui,
    next_button: button::State,
    previous_button: button::State,
    confirm_button: button::State,
   
}

#[derive(Debug, Clone)]
//Messages for the Sandbox
pub enum Message {
    BackPressed,
    NextPressed,
    ConfirmPressed,
    PageMessage(PageMessage),
}

impl Sandbox for Program {
    type Message = Message;

    fn new() -> Program {
        Program {
            pages: Gui::new(),
            confirm_button: button::State::new(),
            next_button: button::State::new(),
            previous_button: button::State::new(),
            
        }
    }

    fn title(&self) -> String {
        format!("{}", self.pages.title())
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::BackPressed => {
                self.pages.previous();
            }

            Message::NextPressed => {
                self.pages.next();
            }

            Message::ConfirmPressed => {

            }

            Message::PageMessage(page_msg) => {
                self.pages.update(page_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message>{
        let Program {
            pages, 
            next_button,
            previous_button,
            confirm_button,
            ..
        } = self;

        let mut controls = Row::new();

        if pages.can_continue() {
            controls = controls.push(
                Button::new(&mut self.next_button, 
                    Text::new("Next").horizontal_alignment(Horizontal::Center)).on_press(
                        Message::ConfirmPressed).width(Length::Units(120))
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if pages.has_previous() {
            controls = controls.push(
                Button::new(&mut self.previous_button, 
                    Text::new("Previous").horizontal_alignment(Horizontal::Center)).on_press(
                        Message::ConfirmPressed).width(Length::Units(120))
            );
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(pages.view().map(Message::PageMessage))
            .push(controls)
            .into();
    }
}
//Types of Pages for GUI, implementation holding the layout and funtionalities for this see "impl <'a> Page{}"
pub enum Page {
    Menu {
        algorithm: Algorithm,
        heuristic: Heuristic,
        draw_button: button::State,
        draw_hole_button: button::State,
    },
    Iteration,
    Result,
}
//Special Messages triggered on the Pages
#[derive(Clone, Debug)]
pub enum PageMessage {
    AlgorithmSelected(Algorithm),
    HeuristicSelected(Heuristic),
    TimerElapsed,
    DrawButtonPressed,
    DrawHolePressed,
    ClearPressed,
    EraserPressed,
}

//Struct holding all the functionalities as a template
pub struct Gui {
    pages: Vec<Page>,
    current: usize,
    

}

impl Gui {
    fn new() -> Gui {
        Gui {
            pages: vec![
                Page::Menu {
                    algorithm: Algorithm::EarClipping,
                    heuristic: Heuristic::Random,
                    draw_button: button::State::new(),
                    draw_hole_button: button::State::new(),
                },
                Page::Iteration,
                Page::Result,
            ],
            current: 0,
            
        }
    }

    fn update(&mut self, msg: PageMessage){
        self.pages[self.current].update(msg)
    }

    fn view(&mut self) -> Element<Message> {
        self.pages[self.current].view()
    }

    fn next(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn previous(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn can_continue(&self) -> bool {
        self.current  +1 < self.pages.len() && self.pages[self.current].can_continue()
    }

    fn has_previous(&self) -> bool {
        self.current  > 0 
    }

    fn title(&self) -> &str {
        self.pages[self.current].title()
    }
}

impl <'a> Page {

    fn update(&mut self, msg: PageMessage) {
        match msg {
            PageMessage::AlgorithmSelected(_algorithm) => {
                if let Page::Menu { algorithm, .. } = self {
                    *algorithm = _algorithm;
                }
            }

            PageMessage::HeuristicSelected(_heuristic) => {
                if let Page::Menu { heuristic, .. } = self {
                    *heuristic = _heuristic;
                }
            }

            PageMessage::TimerElapsed => {
                //to do
            }
            PageMessage::DrawButtonPressed => {

            }

            PageMessage::DrawHolePressed => {
                
            }

            PageMessage::ClearPressed => {
                
            }

            PageMessage::EraserPressed => {
                
            }
            //evtl togglerChanged noch nötig
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
            Page::Menu { .. } => true,
            //Page::Menu { algorithm, heuristic, .. } => *algorithm == Some(Algorithm) && *heuristic == Some(Heuristic),
            Page::Iteration { .. } => true,
            Page::Result => false,
        }
    }

    fn view(&mut self) -> Element<PageMessage> {
        match self {
            Page::Menu { .. }=> Self::menu(),
            Page::Iteration => Self::iteration(),
            Page::Result => Self::result(),
        }
    }

    //Page Definitions as Containers
    //Default Page Layout
    fn container() -> Column<'a, PageMessage> {
        Column::new()
            .spacing(20)
            .width(Length::Units(1000))
            .max_width(600)
            .height(Length::Units(400))

    }
    //Menu Page (Start Page with all Options)
    fn menu() -> Column<'a, PageMessage> {
        //variables
        let mut algorithm: Algorithm;
        let mut heuristic: Heuristic;

        //Layout
        Self::container()
        .push(Self::menu_tools())
        .push(Self::menu_drawpanel())
        .push(Self::menu_options())

    }
    //Menu Parts
    fn menu_tools() -> Column<'a, PageMessage> {

        Column::new()
            .spacing(20)
            .width(Length::Units(250))
            .height(Length::Units(300))
            .align_items(Alignment::Center)
            .padding(0)
            .push(Text::new("Drawing Tools").size(25))
            .push(Row::new().spacing(10)
                // .push(button(draw_button, "Draw"))
                // .push(draw_hole_button)
            )
            .push(Row::new().spacing(10)
                // .push(eraser_button)
                // .push(clear_button)
            )
            .push(Space::with_height(Length::Fill))
    }

    fn menu_drawpanel() -> Column<'a, PageMessage> {
        // let draw_button = Button::new(&mut self.draw_button, 
        //     Text::new("Draw").horizontal_alignment(Horizontal::Center)).on_press(
        //         PageMessage::DrawButtonPressed).width(Length::Units(120));
        Column::new()
            .spacing(20)
            .width(Length::Units(250))
            .height(Length::Units(300))
            .align_items(Alignment::Center)
            .padding(0)
        //.push(Row::new().spacing(10).push(draw_button))
    }

    fn menu_options() -> Row<'a, PageMessage> {

        //let draw_button = Button::new(&mut draw_button, 
        //    Text::new("Draw").horizontal_alignment(Horizontal::Center)).on_press(
        //        PageMessage::DrawButtonPressed).width(Length::Units(120));
        Row::new()
            .spacing(20)
            .width(Length::Units(250))
            .height(Length::Units(300))
            .align_items(Alignment::Center)
            .padding(0)
                //.push(Row::new().spacing(10).push(draw_button))
    }

    //Iteration page, where the algorithm ist displayed stepwise
    fn iteration() -> Column<'a, PageMessage> {
        //variables
        
        //Layout
        Self::container()
        

    }
    //Ending page for displaying the resulting triangulation of the polygon as well as a optional comparison to other triangulations in quality
    fn result() -> Column<'a, PageMessage> {
        //variables
        
        //Layout
        Self::container()
        
    }
    
    
}

//Option enums for the Menu Page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    EarClipping,
    DelaunyTriangulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    Random,
    BiggestArea,
    BiggestMinAngle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Option {
    EdgeSwapping,

}
//Helping implementations to make Strings from enum type and make a list of all enum types for Algorithm, Heuristic and Option
impl Algorithm {
    fn all() -> [Algorithm; 2] {
        [
            Algorithm::EarClipping,
            Algorithm::DelaunyTriangulation,
        ]
    }
}

impl From<Algorithm> for String {
    fn from(algorithm: Algorithm) -> String {
        String::from(match algorithm {
            Algorithm::EarClipping => "Ear Clipping",
            Algorithm::DelaunyTriangulation => "Delauny Triangulation",
        })
    }
}

impl Heuristic {
    fn all() -> [Heuristic; 3] {
        [
            Heuristic::Random,
            Heuristic::BiggestArea,
            Heuristic::BiggestMinAngle,
        ]
    }
}

impl From<Heuristic> for String {
    fn from(heuristic: Heuristic) -> String {
        String::from(match heuristic {
            Heuristic::Random => "Random",
            Heuristic::BiggestArea => "Biggest Area",
            Heuristic::BiggestMinAngle => "Biggest Minimal Angle",
        })
    }
}

impl Option {
    fn all() -> [Option; 1] {
        [
            Option::EdgeSwapping, 
        ]
    }
}

impl From<Option> for String {
    fn from(option: Option) -> String {
        String::from(match option {
            Option::EdgeSwapping => "Edge Swapping",
        })
    }
}

//Helping function for UI-Element Creation
fn button<'a, Message: Clone>( state: &'a mut button::State, label: &str) -> Button<'a, Message>{
    Button::new(state, Text::new(label).horizontal_alignment(Horizontal::Center))
    .padding(10)
    .width(Length::Units(120))
}