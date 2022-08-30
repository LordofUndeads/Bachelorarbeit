use iced::{
    alignment, Alignment, button, Button, 
    Column, Container,  Element, Length, Radio, Row, Sandbox,
     Settings, Space, Text, Toggler, Rule,
};



pub struct Loader;

impl Loader {
    pub fn init_gui() -> iced::Result {
        Gui::run(Settings::default())
    }
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
           
            Message::PageMessage(step_msg) => {
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
            //.push(controls)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .center_y()
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    
    PageMessage(PageMessage),
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
                    // algorithm: None, 
                    // heuristic: None, 
                    // algopt: None,
                    tools: Tools::new(),
                    progset: ProgramSettings::new(),
                },
                Page::Iteration,
                Page::Result,
                // Page::Welcome,
                // Page::Radio { selection: None },
                // Page::Toggler {
                //     can_continue: false,
                // },
                // Page::End,
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

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

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
    },
    Iteration,
    Result,

}

#[derive(Debug, Clone)]
pub enum PageMessage {
    // BackPressed,
    // NextPressed,
    AlgorithmSelected(Algorithm),
    HeuristicSelected(Heuristic),
    EdgeSwappingToggled(bool),
    StepTrigToggled(bool),
    DrawPressed,
    DrawHolePressed,
    EraserPressed,
    ClearPressed,
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

            }

            PageMessage::DrawHolePressed => {

            }

            PageMessage::ClearPressed => {

            }

            PageMessage::EraserPressed => {

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
            Page::Menu { progset, .. } => true
                // *algorithm != None && *heuristic != None
                //&& algopt != None
                ,
            Page::Iteration => true,
            Page::Result => false,

        }
    }

    fn view(&mut self) -> Element<PageMessage> {
        match self {
            Page::Menu { tools, progset} =>
                Self::menu(tools, progset),

            Page::Iteration => Self::iteration(),
            Page::Result => Self::result(),

            // Page::Toggler { can_continue } => Self::toggler(*can_continue),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, PageMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }


    


    fn menu(tools: &'a mut Tools, progset: &'a mut ProgramSettings) -> Column<'a, PageMessage> {
        
        
        let tool_menu: Column<PageMessage> = Column::new() 
                                                .width(Length::Fill).height(Length::Units(350))
                                                    .push(Row::new()
                                                        .push(Rule::vertical(2).style(style::Rule::Ligth))
                                                        .push(Tools::tool_menu(tools))
                                                        .push(Rule::vertical(2).style(style::Rule::Ligth)));
        let draw_panel: Column<PageMessage> = Column::new() ;
        let setting_menu: Row<PageMessage> = ProgramSettings::prog_settings(progset);
        Self::container("")
        .max_width(1280)
        .max_height(720)
        .spacing(0)
        .push(tool_menu)
        
        .push(draw_panel)
        .push(setting_menu)
            
          
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

//Struct for Tools in Menu
#[derive(Debug, Clone)]
pub struct Tools {
    draw_button:        button::State,
    draw_active:        bool,
    draw_hole_button:   button::State,
    draw_hole_active:   bool,
    clear_button:       button::State,
    clear_active:       bool,
    eraser_button:      button::State,
    eraser_active:      bool,
}

impl<'a> Tools {
    fn new() -> Self {
        Tools {
            draw_button: button::State::new(),
            draw_active: false, //true if button clicked
            draw_hole_button: button::State::new(),
            draw_hole_active: false, //true if button clicked
            clear_button: button::State::new(),
            clear_active: false, //only true if content is available
            eraser_button: button::State::new(),
            eraser_active: false //true if button clicked
        }
    }
    
    fn tool_menu(tools: &'a mut Tools) -> Column<'a, PageMessage>{

        //defining buttons with active and inactive style
        let mut button_d = Column::new();
        if tools.draw_active {
            button_d = button_d.push(button(&mut tools.draw_button, "Draw"))
        }
        else {
            button_d = button_d.push(button(&mut tools.draw_button, "Draw")
            .on_press(PageMessage::DrawPressed)
            .style(style::Button::Secondary))
        };

        let mut button_dh = Column::new();
        if tools.draw_hole_active {
            button_dh = button_dh.push(button(&mut tools.draw_hole_button, "Draw Hole"))
        }
        else {
            button_dh = button_dh.push(button(&mut tools.draw_hole_button, "Draw Hole")
            .on_press(PageMessage::DrawHolePressed)
            .style(style::Button::Secondary))
        };

        
        let mut button_c = Column::new();
        if !tools.clear_active {
            button_c = button_c.push(button(&mut tools.clear_button, "Clear"))
        }
        else {
            button_c = button_c.push(button(&mut tools.clear_button, "Clear")
            .on_press(PageMessage::ClearPressed)
            .style(style::Button::Secondary))

        };

        let mut button_e = Column::new();
        if tools.eraser_active {
            button_e = button_e.push(button(&mut tools.eraser_button, "Eraser"))
        }
        else {
            button_e = button_e.push(button(&mut tools.eraser_button, "Eraser")
            .on_press(PageMessage::EraserPressed)
            .style(style::Button::Secondary))
            
        };

        //the actual layout of the tool menu
        Column::new()
            .spacing(20)
            .width(Length::Units(300))
            .height(Length::Units(300))
            .align_items(Alignment::Center)
            .padding(0)
            .push(Rule::horizontal(2).style(style::Rule::Ligth))
            .push(Text::new("Drawing Tools").size(25))
            .push(Row::new()
                .spacing(10)
                .padding(10)
                .push(button_d)
                .push(button_dh)
               
            )
            .push(Row::new().spacing(10)
                .padding(10)
                .push(button_e)
                .push(button_c)
            
            )
            .push(Space::with_height(Length::Fill))

    }
}

#[derive(Debug, Clone)]
struct ProgramSettings {
    algorithm: Option<Algorithm>,
    heuristic: Option<Heuristic>,

    bools: ProgramSettingsBools,
}
#[derive(Debug, Clone)]
struct ProgramSettingsBools {
    edgeswapping_active: bool,
    stepwise_active: bool,
}

impl<'a> ProgramSettingsBools {
    fn new() -> Self {
        ProgramSettingsBools { edgeswapping_active: false, stepwise_active: true }
    }
}

impl<'a> ProgramSettings{
    fn new() -> Self {
        ProgramSettings {
            algorithm: Some(Algorithm::EarClipping),
            heuristic: Some(Heuristic::Random),
            bools: ProgramSettingsBools::new()
        }
    }

    fn prog_settings(program_settings: &'a mut ProgramSettings) -> Row<'a, PageMessage>{

        Row::new().max_height(250)
            .push(Rule::vertical(2).style(style::Rule::Ligth))

            .push(Self::alg_select(program_settings.algorithm))
            .push(Rule::vertical(2).style(style::Rule::Ligth))

            .push(Self::heur_select(program_settings.heuristic))
            .push(Rule::vertical(2).style(style::Rule::Ligth))

            .push(Self::algopt_select(&mut program_settings.bools))
            .push(Rule::vertical(2).style(style::Rule::Ligth))
    }

    fn alg_select(selection: Option<Algorithm>) -> Column<'a, PageMessage>{
        let alg_choices = Column::new()
                .padding(0)
                .spacing(10)
                .width(Length::Units(300))
                .push(Rule::horizontal(2).style(style::Rule::Ligth))
                .push(Text::new("   Algorithmus").size(24))
                .push(Algorithm::all().iter().cloned().fold(
                                Column::new().padding(10).spacing(20),
                                |choices, algorithm| {
                                    choices.push(Radio::new(
                                        algorithm,
                                        algorithm,
                                        selection,
                                        PageMessage::AlgorithmSelected,
                                    ))
                                },
                            ))
                .push(Space::with_height(Length::Fill))
                .push(Rule::horizontal(2).style(style::Rule::Ligth));
                            
        Column::new()
          .push(alg_choices)
          
    }

    fn heur_select(selection: Option<Heuristic>) -> Column<'a, PageMessage>{
        let heur_choices = Column::new()
                .padding(0)
                .spacing(10)
                .width(Length::Units(300))
                .push(Rule::horizontal(2).style(style::Rule::Ligth))
                .push(Text::new("   Heuristic").size(24))
                .push(Heuristic::all().iter().cloned().fold(
                                Column::new().padding(10).spacing(20),
                                |choices, heuristic| {
                                    choices.push(Radio::new(
                                        heuristic,
                                        heuristic,
                                        selection,
                                        PageMessage::HeuristicSelected,
                                    ))
                                },
                            ))
                .push(Space::with_height(Length::Fill))
                .push(Rule::horizontal(2).style(style::Rule::Ligth));

        Column::new()
          .push(heur_choices)
          
    }

    fn algopt_select(bools: &mut ProgramSettingsBools) -> Column<'a, PageMessage>{
        let algopt_choices = Column::new()
                .padding(0)
                .spacing(10)
                .width(Length::Units(300))
                .push(Rule::horizontal(2).style(style::Rule::Ligth))
                .push(Row::new()
                    .push(Text::new("   Options").size(24)))
                .push(Row::new()
                    .push(Column::new().width(Length::FillPortion(5)).padding(10).spacing(10)
                         
                    //Toggler for EdgeSwapping
                        .push(Toggler::new(
                    bools.edgeswapping_active,
                        String::from("Edge Swapping"),
                            PageMessage::EdgeSwappingToggled,
                            ))
                    //Toggler for StepWiseTriangulation
                        .push(Toggler::new(
                    bools.stepwise_active,
                        String::from("Stepwise Triangulation"),
                            PageMessage::StepTrigToggled,
                            ))
                        )
                    .push(Column::new().width(Length::FillPortion(2))
                        .push(Space::with_width(Length::FillPortion(2))))

                .push(Space::with_height(Length::Fill)))
                
                .push(Rule::horizontal(2).style(style::Rule::Ligth));

        Column::new()
          .push(algopt_choices)
          
    }
}

fn button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(12)
    .width(Length::Units(120))
}

mod style {
    use iced::button;
    use iced::{Background, Color, Vector};
    use iced::rule;

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }

        
    }

    pub enum Rule {
        Ligth,
        Dark,
    }

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: match self {
                    Rule::Ligth => Color::from_rgb(0.5, 0.5, 0.5),
                    Rule::Dark => Color::from_rgb(0.5, 0.5, 0.5)},
                ..rule::Style::default()
            }

        }
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
pub enum AlgOption {
    EdgeSwapping,

}


//Helping implementations to make Strings from enum type and make a list of all enum types for Algorithm, Heuristic and Option
impl<'a> Algorithm {
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

impl AlgOption {
    fn all() -> [AlgOption; 1] {
        [
            AlgOption::EdgeSwapping, 
        ]
    }
}

impl From<AlgOption> for String {
    fn from(algopt: AlgOption) -> String {
        String::from(match algopt {
            AlgOption::EdgeSwapping => "Edge Swapping",
        })
    }
}