#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType{
    // 布尔表达式
    Or,
    And,
    Not,    // (后置)非：'
    PreNot,   // 前置非：！

    // 赋值
    Eq,

    // 括号
    LeftParen,
    RightParen,

    True,
    False,
    Num,

    Identifier,

    Eof,
    Semicolon,
}