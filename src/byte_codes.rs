// "Keywords" used in bytecode files
// All start with BC_, which stands for byte code
pub const BC_NULL: u8 = 0;
pub const BC_OPEN_BLOCK: u8 = 1;
pub const BC_CLOSE_BLOCK: u8 = 2;
pub const BC_EQUALS: u8 = 3;
pub const BC_NOT: u8 = 4;
pub const BC_AND: u8 = 5;
pub const BC_OR: u8 = 6;
pub const BC_OPEN_PARENTHESIS: u8 = 7;
pub const BC_CLOSE_PARENTHESIS: u8 = 8;
pub const BC_ADD: u8 = 9;
pub const BC_SUBTRACT: u8 = 10;
pub const BC_MULTIPLY: u8 = 11;
pub const BC_DIVIDE: u8 = 12;
pub const BC_MODULO: u8 = 13;
pub const BC_NEGATIVE: u8 = 14;
pub const BC_DECLARE: u8 = 15;
pub const BC_IF: u8 = 16;
pub const BC_ELSE: u8 = 17;
pub const BC_CALL: u8 = 18;
pub const BC_SET: u8 = 19;
pub const BC_END_STATEMENT: u8 = 20;
pub const BC_TO_THE_POWER_OF: u8 = 21;
pub const BC_ARRAY_OF: u8 = 22;
pub const BC_PUBLIC: u8 = 23;
pub const BC_INTERNAL: u8 = 24;
pub const BC_EXTERNAL: u8 = 25;
pub const BC_LABEL: u8 = 26;
pub const BC_GO_TO_LABEL: u8 = 27;
pub const BC_FOR: u8 = 28;
pub const BC_IN: u8 = 29;
pub const BC_ARRAY_INDEX: u8 = 30;
pub const BC_MODULE: u8 = 31;
pub const BC_OPEN_STRING: u8 = 32;
pub const BC_CLOSE_STRING: u8 = 33;
pub const BC_GREATER_THAN: u8 = 34;
pub const BC_LESS_THAN: u8 = 35;
pub const BC_GREATER_OR_EQUAL: u8 = 36;
pub const BC_LESS_OR_EQUAL: u8 = 37;
pub const BC_RETURN: u8 = 38;
pub const BC_FUNCTION: u8 = 39;
pub const BC_ROCHE_FILE_HEADER: u8 = 40;
pub const BC_ROCHE_FILE_BODY: u8 = 41;
pub const BC_PARAMETER: u8 = 42;
pub const BC_IMPORT: u8 = 43;