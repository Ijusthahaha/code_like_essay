use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    KeywordSet,          // e.g., "Set" or "Define"
    KeywordDisplay,      // e.g., "Display", "Print", or "Show"
    KeywordLoop,         // e.g., "Repeat" or "Loop"
    KeywordFunction,     // Function-related tokens
    KeywordClass,        // Class-related tokens
    KeywordIf,           // Conditionals
    KeywordElse,
    KeywordTry,          // Exception handling
    KeywordCatch,
    KeywordImport,       // Imports
    KeywordIncrement,
    KeywordDecrement,
    KeywordMultiply,
    KeywordDivide,

    ListType,
    MapType,
    List(Vec<Token>),
    Map(Vec<(Token, Token)>),

    Bool,
    NoneType,
    Identifier(String),  // variable or function names
    Number(i32),         // integer values
    Text(String),        // string values
    Boolean(bool),       // true or false
    Comment(String),     // Comments
    Group,               // represents "and" (",")
    End,                 // represents statement termination (".")
    Add,                 // For "add"
    Subtract,            // For "subtract"
    Multiply,            // For "multiply"
    Divide,              // For "divide"
    Skip,
    And,
    Or,
    Equals,
    GreaterThan,
    LessThan,
    GreaterOrEquals,
    LessOrEquals,
    NotEquals,
    AddInto,
}