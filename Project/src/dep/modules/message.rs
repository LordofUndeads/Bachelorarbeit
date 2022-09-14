
use super::{algorithm::Algorithm, heuristic::Heuristic, geometry::Line};

#[derive(Debug, Clone)]
pub enum Message {
    
    PageMessage(PageMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageMessage {
    PreviousPressed,
    NextPressed,
    AlgorithmSelected(Algorithm),
    HeuristicSelected(Heuristic),
    EdgeSwappingToggled(bool),
    StepTrigToggled(bool),
    DrawPressed,
    DrawHolePressed,
    UndoPressed,
    RedoPressed,
    ClearPressed, //Opens Popup
    PopUpClosed, //Closes PopUp
    AddLine(Line),
    ConfirmPressed,
    ClearAll,
    RejectClear,
    DarkModeToggled(bool),
 
}
