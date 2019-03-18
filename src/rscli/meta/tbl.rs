use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::Sized;

use crate::BinaryReader;
use crate::rscli::meta::CLITildeStream;


#[derive(Eq, Debug, PartialEq, Hash,Copy, Clone)]
pub enum CLIColumnType {
    TypeDefOrRef = 0,
    HasConstant = 1,
    HasCustomAttribute = 2,
    HasFieldMarshall = 3,
    HasDeclSecurity = 4,
    MemberRefParent = 5,
    HasSemantics = 6,
    MethodDefOrRef = 7,
    MemberForwarded = 8,
    Implementation = 9,
    CustomAttributeType = 10,
    ResolutionScope = 11,
    TypeOrMethodDef = 12,
}

lazy_static!{
    pub static ref CLIColumnMap:HashMap<CLIColumnType,Vec<CLITableId>> = {
        let mut m:HashMap<CLIColumnType,Vec<CLITableId>> = HashMap::new();
        m.insert(CLIColumnType::TypeDefOrRef,vec![
            CLITableId::TypeDef,
            CLITableId::TypeRef,
            CLITableId::TypeSpec,
        ]);
        m.insert(CLIColumnType::HasConstant,vec![
            CLITableId::Field,
            CLITableId::Param,
            CLITableId::Property,
        ]);
        m.insert(CLIColumnType::HasCustomAttribute,vec![
            CLITableId::MethodDef,
            CLITableId::Field,
            CLITableId::TypeRef,
            CLITableId::TypeDef,
            CLITableId::Param,
            CLITableId::InterfaceImpl,
            CLITableId::MemberRef,
            CLITableId::Module,

            CLITableId::Property,
            CLITableId::Event,
            CLITableId::StandAloneSig,
            CLITableId::ModuleRef,
            CLITableId::TypeSpec,
            CLITableId::Assembly,
            CLITableId::AssemblyRef,
            CLITableId::File,
            CLITableId::ExportedType,
            CLITableId::ManifestResource,
            CLITableId::GenericParam,
            CLITableId::GenericParamConstraint,
            CLITableId::MethodSpec,
            CLITableId::Invalid,//CLITableId::Permission
        ]);
        m.insert(CLIColumnType::HasFieldMarshall,vec![
            CLITableId::Field,
            CLITableId::Param,
        ]);
        m.insert(CLIColumnType::HasDeclSecurity,vec![
            CLITableId::TypeDef,
            CLITableId::MethodDef,
            CLITableId::Assembly,
        ]);
        m.insert(CLIColumnType::MemberRefParent,vec![
            CLITableId::TypeDef,
            CLITableId::TypeRef,
            CLITableId::ModuleRef,
            CLITableId::MethodDef,
            CLITableId::TypeSpec
        ]);
        m.insert(CLIColumnType::HasSemantics,vec![
            CLITableId::Event,
            CLITableId::Property,
        ]);
        m.insert(CLIColumnType::MethodDefOrRef,vec![
            CLITableId::MethodDef,
            CLITableId::MemberRef,
        ]);
        m.insert(CLIColumnType::MemberForwarded,vec![
            CLITableId::Field,
            CLITableId::MethodDef,
        ]);
        m.insert(CLIColumnType::Implementation,vec![
            CLITableId::Field,
            CLITableId::AssemblyRef,
            CLITableId::ExportedType,
        ]);
        m.insert(CLIColumnType::CustomAttributeType,vec![
            CLITableId::MethodDef,
            CLITableId::MemberRef,
            CLITableId::Invalid,
            CLITableId::Invalid,
            CLITableId::Invalid,
        ]);
        m.insert(CLIColumnType::ResolutionScope,vec![
            CLITableId::Module,
            CLITableId::ModuleRef,
            CLITableId::AssemblyRef,
            CLITableId::TypeRef,
        ]);
        m.insert(CLIColumnType::TypeOrMethodDef,vec![
            CLITableId::TypeDef,
            CLITableId::MethodDef,
        ]);
        m
    };
}


