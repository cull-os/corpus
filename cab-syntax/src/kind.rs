use derive_more::Display;
use Kind::*;

/// derive_more causes [`unreachable`] to warn too many times
/// so we're just suppressing them like this. No, #[allow(unreachable_code)]
/// doesn't suppress the ones coming from the #[derive(Display)].
fn reachable_unreachable() -> &'static str {
    unreachable!()
}

/// The Cab syntax kind.
#[derive(
    Display,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    enumset::EnumSetType,
    num_enum::TryFromPrimitive,
)]
#[enumset(no_super_impls)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Kind {
    /// Represents any sequence of tokens that was not recognized.
    #[display("an unknown token sequence")]
    TOKEN_ERROR,

    /// Anything that matches [`char::is_whitespace`].
    #[display("whitespace")]
    TOKEN_WHITESPACE,

    /// Anything that starts with a `#`.
    ///
    /// When the comment starts with more than 3 `#` characters, it will be
    /// multiline. Multiline comments can be closed with the initial delimiter,
    /// but they don't have to be.
    #[display("a comment")]
    TOKEN_COMMENT, // #[^\r\n]* and (#{3,}).*\1

    #[display("'<|'")]
    TOKEN_LESS_PIPE,
    #[display("'|>'")]
    TOKEN_PIPE_MORE,

    #[display("'@'")]
    TOKEN_AT,

    #[display("'('")]
    TOKEN_LEFT_PARENTHESIS,
    #[display("')'")]
    TOKEN_RIGHT_PARENTHESIS,

    #[display("'++'")]
    TOKEN_PLUS_PLUS,
    #[display("'['")]
    TOKEN_LEFT_BRACKET,
    #[display("']'")]
    TOKEN_RIGHT_BRACKET,

    #[display("'==>'")]
    TOKEN_EQUAL_EQUAL_MORE,
    #[display("'<=='")]
    TOKEN_LESS_EQUAL_EQUAL,
    #[display("'//'")]
    TOKEN_SLASH_SLASH,
    #[display("'.'")]
    TOKEN_PERIOD,
    #[display("'{{'")]
    TOKEN_LEFT_CURLYBRACE,
    #[display("'}}'")]
    TOKEN_RIGHT_CURLYBRACE,
    #[display("'?'")]
    TOKEN_QUESTIONMARK,
    #[display("';'")]
    TOKEN_SEMICOLON,

    #[display("'!='")]
    TOKEN_EXCLAMATION_EQUAL,
    #[display("'=='")]
    TOKEN_EQUAL_EQUAL,
    #[display("'='")]
    TOKEN_EQUAL,
    #[display("'<='")]
    TOKEN_LESS_EQUAL,
    #[display("'<'")]
    TOKEN_LESS,
    #[display("'>='")]
    TOKEN_MORE_EQUAL,
    #[display("'>'")]
    TOKEN_MORE,
    #[display("'->'")]
    TOKEN_MINUS_MORE,

    #[display("','")]
    TOKEN_COMMA,
    #[display("':'")]
    TOKEN_COLON,

    #[display("'+'")]
    TOKEN_PLUS,
    #[display("'-'")]
    TOKEN_MINUS,
    #[display("'**'")]
    TOKEN_ASTERISK_ASTERISK,
    #[display("'*'")]
    TOKEN_ASTERISK,
    #[display("'/'")]
    TOKEN_SLASH,

    #[display("an integer")]
    TOKEN_INTEGER,
    #[display("a float")]
    TOKEN_FLOAT,

    #[display("the keyword 'if'")]
    TOKEN_LITERAL_IF,
    #[display("the keyword 'then'")]
    TOKEN_LITERAL_THEN,
    #[display("the keyword 'else'")]
    TOKEN_LITERAL_ELSE,

    #[display("the keyword 'and'")]
    TOKEN_LITERAL_AND,
    #[display("the keyword 'or'")]
    TOKEN_LITERAL_OR,
    #[display("the keyword 'not'")]
    TOKEN_LITERAL_NOT,

    #[display(r"'\('")]
    TOKEN_INTERPOLATION_START,
    #[display("')'")]
    TOKEN_INTERPOLATION_END,

    /// A path. Valid paths start with `./`, `..` or `/`, followed by
    /// characters that are either [alphanumeric](char::is_alphanumeric) or
    /// are any of the following characters: `.`, `/`, `_`, `-`, `\`.
    ///
    /// The `\` character can be used to escape characters that are normally
    /// not allowed in paths, like spaces and other weird characters.
    /// It is also useful to escape `\(` literally, to not begin string
    /// interpolation like so: `./\\\(foo`
    ///
    /// Every path part will be represented using this kind, so a path node with
    /// interpolation will be represented as the following:
    ///
    /// ```txt
    /// ./foo\(bar)baz -- TOKEN_PATH
    /// +---/\|\|/\-- TOKEN_INTERPOLATION_END
    /// |     | +-- TOKEN_IDENTIFIER
    /// |     +-- TOKEN_INTERPOLATION_START
    /// +-- TOKEN_PATH
    /// ```
    #[display("a path")]
    TOKEN_PATH,

    #[display("content")]
    TOKEN_CONTENT,

    /// A normal non-quoted identifier. The initial character must not match
    /// [`char::is_ascii_digit`], the other characters must be either
    /// [`char::is_alphanumeric`], `_` or `-`.
    #[display("an identifier")]
    TOKEN_IDENTIFIER,

    #[display("an identifier")]
    TOKEN_IDENTIFIER_START,
    #[display("the end of an identifier")]
    TOKEN_IDENTIFIER_END,

    #[display("a string")]
    TOKEN_STRING_START,
    #[display("the end of a string")]
    TOKEN_STRING_END,

    #[display("an island")]
    TOKEN_ISLAND_START,
    #[display("the end of an island")]
    TOKEN_ISLAND_END,

    #[display("{}", reachable_unreachable())]
    NODE_ROOT,
    #[display("an erroneous expression")]
    NODE_ERROR,

    #[display("a function application")]
    NODE_APPLICATION,

    #[display("a prefix operation")]
    NODE_PREFIX_OPERATION,
    #[display("an infix operation")]
    NODE_INFIX_OPERATION,

    #[display("a parenthesized expression")]
    NODE_PARENTHESIS,

    #[display("a list")]
    NODE_LIST,

    #[display("an attribute set")]
    NODE_ATTRIBUTE_SET,
    #[display("{}", reachable_unreachable())]
    NODE_ATTRIBUTE,
    #[display("{}", reachable_unreachable())]
    NODE_ATTRIBUTE_PATH,
    #[display("{}", reachable_unreachable())]
    NODE_ATTRIBUTE_INHERIT,

    #[display("an attribute select")]
    NODE_ATTRIBUTE_SELECT,
    #[display("an attribute check")]
    NODE_ATTRIBUTE_CHECK,

    #[display("a bind expression")]
    NODE_BIND,

    #[display("a lambda")]
    NODE_LAMBDA,
    #[display("{}", reachable_unreachable())]
    NODE_LAMBDA_PARAMETER_IDENTIFIER,
    #[display("{}", reachable_unreachable())]
    NODE_LAMBDA_PARAMETER_PATTERN,
    #[display("{}", reachable_unreachable())]
    NODE_LAMBDA_PARAMETER_PATTERN_ATTRIBUTE,

    /// A node which starts with a [`TOKEN_INTERPOLATION_START`], ends with a
    /// [`TOKEN_INTERPOLATION_END`] while having a node at the middle that can
    /// be cast to an [Expression](crate::node::Expression)
    #[display("{}", reachable_unreachable())]
    NODE_INTERPOLATION,

    /// A node that only has [`TOKEN_PATH`]s and [`NODE_INTERPOLATION`]s as its
    /// direct children without any delimiters.
    #[display("a path")]
    NODE_PATH,

    /// A stringlike that is delimited by a single backtick. See [`NODE_STRING`]
    /// for the definition of stringlike.
    #[display("an identifier")]
    NODE_IDENTIFIER,

    /// A stringlike that is delimited by a single `"` or any number of `'`.
    ///
    /// A stringlike is a sequence of nodes and tokens, where all the immediate
    /// children tokens are [`TOKEN_CONTENT`]s, while all the immediate children
    /// nodes are all [`NODE_INTERPOLATION`]s.
    #[display("a string")]
    NODE_STRING,

    /// A stringlike that is delimited by `<` and `>`. See [`NODE_STRING`] for
    /// the definition of stringlike.
    #[display("a number")]
    NODE_ISLAND,

    /// A node containing a single token, which can be either a
    /// [`TOKEN_INTEGER`] or [`TOKEN_FLOAT`].
    #[display("a number")]
    NODE_NUMBER,

    #[display("an if else")]
    NODE_IF_ELSE,
}

impl From<Kind> for rowan::SyntaxKind {
    fn from(kind: Kind) -> Self {
        Self(kind as u16)
    }
}

impl Kind {
    /// Whether if this token can be used as a function argument.
    ///
    /// ```txt
    /// max 42 (38) + 61
    ///     t  t    f
    /// ```
    pub fn is_argument(self) -> bool {
        matches!(
            self,
            TOKEN_ERROR
                | TOKEN_LEFT_PARENTHESIS
                | TOKEN_LEFT_BRACKET
                | TOKEN_LEFT_CURLYBRACE
                | TOKEN_INTEGER
                | TOKEN_FLOAT
                | TOKEN_PATH
                | TOKEN_IDENTIFIER
                | TOKEN_IDENTIFIER_START
                | TOKEN_STRING_START
                | TOKEN_ISLAND_START
        )
    }

    /// Whether if the token should be ignored by the parser.
    pub fn is_trivia(self) -> bool {
        matches!(self, TOKEN_COMMENT | TOKEN_WHITESPACE)
    }
}
