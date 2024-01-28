// Identifier related
pub const UNDERSCORE: char = '_';

// Calling conventions
pub const LPAREN: char = '(';
pub const RPAREN: char = ')';

// Scope control
pub const OPEN_BRACE: char = '{';
pub const CLOSE_BRACE: char = '}';

// Statement terminators
pub const SEMI: char = ';';

// Reserved for assignment operations
pub const EQUALS: char = '=';
pub const LET: &'static str = "let";

// Reserved for arithmetic operations
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const ASTERISK: char = '*';
pub const SLASH: char = '/';
pub const PERCENT: char = '%';

// Reserved for comparison operations
pub const NOT_EQUALS: &'static str = "!=";
pub const EQUALS_EQUALS: &'static str = "==";
pub const LESS_THAN: char = '<';
pub const GREATER_THAN: char = '>';
pub const NOT: &'static str = "not";
pub const OR: &'static str = "or";
pub const AND: &'static str = "and";

// Bitwise operations
pub const BITWISE_OR: char = '|';
pub const BITWISE_AND: char = '&';
pub const BITWISE_XOR: char = '^';
pub const BITWISE_NOT: char = '~';
pub const BITWISE_LEFT_SHIFT: &'static str = "<<";
pub const BITWISE_RIGHT_SHIFT: &'static str = ">>";

// Reserved kwds for special tokens
pub const STRUCT: &'static str = "struct";
pub const FN: &'static str = "fn";
pub const ASYNC: &'static str = "async"; // TODO: Implement async functions
