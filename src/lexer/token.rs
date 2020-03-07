pub enum Token {
    Namespace(u64),
    Use(u64), // Will probably need support for PHP7 soon

    ClassStart(u64),
    ClassEnd(u64),

    MethodStart(u64),
    MethodEnd(u64),

    FunctionStart(u64),
    FunctionEnd(u64)
}