
use super::{algorithm::Algorithm, heuristic::Heuristic};
use iced::Point;

#[derive(Debug, Clone)]
pub enum Message {
    PageMessage(PageMessage),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PageMessage {
    //Messegaes needed for inteactions on the menu page
    
    //options
    AlgorithmSelected(Algorithm),
    HeuristicSelected(Heuristic),
    EdgeSwappingToggled(bool),
    StepTrigToggled(bool),
    DarkModeToggled(bool),

    //Drawing tools
    DrawPressed,
    DrawHolePressed,
    UndoPressed,
    RedoPressed,
    ClearPressed, //Opens Popup
    PopUpClosed, //Closes PopUp
    AddPoint(Point),
    ConfirmPressed,
    ClearAll,
    RejectClear,
    
    //Messages for interactions on the iteratiuon page
    PreviousPressed,
    NextPressed,
    EndPressed,

    //Messages for interactions on the result page
    ExitPressed,
    RepeatPressed,
}
