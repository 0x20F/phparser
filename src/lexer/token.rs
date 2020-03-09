pub enum Token {
    Namespace(u64, String),
    Import(u64, String), // Will probably need support for PHP7 soon

    ClassStart(u64),
    ClassName(u64, String),
    ClassEnd(u64),

    MethodStart(u64),
    MethodName(u64, String),
    MethodPrivacy(u64, String),
    MethodEnd(u64),

    FunctionStart(u64),
    FunctionName(u64, String),
    FunctionPrivacy(u64, String),
    FunctionEnd(u64)
}