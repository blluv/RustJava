use alloc::vec::Vec;

use nom::{
    combinator::{flat_map, map, success},
    multi::length_count,
    number::complete::{be_i16, be_i32, be_u16, be_u32, u8},
    sequence::tuple,
    IResult,
};

use crate::constant_pool::{ConstantPoolItem, ReferenceConstant, ValueConstant};

#[derive(Clone, Debug)]
pub enum Opcode {
    Aaload,
    Aastore,
    AconstNull,
    Aload(u8),
    Anewarray(u16),
    Areturn,
    Arraylength,
    Astore(u8),
    Athrow,
    Baload,
    Bastore,
    Bipush(u8),
    Caload,
    Castore,
    Checkcast(u16),
    D2f,
    D2i,
    D2l,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    Dload(u8),
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore(u8),
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dsub,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload(u8),
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore(u8),
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fsub,
    Getfield(u16),
    Getstatic(ReferenceConstant),
    Goto(i16),
    GotoW(i32),
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    Iconst(i8),
    Idiv,
    IfAcmpeq(i16),
    IfAcmpne(i16),
    IfIcmpeq(i16),
    IfIcmpne(i16),
    IfIcmplt(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    Ifnonnull(i16),
    Ifnull(i16),
    Iinc(u8, u8),
    Iload(u8),
    Imul,
    Ineg,
    Instanceof(u16),
    Invokedynamic(ReferenceConstant),
    Invokeinterface(ReferenceConstant, u8, u8),
    Invokespecial(ReferenceConstant),
    Invokestatic(ReferenceConstant),
    Invokevirtual(ReferenceConstant),
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore(u8),
    Isub,
    Iushr,
    Ixor,
    Jsr(i16),
    JsrW(i32),
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc(ValueConstant),
    LdcW(ValueConstant),
    Ldc2W(ValueConstant),
    Ldiv,
    Lload(u8),
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch(i32, Vec<(i32, i32)>),
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore(u8),
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray(u16, u8),
    New(ValueConstant),
    Newarray(u8),
    Nop,
    Pop,
    Pop2,
    Putfield(u16),
    Putstatic(u16),
    Ret(u8),
    Return,
    Saload,
    Sastore,
    Sipush(u16),
    Swap,
    Tableswitch(i32, i32, i32, Vec<i32>),
    Wide,
}

impl Opcode {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        flat_map(u8, |x| move |i| Self::parse_opcode(x, i, constant_pool))(data)
    }

