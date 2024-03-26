use std::collections::BTreeMap;
use crate::byte_codes::*;
use crate::keywords::*;

// Syntax operators, such as *, =, and ~
const OPERATORS: [u8; 31] = [
    033,064,035,036,037,094,038,042,040,
    041,043,045,061,123,125,091,093,124,092,
    058,034,059,039,060,062,063,044,046,047,126,096
];

// Splits into keywords, references, and operators
fn split_into_symbols(code: Vec<u8>) -> Vec<Vec<u8>> {
    let mut unsorted_code: Vec<u8> = code.clone();
    let mut symbols: Vec<Vec<u8>> = Vec::new();
    let mut i: u16 = 0;
    let mut prev_byte: u8 = 0;
    for byte in code {
        if unsorted_code.len() == 0 {break}
        if (byte == 10 || byte == 13 || byte == 9 || byte == 32)
            && !(prev_byte == 10 || prev_byte == 13 || prev_byte == 9 || prev_byte == 32)
            || (OPERATORS.contains(&prev_byte) && i == 1_u16) {
            push_as_trimmed(i, &unsorted_code, &mut symbols);
            unsorted_code = shift_vec_left(unsorted_code, i as usize);
            i = 0;
        }
        i += 1;
        prev_byte = byte;
    }
    push_as_trimmed(i, &unsorted_code, &mut symbols);
    return symbols;
}

// Removes all spaces from a symbol then pushes it onto symbols, and/or pushes an operator if found
fn push_as_trimmed(i: u16, unsorted_code: &Vec<u8>, symbols: &mut Vec<Vec<u8>>) {
    let mut addition: Vec<u8> = Vec::new();
    let mut operators_found: Vec<u8> = Vec::new();
    for j in 0..(i as usize) {
        if !(unsorted_code[j] == 10 || unsorted_code[j] == 13 || unsorted_code[j] == 9 || unsorted_code[j] == 32) {
            if OPERATORS.contains(&unsorted_code[j]) {
                operators_found.push(unsorted_code[j]);
            } else {
                addition.push(unsorted_code[j]);
            }
        }
    }
    if addition.len() != 0 {
        symbols.push(addition);
    }
    for o in operators_found {
        symbols.push(vec![o]);
    }
}

// removes a certain number of elements from the left of an array, and slides the other elements over in their place
fn shift_vec_left(v: Vec<u8>, i: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for j in 0..v.len()-i {
        result.push(v[j+i]);
    }
    return result.into_iter().collect();
}

pub fn generate_bc_from_src(src: Vec<u8>) -> Vec<u8> {
    let symbol_vec = split_into_symbols(src);
    let mut bc_header: Vec<u8> = Vec::new();
    let mut bc_body: Vec<u8> = Vec::new();
    bc_body.push(BC_ROCHE_FILE_BODY);
    bc_header.push(BC_ROCHE_FILE_HEADER);
    let mut src_references: BTreeMap<String, u16> = BTreeMap::new(); // keeps track of what references in the source file code for what reference ID
    let mut reference_num: u16 = 0;
    // generates placeholder bytes for header
    for (i, symbol) in symbol_vec.iter().enumerate() {
        if symbol == &KW_FUNCTION_DECLARE.to_vec() {
            reference_num += 1;
            src_references.insert(String::from_utf8(symbol.clone()).unwrap(), reference_num);
            bc_header.push(BC_FUNCTION);
            push_u16(reference_num, &mut bc_header);
            push_u16(0, &mut bc_header); // pushes blank bytes as placeholders for bytecode index and return type reference ID
            push_u16(0, &mut bc_header);
            push_u16(0, &mut bc_header);
            /*for j in i..symbol_vec.len() {
                if symbol_vec[j] == "!".to_string().into_bytes() {
                    bc_header.push(0);
                    break
                } else if &symbol_vec[j] == &"-".to_string().into_bytes() && &symbol_vec[j+1] == &">".to_string().into_bytes() {

                }
            }*/
        }
    }
    // generates body
    for (i, symbol) in symbol_vec.iter().enumerate() {
        
    }
    let mut bytecode: Vec<u8> = Vec::new();
    bytecode.append(&mut bc_header);
    bytecode.append(&mut bc_body);
    return bytecode;
}

fn push_u16(n: u16, v: &mut Vec<u8>) {
    let bytes: [u8; 2] = n.to_be_bytes();
    for b in bytes {
        v.push(b);
    }
}