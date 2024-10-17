#[derive(Debug)]
pub enum ErrorTypes {
    Div0,       // "#DIV/0!"
    Name,       // "#NAME?"
    NA,         // "#N/A"
    Null,       // "#NULL!"
    Num,        // "#NUM!"
    Ref,        // "#REF!"
    Value,      // "#VALUE!"
    GettingData // "#GETTING_DATA"
}

impl ErrorTypes {
   pub fn to_string(&self) -> &str {
        match self {
            ErrorTypes::Div0 => "#DIV/0!",
            ErrorTypes::Name => "#NAME?",
            ErrorTypes::NA => "#N/A",
            ErrorTypes::Null => "#NULL!",
            ErrorTypes::Num => "#NUM!",
            ErrorTypes::Ref => "#REF!",
            ErrorTypes::Value => "#VALUE!",
            ErrorTypes::GettingData => "#GETTING_DATA",
        }
    }
}
