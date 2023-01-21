use super::super::modules::{algorithm::Algorithm, heuristic::Heuristic, message::PageMessage,
                            style};

use iced::{Column,  Length, Radio, Row, Space, Text, Toggler, Rule,};

#[derive(Debug, Clone)]
pub struct ProgramSettings {
    pub algorithm: Option<Algorithm>,
    pub heuristic: Option<Heuristic>,
    pub bools: ProgramSettingsBools,
}
#[derive(Debug, Clone)]
pub struct ProgramSettingsBools {
    pub edgeswapping_active: bool,
    pub stepwise_active: bool,
    pub dark_mode: bool,
}

impl<'a> ProgramSettingsBools {
    pub fn new() -> Self {
        ProgramSettingsBools { edgeswapping_active: false, stepwise_active: true , dark_mode: false}
    }
}

impl<'a> ProgramSettings{
    pub fn new() -> Self {
        ProgramSettings {
            algorithm: Some(Algorithm::EarClipping),
            heuristic: Some(Heuristic::Random),
            bools: ProgramSettingsBools::new()
        }
    }

    pub fn prog_settings(program_settings: &'a mut ProgramSettings) -> Row<'a, PageMessage>{

        Row::new().max_height(200)
            .push(Rule::vertical(2).style(style::RuleStyle::Light))

            .push(Self::alg_select(program_settings.algorithm))
            .push(Rule::vertical(2).style(style::RuleStyle::Light))

            .push(Self::heur_select(program_settings.heuristic))
            .push(Rule::vertical(2).style(style::RuleStyle::Light))

            .push(Self::algopt_select(&mut program_settings.bools))
            .push(Rule::vertical(2).style(style::RuleStyle::Light))
    }

    fn alg_select(selection: Option<Algorithm>) -> Column<'a, PageMessage>{
        
        let alg_choices = Column::new()
                .padding(0)
                .spacing(10)
                .width(Length::Units(300))
                .push(Rule::horizontal(2).style(style::RuleStyle::Light))
                .push(Text::new("   Algorithmus").size(18))
                .push(Algorithm::all().iter().cloned().fold(
                                Column::new().padding(10).spacing(15),
                                |choices, algorithm| {
                                    choices.push(Radio::new(
                                        algorithm,
                                        algorithm,
                                        selection,
                                        PageMessage::AlgorithmSelected,
                                    ).text_size(20).size(15))
                                },
                            ))
                .push(Space::with_height(Length::Fill))
                .push(Rule::horizontal(2).style(style::RuleStyle::Light));
                            
        Column::new()
          .push(alg_choices)
          
    }

    fn heur_select(selection: Option<Heuristic>) -> Column<'a, PageMessage>{
        let heur_choices = Column::new()
                .padding(0)
                .spacing(10)
                .width(Length::Units(300))
                .push(Rule::horizontal(2).style(style::RuleStyle::Light))
                .push(Text::new("   Heuristic").size(18))
                .push(Heuristic::all().iter().cloned().fold(
                                Column::new().padding(10).spacing(15),
                                |choices, heuristic| {
                                    choices.push(Radio::new(
                                        heuristic,
                                        heuristic,
                                        selection,
                                        PageMessage::HeuristicSelected,
                                    ).text_size(20).size(15))
                                },
                            ))
                .push(Space::with_height(Length::Fill))
                .push(Rule::horizontal(2).style(style::RuleStyle::Light));

        Column::new()
          .push(heur_choices)
          
    }

    fn algopt_select(bools: &mut ProgramSettingsBools) -> Column<'a, PageMessage>{
        let algopt_choices = Column::new()
                .padding(0)
                .spacing(10)
                .width(Length::Units(300))
                .push(Rule::horizontal(2).style(style::RuleStyle::Light))
                .push(Row::new()
                    .push(Text::new("   Options").size(18)))
                .push(Row::new()
                    .push(Column::new().width(Length::FillPortion(5)).padding(10).spacing(10)
                         
                    //Toggler for EdgeSwapping
                        .push(Toggler::new(
                    bools.edgeswapping_active,
                        String::from("Edge Swapping"),
                            PageMessage::EdgeSwappingToggled,
                            ).size(15).text_size(20))
                    //Toggler for StepWiseTriangulation
                        .push(Toggler::new(
                    bools.stepwise_active,
                        String::from("Stepwise Triangulation"),
                            PageMessage::StepTrigToggled,
                            ).size(15).text_size(20))
                    //Toggler for Dark Mode
                        .push(Space::with_height(Length::Fill))
                        .push(Toggler::new(
                        bools.dark_mode,
                                String::from("Dark Mode"),
                                    PageMessage::DarkModeToggled,
                                    ).size(15).text_size(20))
                        )
                    
                    .push(Column::new().width(Length::FillPortion(2))
                        .push(Space::with_width(Length::FillPortion(2))))

                .push(Space::with_height(Length::Fill)))
                
                .push(Rule::horizontal(2).style(style::RuleStyle::Light));

        Column::new()
          .push(algopt_choices)
          
    }
}