#[derive(Debug, Copy, Clone, Eq)]
pub enum CLITableId {
    Assembly = 0x20,
    AssemblyOS = 0x22,
    AssemblyProcessor = 0x21,
    AssemblyRef = 0x23,
    AssemblyRefOS = 0x25,
    AssemblyRefProcessor = 0x24,
    ClassLayout = 0x0F,
    Constant = 0x0B,
    CustomAttribute = 0x0C,
    DeclSecurity = 0x0E,
    EventMap = 0x12,
    Event = 0x14,
    ExportedType = 0x27,
    Field = 0x04,
    FieldLayout = 0x10,
    FieldMarshal = 0x0D,
    FieldRVA = 0x1D,
    File = 0x26,
    GenericParam = 0x2A,
    GenericParamConstraint = 0x2C,
    ImplMap = 0x1C,
    InterfaceImpl = 0x09,
    ManifestResource = 0x28,
    MemberRef = 0x0A,
    MethodDef = 0x06,
    MethodImpl = 0x19,
    MethodSemantics = 0x18,
    MethodSpec = 0x2B,
    Module = 0x00,
    ModuleRef = 0x1A,
    NestedClass = 0x29,
    Param = 0x08,
    Property = 0x17,
    PropertyMap = 0x15,
    StandAloneSig = 0x11,
    TypeDef = 0x02,
    TypeRef = 0x01,
    TypeSpec = 0x1B,
    Invalid = 0xFF,
}

