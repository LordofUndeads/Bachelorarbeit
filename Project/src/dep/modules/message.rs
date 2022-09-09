
use super::{algorithm::Algorithm, heuristic::Heuristic, geometry::Line};

#[derive(Debug, Clone)]
pub enum Message {
    
    PageMessage(PageMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageMessage {
    // BackPressed,
    // NextPressed,
    AlgorithmSelected(Algorithm),
    HeuristicSelected(Heuristic),
    EdgeSwappingToggled(bool),
    StepTrigToggled(bool),
    DrawPressed,
    DrawHolePressed,
    UndoPressed,
    RedoPressed,
    ClearPressed,
    AddLine(Line),
    ConfirmPressed,
 
}