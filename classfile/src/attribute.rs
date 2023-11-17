use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};

use nom::{
    combinator::map,
    multi::length_count,
    number::complete::{be_u16, be_u32, u8},
    sequence::tuple,
    IResult,
};
use nom_derive::{NomBE, Parse};

use crate::{constant_pool::ConstantPoolItem, opcode::Opcode, ValueConstant};

#[derive(NomBE)]
pub struct CodeAttributeExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub struct AttributeInfoCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: BTreeMap<u32, Opcode>,
    pub exception_table: Vec<CodeAttributeExceptionTable>,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeInfoCode {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                be_u16,
                be_u16,
                map(length_count(be_u32, u8), |x| Self::parse_code(x, constant_pool)),
                length_count(be_u16, CodeAttributeExceptionTable::parse),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            )),
            |(max_stack, max_locals, code, exception_table, attributes)| Self {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            },
        )(data)
    }

    fn parse_code(code: Vec<u8>, constant_pool: &[ConstantPoolItem]) -> BTreeMap<u32, Opcode> {
        let mut result = BTreeMap::new();

        let mut data = code.as_slice();
        while let Ok((remaining, opcode)) = Opcode::parse(data, constant_pool) {
            let offset = unsafe { data.as_ptr().offset_from(code.as_slice().as_ptr()) } as u32;
            result.insert(offset, opcode);

            data = remaining;
        }

        result
    }
}

#[derive(NomBE)]
pub struct AttributeInfoLineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

pub enum AttributeInfo {
    ConstantValue(ValueConstant),
    Code(AttributeInfoCode),
    Exceptions,
    InnerClasses,
    Synthetic,
    SourceFile(Rc<String>),
    SourceDebugExtension,
    LineNumberTable(Vec<AttributeInfoLineNumberTableEntry>),
    LocalVariableTable,
}

impl AttributeInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(
            tuple((map(be_u16, |x| constant_pool[x as usize - 1].utf8()), length_count(be_u32, u8))),
            |(name, info)| match name.as_str() {
                "ConstantValue" => AttributeInfo::ConstantValue(Self::parse_constant_value(&info, constant_pool).unwrap().1),
                "Code" => AttributeInfo::Code(AttributeInfoCode::parse(&info, constant_pool).unwrap().1),
                "LineNumberTable" => AttributeInfo::LineNumberTable(length_count(be_u16, AttributeInfoLineNumberTableEntry::parse)(&info).unwrap().1),
                "SourceFile" => AttributeInfo::SourceFile(Self::parse_source_file(&info, constant_pool).unwrap().1),
                _ => panic!("Unknown attribute {}", name),
            },
        )(data)
    }

    fn parse_source_file<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Rc<String>> {
        map(be_u16, |x| constant_pool[x as usize - 1].utf8())(data)
    }

    fn parse_constant_value<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], ValueConstant> {
        map(be_u16, |x| ValueConstant::from_constant_pool(constant_pool, x as _))(data)
    }
}