impl Ord for CLITableId {
    fn cmp(&self, other: &CLITableId) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for CLITableId {
    fn partial_cmp(&self, other: &CLITableId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CLITableId {
    fn eq(&self, other: &CLITableId) -> bool {
        (*self as u8) == (*other as u8)
    }
}

impl CLITableId {
    pub fn map() -> [CLITableId; 38] {
        static TABLES: [CLITableId; 38] = [
            CLITableId::Assembly,
            CLITableId::AssemblyOS,
            CLITableId::AssemblyProcessor,
            CLITableId::AssemblyRef,
            CLITableId::AssemblyRefOS,
            CLITableId::AssemblyRefProcessor,
            CLITableId::ClassLayout,
            CLITableId::Constant,
            CLITableId::CustomAttribute,
            CLITableId::DeclSecurity,
            CLITableId::EventMap,
            CLITableId::Event,
            CLITableId::ExportedType,
            CLITableId::Field,
            CLITableId::FieldLayout,
            CLITableId::FieldMarshal,
            CLITableId::FieldRVA,
            CLITableId::File,
            CLITableId::GenericParam,
            CLITableId::GenericParamConstraint,
            CLITableId::ImplMap,
            CLITableId::InterfaceImpl,
            CLITableId::ManifestResource,
            CLITableId::MemberRef,
            CLITableId::MethodDef,
            CLITableId::MethodImpl,
            CLITableId::MethodSemantics,
            CLITableId::MethodSpec,
            CLITableId::Module,
            CLITableId::ModuleRef,
            CLITableId::NestedClass,
            CLITableId::Param,
            CLITableId::Property,
            CLITableId::PropertyMap,
            CLITableId::StandAloneSig,
            CLITableId::TypeDef,
            CLITableId::TypeRef,
            CLITableId::TypeSpec,
        ];
        TABLES
    }
}

#[derive(Debug)]
pub struct CLITable<D>
{
    pub row:u32,
    pub data:Vec<D>
}


pub trait MetaItem<D>{
    fn parse_table(reader:&mut BinaryReader,tilde_stream:&CLITildeStream)->CLITable<D>;
}

type StrIndex = u32;
type GuidIndex = u32;
type BlobIndex = u32;
type TagIndex = u32;
type RowIndex = u32;

#[derive(Debug)]
pub struct MetaModule {
    pub name:StrIndex,
    pub mvid:GuidIndex,
}

impl MetaItem<MetaModule> for MetaModule {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaModule>{
        let row = tilde_stream.get_table_row(CLITableId::Module);
        let mut data:Vec<MetaModule> = Vec::new();
        let heap_size = tilde_stream.heap_size;
        for _ in 0..row{
            reader.ate(2);
            let item_name = reader.le_uint(heap_size.string);
            let item_mvid = reader.le_uint(heap_size.guid);
            reader.le_uint(heap_size.guid);
            reader.le_uint(heap_size.guid);
            data.push(MetaModule {
                name: item_name,
                mvid: item_mvid
            });
        };
        CLITable::<MetaModule>{
            row,
            data,
        }
    }
}

#[derive(Debug)]
pub struct MetaTypeRef{
    pub resolution_scope: TagIndex,//ResolutionScope
    pub name: StrIndex,
    pub namespace:StrIndex,
}

impl MetaItem<MetaTypeRef> for MetaTypeRef{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaTypeRef> {
        let row = tilde_stream.get_table_row(CLITableId::TypeRef);
        let heap_size = tilde_stream.heap_size;
        let column_size =tilde_stream.get_column_byte(CLIColumnType::ResolutionScope);
        let mut data = Vec::new();
        for _ in 0..row{
            let scope = reader.le_uint(column_size);
            let name = reader.le_uint(heap_size.string);
            let namespace = reader.le_uint(heap_size.string);
            data.push(MetaTypeRef{
                resolution_scope:scope,
                name,
                namespace
            });
        };
        CLITable::<MetaTypeRef>{row,data }
    }
}

#[derive(Debug)]
pub struct MetaTypeDef{
    pub type_attribute: TagIndex, //TypeAttribute 4byte
    pub name:StrIndex,
    pub namespace:StrIndex,
    pub extends: TagIndex, //TypeDefOrRef
    pub field_list:RowIndex,//Field table TODO
    pub method_list:RowIndex//MethodDef table TODO
}

impl MetaItem<MetaTypeDef> for MetaTypeDef{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaTypeDef> {
        let row = tilde_stream.get_table_row(CLITableId::TypeDef);
        let heap_size = tilde_stream.heap_size;
        let byte_extends = tilde_stream.get_column_byte(CLIColumnType::TypeDefOrRef);

        let mut data= Vec::new();
        for _ in 0..row{
            let type_attr = reader.le_u32();
            let name = reader.le_uint(heap_size.string);
            let namespace = reader.le_uint(heap_size.string);
            let extends = reader.le_uint(byte_extends);
            let field_list = reader.le_u16() as u32;
            let method_list = reader.le_u16() as u32;
            data.push(MetaTypeDef{
                type_attribute:type_attr,
                name,
                namespace,
                extends,
                field_list,
                method_list,
            });
        };
        CLITable::<MetaTypeDef>{
            row,
            data
        }
    }
}

pub struct MetaPropertyMap{
    pub parent:RowIndex, // TypeDef table
    pub property_list:RowIndex, // Property table TODO
}

pub struct MetaProperty{
    pub flags:u16,// PropertyAttribute 2byte
    pub name:StrIndex,
    pub type_data:BlobIndex,
}

pub struct MetaParam{
    pub flags:u16, //ParamAttribute
    pub sequence:u16,
    pub name:StrIndex,
}

pub struct MetaNestedClass{
    pub nested_class:RowIndex, //TypeDef table
    pub enclosing_class:RowIndex, //TypeDef table
}

pub struct MetaModuleRef{
    pub name:StrIndex,
}

pub struct MetaMethodSpec{
    pub method:TagIndex,//MethodDefOrRef
    pub instantiation:BlobIndex,
}

pub struct MetaMethodSemantics{
    pub semantics:u16,//MethodSemanticsAttribute
    pub method:RowIndex,//MethodDef table
    pub association:TagIndex,// HasSemantics column
}

pub struct MetaMethodImpl{
    pub class:RowIndex,//TypeDef table
    pub method_body:TagIndex,// MethodDefOrRef,
    pub method_decl:TagIndex,// MethodDefOrRef,
}

#[derive(Debug)]
pub struct MetaMethodDef{
    pub rva:u32,
    pub impl_flags:u16,//MethodImplAttributes
    pub flags: u16,//MethodAttributes,
    pub name:StrIndex,
    pub signature:BlobIndex,
    pub param_list:RowIndex,//Param table TODO
}

impl MetaItem<MetaMethodDef> for MetaMethodDef{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaMethodDef> {
        let row = tilde_stream.get_table_row(CLITableId::MethodDef);
        let heap_size = tilde_stream.heap_size;
        let mut data = Vec::new();
        for _ in 0..row{
            let rva = reader.le_u32();
            let impl_flags = reader.le_u16();
            let flags = reader.le_u16();
            let name = reader.le_u16() as u32;
            let signature = reader.le_u16() as u32;
            let param_list = reader.le_u16() as u32;
            data.push(MetaMethodDef{
                rva,impl_flags,flags,name,signature,param_list
            });
        };
        CLITable::<MetaMethodDef>{row,data}
    }
}

#[derive(Debug)]
pub struct MetaMemberRef{
    pub class:TagIndex, //MemberRefParent
    pub name:StrIndex,
    pub signature:BlobIndex,
}

impl MetaItem<MetaMemberRef> for MetaMemberRef{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaMemberRef> {
        let row = tilde_stream.get_table_row(CLITableId::MemberRef);
        let heap_size = tilde_stream.heap_size;
        let column_class = tilde_stream.get_column_byte(CLIColumnType::MemberRefParent);

        let mut data = Vec::new();
        for _ in 0..row{
            let class = reader.le_uint(column_class);
            let name = reader.le_uint(heap_size.string);
            let signature = reader.le_uint(heap_size.blob);
            data.push(MetaMemberRef{
                class,name,signature
            });
        };
        CLITable::<MetaMemberRef>{row,data}

    }
}

pub struct MetaManifestResource{
    pub offset:u32,
    pub flags:u32, //ManifestResourceAttributes,
    pub name:StrIndex,
    pub implementation:TagIndex,//Implementation,
}

pub struct MetaInterfaceImpl{
    pub class:RowIndex,//TypeDef,
    pub interface:TagIndex, //TypeDefOrRef
}

pub struct MetaImplMap{
    pub mapping_flags:u16,//PInvokeAttribute,
    pub member_forwarded:RowIndex, // MethodDef table,
    pub import_name:StrIndex,
    pub import_scope:RowIndex,//ModuleRef
}

pub struct MetaGenericParamConstraint{
    pub owner:RowIndex,//GenricParam table
    pub constraint:TagIndex,// TypeDefOrRef
}

pub struct MetaGenericParam{
    pub number:RowIndex,// two byte
    pub flags:u16,// GenericParamAttribute,
    pub owner:TagIndex,// TypeOrMethodDef,
    pub name:StrIndex,
}

pub struct MetaFile{
    pub flags:u32,//FileAttribute
    pub name:StrIndex,
    pub hash_value:BlobIndex,
}

pub struct MetaFieldRVA{
    pub rva:u32,
    pub field:RowIndex, //Field table
}

pub struct MetaFieldMarshal{
    pub parent:TagIndex,//HasFieldMarshal
    pub native_type:BlobIndex,
}

pub struct MetaFieldLayout{
    pub offset:u32,
    pub field:RowIndex,// Field table,
}

pub struct MetaField{
    pub flags:u16,//FieldAttribute,
    pub name:StrIndex,
    pub signature:BlobIndex,
}

pub struct MetaExportedType{
    pub flags:u32,//TypeAttribute,
    pub type_def_id:RowIndex,// 4byte index
    pub type_name:StrIndex,
    pub type_namespace:StrIndex,
    pub implementation:TagIndex,//Implementation column,
}

pub struct MetaEvent{
    pub event_flags:u16,//EventAttribute,
    pub name:StrIndex,
    pub event_type:TagIndex,//TypeDefOrRef,
}

pub struct MetaEventMap{
    pub parent:RowIndex,//TypeDef table,
    pub event_list:RowIndex,//Event table TODO
}

pub struct MetaDeclSecurity{
    pub action:u16,
    pub parent:TagIndex,//HasDeclSecurity column,
    pub permission_set:BlobIndex,
}

#[derive(Debug)]
pub struct MetaCustomAttribute{
    pub parent:TagIndex,//HasCustomAttribute,
    pub attr_type:TagIndex,//CustomAttributeType,
    pub value:BlobIndex,
}

impl MetaItem<MetaCustomAttribute> for MetaCustomAttribute{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaCustomAttribute> {
        let row = tilde_stream.get_table_row(CLITableId::CustomAttribute);
        let heap_size = tilde_stream.heap_size;
        let column_parent = tilde_stream.get_column_byte(CLIColumnType::HasCustomAttribute);
        let column_attr_type = tilde_stream.get_column_byte(CLIColumnType::CustomAttributeType);

        let mut data = Vec::new();

        for _ in 0..row{
            let parent = reader.le_uint(column_parent);
            let attr_type = reader.le_uint(column_attr_type);
            let value = reader.le_uint(heap_size.blob);
            data.push(MetaCustomAttribute{
                parent,attr_type,value
            })
        }

        CLITable::<MetaCustomAttribute>{row,data}

    }
}

pub struct MetaConstant{
    pub const_type:u8,//two bytes , the second byte is 0,
    pub parent:TagIndex,//HasConstant column,
    pub value:BlobIndex,
}

pub struct MetaClassLayout{
    pub packing_size:u16,
    pub class_size:u32,
    pub parent:RowIndex,//TypeDef table
}

pub struct MetaAssemblyRefProcessor{
    pub processor:u32,
    pub assembly_ref:RowIndex,//AssemblyRef,
}

pub struct MetaAssemblyRefOS{
    pub platform_id:u32,
    pub major_ver:u32,
    pub minor_ver:u32,
    pub asssmbly_ref:RowIndex,//AssemblyRef table
}

#[derive(Debug)]
pub struct MetaAssemblyRef{
    pub maj_ver:u16,
    pub min_ver:u16,
    pub build_num:u16,
    pub revision_num:u16,
    pub flags:u32,//AssemblyFlags
    pub name:StrIndex,
    pub culture:StrIndex,
    pub hash_value:BlobIndex,
}

impl MetaItem<MetaAssemblyRef> for MetaAssemblyRef{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaAssemblyRef> {
        let row = tilde_stream.get_table_row(CLITableId::AssemblyRef);
        let heap_size = tilde_stream.heap_size;

        let mut data = Vec::new();
        for _ in 0..row{
            let maj_ver = reader.le_u16();
            let min_ver = reader.le_u16();
            let build_num = reader.le_u16();
            let revision_num = reader.le_u16();
            let flags = reader.le_u32();
            let name = reader.le_uint(heap_size.string);
            let culture = reader.le_uint(heap_size.string);
            let hash_value = reader.le_uint(heap_size.blob);
            data.push(MetaAssemblyRef{
                maj_ver,
                min_ver,
                build_num,
                revision_num,
                flags,
                name,
                culture,
                hash_value
            });
        }
        CLITable::<MetaAssemblyRef>{row,data}
    }
}

pub struct MetaAssemblyProcessor{
    pub processor:u32,
}

pub struct MetaAssemblyOS{
    pub platform_id:u32,
    pub major_ver:u32,
    pub minor_ver:u32,
}

#[derive(Debug)]
pub struct MetaAssembly{
    pub hash_alg_id:u32, //AssemblyHashAlgorithms,
    pub major_ver:u16,
    pub minor_ver:u16,
    pub build_num:u16,
    pub revision_num:u16,
    pub flags:u32, //AssemblyFlags
    pub public_key:BlobIndex,
    pub name:StrIndex,
    pub culture:StrIndex,
}

impl MetaItem<MetaAssembly> for MetaAssembly{
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream) -> CLITable<MetaAssembly> {
        let row = tilde_stream.get_table_row(CLITableId::Assembly);
        let heap_size = tilde_stream.heap_size;
        let mut data =Vec::new();
        for _ in 0..row{
            let hash_alg_id = reader.le_u32();
            let major_ver = reader.le_u16();
            let minor_ver = reader.le_u16();
            let build_num = reader.le_u16();
            let revision_num = reader.le_u16();
            let flags = reader.le_u32();
            let public_key = reader.le_uint(heap_size.blob);
            let name = reader.le_uint(heap_size.string);
            let culture = reader.le_uint(heap_size.string);
            data.push(MetaAssembly{
                hash_alg_id,
                major_ver,
                minor_ver,
                build_num,
                revision_num,
                flags,
                public_key,
                name,
                culture,
            });
        }
        CLITable::<MetaAssembly>{row,data}
    }
}
