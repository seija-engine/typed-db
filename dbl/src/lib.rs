mod parser;
use std::sync::Arc;
use smol_str::SmolStr;

pub enum BuiltinType {
    String,
    Byte,
    Char,
    Int,
    Float,
    Array
}

type ArcType = Arc<Type>;
pub enum Type {
    Builtin(BuiltinType),
    Class(ClassType),
    Enum(EnumType),
    Generic(GenericType),
    TypeIdent(SmolStr),
    Meta(MetaValue)
}

pub struct ClassType {
    pub name:SmolStr,
    pub field_list:Vec<FieldInfo>
}

pub struct FieldInfo {
  pub name:SmolStr,
  pub typ:ArcType
}

pub struct EnumType {
    pub name:SmolStr,
    list:Vec<EnumItem>
}

pub struct EnumItem {
    pub name:SmolStr,
    field_list:Vec<FieldInfo>
}

pub struct GenericType {
    typ:ArcType
}

pub enum MetaValue {
    Int(i32),
    String(String),
    Bool(bool),
    Array(ArcType,Vec<MetaValue>),
    Class(MetaClass),
    Enum(MetaEnum)
}

pub struct MetaClass {
    typ:ArcType,
    props:Vec<MetaValue>
}

pub struct MetaEnum {
    typ:ArcType,
    tag:u32,
    props:Vec<MetaValue>   
}