    fn parse_opcode<'a>(opcode: u8, data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        match opcode {
            0x32 => success(Opcode::Aaload)(data),
            0x53 => success(Opcode::Aastore)(data),
            0x01 => success(Opcode::AconstNull)(data),
            0x19 => map(u8, Opcode::Aload)(data),
            0x2a => success(Opcode::Aload(0))(data),
            0x2b => success(Opcode::Aload(1))(data),
            0x2c => success(Opcode::Aload(2))(data),
            0x2d => success(Opcode::Aload(3))(data),
            0xbd => map(be_u16, Opcode::Anewarray)(data),
            0xb0 => success(Opcode::Areturn)(data),
            0xbe => success(Opcode::Arraylength)(data),
            0x3a => map(u8, Opcode::Astore)(data),
            0x4b => success(Opcode::Astore(0))(data),
            0x4c => success(Opcode::Astore(1))(data),
            0x4d => success(Opcode::Astore(2))(data),
            0x4e => success(Opcode::Astore(3))(data),
            0xbf => success(Opcode::Athrow)(data),
            0x33 => success(Opcode::Baload)(data),
            0x54 => success(Opcode::Bastore)(data),
            0x10 => map(u8, Opcode::Bipush)(data),
            0x34 => success(Opcode::Caload)(data),
            0x55 => success(Opcode::Castore)(data),
            0xc0 => map(be_u16, Opcode::Checkcast)(data),
            0x90 => success(Opcode::D2f)(data),
            0x8e => success(Opcode::D2i)(data),
            0x8f => success(Opcode::D2l)(data),
            0x63 => success(Opcode::Dadd)(data),
            0x31 => success(Opcode::Daload)(data),
            0x52 => success(Opcode::Dastore)(data),
            0x98 => success(Opcode::Dcmpg)(data),
            0x97 => success(Opcode::Dcmpl)(data),
            0x0e => success(Opcode::Dconst0)(data),
            0x0f => success(Opcode::Dconst1)(data),
            0x6f => success(Opcode::Ddiv)(data),
            0x18 => map(u8, Opcode::Dload)(data),
            0x26 => success(Opcode::Dload0)(data),
            0x27 => success(Opcode::Dload1)(data),
            0x28 => success(Opcode::Dload2)(data),
            0x29 => success(Opcode::Dload3)(data),
            0x6b => success(Opcode::Dmul)(data),
            0x77 => success(Opcode::Dneg)(data),
            0x73 => success(Opcode::Drem)(data),
            0xaf => success(Opcode::Dreturn)(data),
            0x39 => map(u8, Opcode::Dstore)(data),
            0x47 => success(Opcode::Dstore0)(data),
            0x48 => success(Opcode::Dstore1)(data),
            0x49 => success(Opcode::Dstore2)(data),
            0x4a => success(Opcode::Dstore3)(data),
            0x67 => success(Opcode::Dsub)(data),
            0x59 => success(Opcode::Dup)(data),
            0x5a => success(Opcode::DupX1)(data),
            0x5b => success(Opcode::DupX2)(data),
            0x5c => success(Opcode::Dup2)(data),
            0x5d => success(Opcode::Dup2X1)(data),
            0x5e => success(Opcode::Dup2X2)(data),
            0x8d => success(Opcode::F2d)(data),
            0x8b => success(Opcode::F2i)(data),
            0x8c => success(Opcode::F2l)(data),
            0x62 => success(Opcode::Fadd)(data),
            0x30 => success(Opcode::Faload)(data),
            0x51 => success(Opcode::Fastore)(data),
            0x96 => success(Opcode::Fcmpg)(data),
            0x95 => success(Opcode::Fcmpl)(data),
            0x0b => success(Opcode::Fconst0)(data),
            0x0c => success(Opcode::Fconst1)(data),
            0x0d => success(Opcode::Fconst2)(data),
            0x6e => success(Opcode::Fdiv)(data),
            0x17 => map(u8, Opcode::Fload)(data),
            0x22 => success(Opcode::Fload0)(data),
            0x23 => success(Opcode::Fload1)(data),
            0x24 => success(Opcode::Fload2)(data),
            0x25 => success(Opcode::Fload3)(data),
            0x6a => success(Opcode::Fmul)(data),
            0x76 => success(Opcode::Fneg)(data),
            0x72 => success(Opcode::Frem)(data),
            0xae => success(Opcode::Freturn)(data),
            0x38 => map(u8, Opcode::Fstore)(data),
            0x43 => success(Opcode::Fstore0)(data),
            0x44 => success(Opcode::Fstore1)(data),
            0x45 => success(Opcode::Fstore2)(data),
            0x46 => success(Opcode::Fstore3)(data),
            0x66 => success(Opcode::Fsub)(data),
            0xb4 => map(be_u16, Opcode::Getfield)(data),
            0xb2 => map(be_u16, |x| {
                Opcode::Getstatic(ReferenceConstant::from_constant_pool(constant_pool, x as _))
            })(data),
            0xa7 => map(be_i16, Opcode::Goto)(data),
            0xc8 => map(be_i32, Opcode::GotoW)(data),
            0x91 => success(Opcode::I2b)(data),
            0x92 => success(Opcode::I2c)(data),
            0x87 => success(Opcode::I2d)(data),
            0x86 => success(Opcode::I2f)(data),
            0x85 => success(Opcode::I2l)(data),
            0x93 => success(Opcode::I2s)(data),
            0x60 => success(Opcode::Iadd)(data),
            0x2e => success(Opcode::Iaload)(data),
            0x7e => success(Opcode::Iand)(data),
            0x4f => success(Opcode::Iastore)(data),
            0x02 => success(Opcode::Iconst(-1))(data),
            0x03 => success(Opcode::Iconst(0))(data),
            0x04 => success(Opcode::Iconst(1))(data),
            0x05 => success(Opcode::Iconst(2))(data),
            0x06 => success(Opcode::Iconst(3))(data),
            0x07 => success(Opcode::Iconst(4))(data),
            0x08 => success(Opcode::Iconst(5))(data),
            0x6c => success(Opcode::Idiv)(data),
            0xa5 => map(be_i16, Opcode::IfAcmpeq)(data),
            0xa6 => map(be_i16, Opcode::IfAcmpne)(data),
            0x9f => map(be_i16, Opcode::IfIcmpeq)(data),
            0xa0 => map(be_i16, Opcode::IfIcmpne)(data),
            0xa1 => map(be_i16, Opcode::IfIcmplt)(data),
            0xa2 => map(be_i16, Opcode::IfIcmpge)(data),
            0xa3 => map(be_i16, Opcode::IfIcmpgt)(data),
            0xa4 => map(be_i16, Opcode::IfIcmple)(data),
            0x99 => map(be_i16, Opcode::Ifeq)(data),
            0x9a => map(be_i16, Opcode::Ifne)(data),
            0x9b => map(be_i16, Opcode::Iflt)(data),
            0x9c => map(be_i16, Opcode::Ifge)(data),
            0x9d => map(be_i16, Opcode::Ifgt)(data),
            0x9e => map(be_i16, Opcode::Ifle)(data),
            0xc7 => map(be_i16, Opcode::Ifnonnull)(data),
            0xc6 => map(be_i16, Opcode::Ifnull)(data),
            0x84 => map(tuple((u8, u8)), |(index, constant)| Opcode::Iinc(index, constant))(data),
            0x15 => map(u8, Opcode::Iload)(data),
            0x1a => success(Opcode::Iload(0))(data),
            0x1b => success(Opcode::Iload(1))(data),
            0x1c => success(Opcode::Iload(2))(data),
            0x1d => success(Opcode::Iload(3))(data),
            0x68 => success(Opcode::Imul)(data),
            0x74 => success(Opcode::Ineg)(data),
            0xc1 => map(be_u16, Opcode::Instanceof)(data),
            0xba => map(be_u16, |x| {
                Opcode::Invokedynamic(ReferenceConstant::from_constant_pool(constant_pool, x as _))
            })(data),
            0xb9 => map(be_u16, |x| {
                Opcode::Invokeinterface(ReferenceConstant::from_constant_pool(constant_pool, x as _), 0, 0)
            })(data),
            0xb7 => map(be_u16, |x| {
                Opcode::Invokespecial(ReferenceConstant::from_constant_pool(constant_pool, x as _))
            })(data),
            0xb8 => map(be_u16, |x| {
                Opcode::Invokestatic(ReferenceConstant::from_constant_pool(constant_pool, x as _))
            })(data),
            0xb6 => map(be_u16, |x| {
                Opcode::Invokevirtual(ReferenceConstant::from_constant_pool(constant_pool, x as _))
            })(data),
            0x80 => success(Opcode::Ior)(data),
            0x70 => success(Opcode::Irem)(data),
            0xac => success(Opcode::Ireturn)(data),
            0x78 => success(Opcode::Ishl)(data),
            0x7a => success(Opcode::Ishr)(data),
            0x36 => map(u8, Opcode::Istore)(data),
            0x3b => success(Opcode::Istore(0))(data),
            0x3c => success(Opcode::Istore(1))(data),
            0x3d => success(Opcode::Istore(2))(data),
            0x3e => success(Opcode::Istore(3))(data),
            0x64 => success(Opcode::Isub)(data),
            0x7c => success(Opcode::Iushr)(data),
            0x82 => success(Opcode::Ixor)(data),
            0xa8 => map(be_i16, Opcode::Jsr)(data),
            0xc9 => map(be_i32, Opcode::JsrW)(data),
            0x8a => success(Opcode::L2d)(data),
            0x89 => success(Opcode::L2f)(data),
            0x88 => success(Opcode::L2i)(data),
            0x61 => success(Opcode::Ladd)(data),
            0x2f => success(Opcode::Laload)(data),
            0x7f => success(Opcode::Land)(data),
            0x50 => success(Opcode::Lastore)(data),
            0x94 => success(Opcode::Lcmp)(data),
            0x09 => success(Opcode::Lconst0)(data),
            0x0a => success(Opcode::Lconst1)(data),
            0x12 => map(u8, |x| Opcode::Ldc(ValueConstant::from_constant_pool(constant_pool, x as _)))(data),
            0x13 => map(be_u16, |x| Opcode::LdcW(ValueConstant::from_constant_pool(constant_pool, x as _)))(data),
            0x14 => map(be_u16, |x| Opcode::Ldc2W(ValueConstant::from_constant_pool(constant_pool, x as _)))(data),
            0x6d => success(Opcode::Ldiv)(data),
            0x16 => map(u8, Opcode::Lload)(data),
            0x1e => success(Opcode::Lload0)(data),
            0x1f => success(Opcode::Lload1)(data),
            0x20 => success(Opcode::Lload2)(data),
            0x21 => success(Opcode::Lload3)(data),
            0x69 => success(Opcode::Lmul)(data),
            0x75 => success(Opcode::Lneg)(data),
            0xab => map(tuple((be_i32, length_count(be_u32, tuple((be_i32, be_i32))))), |(default, pairs)| {
                Opcode::Lookupswitch(default, pairs)
            })(data),
            0x81 => success(Opcode::Lor)(data),
            0x71 => success(Opcode::Lrem)(data),
            0xad => success(Opcode::Lreturn)(data),
            0x79 => success(Opcode::Lshl)(data),
            0x7b => success(Opcode::Lshr)(data),
            0x37 => map(u8, Opcode::Lstore)(data),
            0x3f => success(Opcode::Lstore0)(data),
            0x40 => success(Opcode::Lstore1)(data),
            0x41 => success(Opcode::Lstore2)(data),
            0x42 => success(Opcode::Lstore3)(data),
            0x65 => success(Opcode::Lsub)(data),
            0x7d => success(Opcode::Lushr)(data),
            0x83 => success(Opcode::Lxor)(data),
            0xc2 => success(Opcode::Monitorenter)(data),
            0xc3 => success(Opcode::Monitorexit)(data),
            0xc5 => map(tuple((be_u16, u8)), |(index, dimensions)| Opcode::Multianewarray(index, dimensions))(data),
            0xbb => map(be_u16, |x| Opcode::New(ValueConstant::from_constant_pool(constant_pool, x as _)))(data),
            0xbc => map(u8, Opcode::Newarray)(data),
            0x00 => success(Opcode::Nop)(data),
            0x57 => success(Opcode::Pop)(data),
            0x58 => success(Opcode::Pop2)(data),
            0xb5 => map(be_u16, Opcode::Putfield)(data),
            0xb3 => map(be_u16, Opcode::Putstatic)(data),
            0xa9 => map(u8, Opcode::Ret)(data),
            0xb1 => success(Opcode::Return)(data),
            0x35 => success(Opcode::Saload)(data),
            0x56 => success(Opcode::Sastore)(data),
            0x11 => map(be_u16, Opcode::Sipush)(data),
            0x5f => success(Opcode::Swap)(data),
            0xaa => map(
                tuple((be_i32, be_i32, be_i32, length_count(be_u32, be_i32))),
                |(default, low, high, offsets)| Opcode::Tableswitch(default, low, high, offsets),
            )(data),
            0xc4 => success(Opcode::Wide)(data),
            _ => panic!("Unknown opcode: {:02x}", opcode),
        }
    }
}
