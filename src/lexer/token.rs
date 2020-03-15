pub enum Token {
    Namespace(String),
    Import(String), // Will probably need support for PHP7 soon

    ClassStart,
    ClassName(String),
    ClassEnd,

    FunctionStart,
    FunctionName(String),
    FunctionPrivacy(Option<String>),
    FunctionEnd
}