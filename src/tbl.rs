use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

use crate::reader::*;
use crate::meta::*;


#[derive(Eq, Debug, PartialEq, Hash, Copy, Clone)]
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

lazy_static! {
    pub static ref CLICOLUMN_MAP:HashMap<CLIColumnType,Vec<CLITableId>> = {
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

#[derive(Debug, Default)]
pub struct CLITable<D>
{
    pub row: u32,
    pub data: Vec<D>,
}

impl<D> CLITable<D> where D: MetaItem<D> {
    pub fn get_data_by_index(&self, index: usize) -> &D {
        &self.data[index]
    }

    pub fn get_data_by_filter(&self, filter: &Fn(&D) -> bool) -> Option<&D> {
        let mut ret = Option::None;
        for item in self.data.iter() {
            if filter(item) {
                ret = Some(item);
                break;
            }
        }
        ret
    }

    pub fn get_data_by_filter_ind(&self, filter: &Fn(&D) -> bool, index: &mut usize) -> Option<&D> {
        let mut ret = Option::None;
        for (ind, item) in self.data.iter().enumerate() {
            if filter(item) {
                ret = Some(item);
                *index = ind;
                break;
            }
        }
        ret
    }
}


pub trait MetaItem<D> {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<D>;
}

type StrIndex = u32;
type GuidIndex = u32;
type BlobIndex = u32;
type TagIndex = u32;
type RowIndex = u32;

#[derive(Debug, Default)]
pub struct MetaModule {
    pub name: Rc<String>,
    pub mvid: GuidIndex,
}

impl MetaItem<MetaModule> for MetaModule {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaModule> {
        let row = tilde_stream.get_table_row(CLITableId::Module);
        let mut data: Vec<MetaModule> = Vec::new();
        let heap_size = tilde_stream.heap_size;
        for _ in 0..row {
            reader.ate(2);
            let item_name = reader.le_uint(heap_size.string);
            let item_mvid = reader.le_uint(heap_size.guid);
            reader.le_uint(heap_size.guid);
            reader.le_uint(heap_size.guid);
            data.push(MetaModule {
                name: string_stream.get_str_by_index(item_name),
                mvid: item_mvid,
            });
        };
        CLITable::<MetaModule> {
            row,
            data,
        }
    }
}

#[derive(Debug, Default)]
pub struct MetaTypeRef {
    pub resolution_scope: TagIndex,
    //ResolutionScope
    pub name: Rc<String>,
    pub namespace: Rc<String>,
}

impl MetaItem<MetaTypeRef> for MetaTypeRef {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaTypeRef> {
        let row = tilde_stream.get_table_row(CLITableId::TypeRef);
        let heap_size = tilde_stream.heap_size;
        let column_size = tilde_stream.get_column_byte(CLIColumnType::ResolutionScope);
        let mut data = Vec::new();
        for _ in 0..row {
            let scope = reader.le_uint(column_size);
            let name = reader.le_uint(heap_size.string);
            let namespace = reader.le_uint(heap_size.string);
            data.push(MetaTypeRef {
                resolution_scope: scope,
                name: string_stream.get_str_by_index(name),
                namespace: string_stream.get_str_by_index(namespace),
            });
        };
        CLITable::<MetaTypeRef> { row, data }
    }
}

#[derive(Debug, Default)]
pub struct MetaTypeDef {
    pub type_attribute: TagIndex,
    //TypeAttribute 4byte
    pub name: Rc<String>,
    pub namespace: Rc<String>,
    pub extends: TagIndex,
    //TypeDefOrRef
    pub field_list: RowIndex,
    //Field table TODO
    pub method_list: RowIndex,//MethodDef table TODO
}

impl MetaItem<MetaTypeDef> for MetaTypeDef {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaTypeDef> {
        let row = tilde_stream.get_table_row(CLITableId::TypeDef);
        let heap_size = tilde_stream.heap_size;
        let byte_extends = tilde_stream.get_column_byte(CLIColumnType::TypeDefOrRef);

        let mut data = Vec::new();
        for _ in 0..row {
            let type_attr = reader.le_u32();
            let name = reader.le_uint(heap_size.string);
            let namespace = reader.le_uint(heap_size.string);
            let extends = reader.le_uint(byte_extends);
            let field_list = reader.le_u16() as u32;
            let method_list = reader.le_u16() as u32;
            data.push(MetaTypeDef {
                type_attribute: type_attr,
                name: string_stream.get_str_by_index(name),
                namespace: string_stream.get_str_by_index(namespace),
                extends,
                field_list,
                method_list,
            });
        };
        CLITable::<MetaTypeDef> {
            row,
            data,
        }
    }
}

#[derive(Debug, Default)]
pub struct MetaPropertyMap {
    pub parent: RowIndex,
    // TypeDef table
    pub property_list: RowIndex, // Property table TODO
}

#[derive(Debug, Default)]
pub struct MetaProperty {
    pub flags: u16,
    // PropertyAttribute 2byte
    pub name: StrIndex,
    pub type_data: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaParam {
    pub flags: u16,
    //ParamAttribute
    pub sequence: u16,
    pub name: StrIndex,
}

#[derive(Debug, Default)]
pub struct MetaNestedClass {
    pub nested_class: RowIndex,
    //TypeDef table
    pub enclosing_class: RowIndex, //TypeDef table
}

#[derive(Debug, Default)]
pub struct MetaModuleRef {
    pub name: StrIndex,
}

#[derive(Debug, Default)]
pub struct MetaMethodSpec {
    pub method: TagIndex,
    //MethodDefOrRef
    pub instantiation: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaMethodSemantics {
    pub semantics: u16,
    //MethodSemanticsAttribute
    pub method: RowIndex,
    //MethodDef table
    pub association: TagIndex,// HasSemantics column
}

#[derive(Debug, Default)]
pub struct MetaMethodImpl {
    pub class: RowIndex,
    //TypeDef table
    pub method_body: TagIndex,
    // MethodDefOrRef,
    pub method_decl: TagIndex,// MethodDefOrRef,
}

#[derive(Debug, Default)]
pub struct MetaMethodDef {
    pub rva: u32,
    pub impl_flags: u16,
    //MethodImplAttributes
    pub flags: u16,
    //MethodAttributes,
    pub name: Rc<String>,
    pub signature: BlobIndex,
    pub param_list: RowIndex,//Param table TODO
}

impl MetaItem<MetaMethodDef> for MetaMethodDef {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaMethodDef> {
        let row = tilde_stream.get_table_row(CLITableId::MethodDef);
        let _heap_size = tilde_stream.heap_size;
        let mut data = Vec::new();
        for _ in 0..row {
            let rva = reader.le_u32();
            let impl_flags = reader.le_u16();
            let flags = reader.le_u16();
            let name = reader.le_u16() as u32;
            let signature = reader.le_u16() as u32;
            let param_list = reader.le_u16() as u32;


            data.push(MetaMethodDef {
                rva,
                impl_flags,
                flags,
                name: string_stream.get_str_by_index(name),
                signature,
                param_list,
            });
        };
        CLITable::<MetaMethodDef> { row, data }
    }
}

#[derive(Debug, Default)]
pub struct MetaMemberRef {
    pub class: TagIndex,
    //MemberRefParent
    pub name: Rc<String>,
    pub signature: BlobIndex,
}

impl MetaItem<MetaMemberRef> for MetaMemberRef {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaMemberRef> {
        let row = tilde_stream.get_table_row(CLITableId::MemberRef);
        let heap_size = tilde_stream.heap_size;
        let column_class = tilde_stream.get_column_byte(CLIColumnType::MemberRefParent);

        let mut data = Vec::new();
        for _ in 0..row {
            let class = reader.le_uint(column_class);
            let name = reader.le_uint(heap_size.string);
            let signature = reader.le_uint(heap_size.blob);
            data.push(MetaMemberRef {
                class,
                name: string_stream.get_str_by_index(name),
                signature,
            });
        };
        CLITable::<MetaMemberRef> { row, data }
    }
}

#[derive(Debug, Default)]
pub struct MetaManifestResource {
    pub offset: u32,
    pub flags: u32,
    //ManifestResourceAttributes,
    pub name: StrIndex,
    pub implementation: TagIndex,//Implementation,
}

#[derive(Debug, Default)]
pub struct MetaInterfaceImpl {
    pub class: RowIndex,
    //TypeDef,
    pub interface: TagIndex, //TypeDefOrRef
}

#[derive(Debug, Default)]
pub struct MetaImplMap {
    pub mapping_flags: u16,
    //PInvokeAttribute,
    pub member_forwarded: RowIndex,
    // MethodDef table,
    pub import_name: StrIndex,
    pub import_scope: RowIndex,//ModuleRef
}

#[derive(Debug, Default)]
pub struct MetaGenericParamConstraint {
    pub owner: RowIndex,
    //GenricParam table
    pub constraint: TagIndex,// TypeDefOrRef
}

#[derive(Debug, Default)]
pub struct MetaGenericParam {
    pub number: RowIndex,
    // two byte
    pub flags: u16,
    // GenericParamAttribute,
    pub owner: TagIndex,
    // TypeOrMethodDef,
    pub name: StrIndex,
}

#[derive(Debug, Default)]
pub struct MetaFile {
    pub flags: u32,
    //FileAttribute
    pub name: StrIndex,
    pub hash_value: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaFieldRVA {
    pub rva: u32,
    pub field: RowIndex, //Field table
}

#[derive(Debug, Default)]
pub struct MetaFieldMarshal {
    pub parent: TagIndex,
    //HasFieldMarshal
    pub native_type: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaFieldLayout {
    pub offset: u32,
    pub field: RowIndex,// Field table,
}

#[derive(Debug, Default)]
pub struct MetaField {
    pub flags: u16,
    //FieldAttribute,
    pub name: StrIndex,
    pub signature: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaExportedType {
    pub flags: u32,
    //TypeAttribute,
    pub type_def_id: RowIndex,
    // 4byte index
    pub type_name: StrIndex,
    pub type_namespace: StrIndex,
    pub implementation: TagIndex,//Implementation column,
}

#[derive(Debug, Default)]
pub struct MetaEvent {
    pub event_flags: u16,
    //EventAttribute,
    pub name: StrIndex,
    pub event_type: TagIndex,//TypeDefOrRef,
}

#[derive(Debug, Default)]
pub struct MetaEventMap {
    pub parent: RowIndex,
    //TypeDef table,
    pub event_list: RowIndex,//Event table TODO
}

#[derive(Debug, Default)]
pub struct MetaDeclSecurity {
    pub action: u16,
    pub parent: TagIndex,
    //HasDeclSecurity column,
    pub permission_set: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaCustomAttribute {
    pub parent: TagIndex,
    //HasCustomAttribute,
    pub attr_type: TagIndex,
    //CustomAttributeType,
    pub value: BlobIndex,
}

impl MetaItem<MetaCustomAttribute> for MetaCustomAttribute {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, _string_stream: &CLIStringStream) -> CLITable<MetaCustomAttribute> {
        let row = tilde_stream.get_table_row(CLITableId::CustomAttribute);
        let heap_size = tilde_stream.heap_size;
        let column_parent = tilde_stream.get_column_byte(CLIColumnType::HasCustomAttribute);
        let column_attr_type = tilde_stream.get_column_byte(CLIColumnType::CustomAttributeType);

        let mut data = Vec::new();

        for _ in 0..row {
            let parent = reader.le_uint(column_parent);
            let attr_type = reader.le_uint(column_attr_type);
            let value = reader.le_uint(heap_size.blob);
            data.push(MetaCustomAttribute {
                parent,
                attr_type,
                value,
            })
        }

        CLITable::<MetaCustomAttribute> { row, data }
    }
}

#[derive(Debug, Default)]
pub struct MetaConstant {
    pub const_type: u8,
    //two bytes , the second byte is 0,
    pub parent: TagIndex,
    //HasConstant column,
    pub value: BlobIndex,
}

#[derive(Debug, Default)]
pub struct MetaClassLayout {
    pub packing_size: u16,
    pub class_size: u32,
    pub parent: RowIndex,//TypeDef table
}

#[derive(Debug, Default)]
pub struct MetaAssemblyRefProcessor {
    pub processor: u32,
    pub assembly_ref: RowIndex,//AssemblyRef,
}

#[derive(Debug, Default)]
pub struct MetaAssemblyRefOS {
    pub platform_id: u32,
    pub major_ver: u32,
    pub minor_ver: u32,
    pub asssmbly_ref: RowIndex,//AssemblyRef table
}

#[derive(Debug, Default)]
pub struct MetaAssemblyRef {
    pub maj_ver: u16,
    pub min_ver: u16,
    pub build_num: u16,
    pub revision_num: u16,
    pub flags: u32,
    //AssemblyFlags
//    pub public_key_or_token:u32,
    pub name: Rc<String>,
    pub culture: Rc<String>,
    pub hash_value: BlobIndex,
}

impl MetaItem<MetaAssemblyRef> for MetaAssemblyRef {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaAssemblyRef> {
        let row = tilde_stream.get_table_row(CLITableId::AssemblyRef);
        let heap_size = tilde_stream.heap_size;

        let mut data = Vec::new();
        for _ in 0..row {
            let maj_ver = reader.le_u16();
            let min_ver = reader.le_u16();
            let build_num = reader.le_u16();
            let revision_num = reader.le_u16();
            let flags = reader.le_u32();
            reader.le_uint(heap_size.blob);
            let name = reader.le_uint(heap_size.string);
            let culture = reader.le_uint(heap_size.string);
            let hash_value = reader.le_uint(heap_size.blob);
            data.push(MetaAssemblyRef {
                maj_ver,
                min_ver,
                build_num,
                revision_num,
                flags,
                name: string_stream.get_str_by_index(name),
                culture: string_stream.get_str_by_index(culture),
                hash_value,
            });
        }
        CLITable::<MetaAssemblyRef> { row, data }
    }
}

#[derive(Debug, Default)]
pub struct MetaAssemblyProcessor {
    pub processor: u32,
}

#[derive(Debug, Default)]
pub struct MetaAssemblyOS {
    pub platform_id: u32,
    pub major_ver: u32,
    pub minor_ver: u32,
}

#[derive(Debug, Default)]
pub struct MetaStandAloneSig {
    pub signature: u32,
}

impl MetaItem<MetaStandAloneSig> for MetaStandAloneSig {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, _string_stream: &CLIStringStream) -> CLITable<MetaStandAloneSig> {
        let row = tilde_stream.get_table_row(CLITableId::StandAloneSig);
        let heap_size = tilde_stream.heap_size;
        let mut data = Vec::new();
        for _ in 0..row {
            let signature = reader.le_uint(heap_size.blob);
            data.push(MetaStandAloneSig { signature });
        }
        CLITable::<MetaStandAloneSig> { row, data }
    }
}


#[derive(Debug, Default)]
pub struct MetaAssembly {
    pub hash_alg_id: u32,
    //AssemblyHashAlgorithms,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub build_num: u16,
    pub revision_num: u16,
    pub flags: u32,
    //AssemblyFlags
    pub public_key: BlobIndex,
    pub name: Rc<String>,
    pub culture: Rc<String>,
}

impl MetaItem<MetaAssembly> for MetaAssembly {
    fn parse_table(reader: &mut BinaryReader, tilde_stream: &CLITildeStream, string_stream: &CLIStringStream) -> CLITable<MetaAssembly> {
        let row = tilde_stream.get_table_row(CLITableId::Assembly);
        let heap_size = tilde_stream.heap_size;
        let mut data = Vec::new();
        for _ in 0..row {
            let hash_alg_id = reader.le_u32();
            let major_ver = reader.le_u16();
            let minor_ver = reader.le_u16();
            let build_num = reader.le_u16();
            let revision_num = reader.le_u16();
            let flags = reader.le_u32();
            let public_key = reader.le_uint(heap_size.blob);
            let name = reader.le_uint(heap_size.string);
            let culture = reader.le_uint(heap_size.string);
            data.push(MetaAssembly {
                hash_alg_id,
                major_ver,
                minor_ver,
                build_num,
                revision_num,
                flags,
                public_key,
                name: string_stream.get_str_by_index(name),
                culture: string_stream.get_str_by_index(culture),
            });
        }
        CLITable::<MetaAssembly> { row, data }
    }
}
