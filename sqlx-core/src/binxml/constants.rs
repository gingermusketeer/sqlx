pub(super) const SQL_SMALL_INT: u8 = 0x01;
pub(super) const SQL_INT: u8 = 0x02;
pub(super) const SQL_REAL: u8 = 0x03;
pub(super) const SQL_FLOAT: u8 = 0x04;
pub(super) const SQL_MONEY: u8 = 0x05;
pub(super) const SQL_BIT: u8 = 0x06;
pub(super) const SQL_TINY_INT: u8 = 0x07;
pub(super) const SQL_BIG_INT: u8 = 0x08;
pub(super) const SQL_UUID: u8 = 0x09;
pub(super) const SQL_DECIMAL: u8 = 0x0A;
pub(super) const SQL_NUMERIC: u8 = 0x0B;
pub(super) const SQL_BINARY: u8 = 0x0C;
pub(super) const SQL_CHAR: u8 = 0x0D;
pub(super) const SQL_NCHAR: u8 = 0x0E;
pub(super) const SQL_VAR_BINARY: u8 = 0x0F;
pub(super) const SQL_VAR_CHAR: u8 = 0x10;
pub(super) const SQL_NVAR_CHAR: u8 = 0x11;
pub(super) const SQL_DATE_TIME: u8 = 0x12;
pub(super) const SQL_SMALL_DATE_TIME: u8 = 0x13;
pub(super) const SQL_SMALL_MONEY: u8 = 0x14;
pub(super) const SQL_TEXT: u8 = 0x16;
pub(super) const SQL_IMAGE: u8 = 0x17;
pub(super) const SQL_NTEXT: u8 = 0x18;
pub(super) const SQL_UDT: u8 = 0x1B;
pub(super) const XSD_TIME_OFFSET: u8 = 0x7A;
pub(super) const XSD_DATE_TIME_OFFSET: u8 = 0x7B;
pub(super) const XSD_DATE_OFFSET: u8 = 0x7C;
pub(super) const XSD_TIME2: u8 = 0x7D;
pub(super) const XSD_DATE_TIME2: u8 = 0x7E;
pub(super) const XSD_DATE2: u8 = 0x7F;
pub(super) const XSD_TIME: u8 = 0x81;
pub(super) const XSD_DATE_TIME: u8 = 0x82;
pub(super) const XSD_DATE: u8 = 0x83;
pub(super) const XSD_BIN_HEX: u8 = 0x84;
pub(super) const XSD_BASE64: u8 = 0x85;
pub(super) const XSD_BOOLEAN: u8 = 0x86;
pub(super) const XSD_DECIMAL: u8 = 0x87;
pub(super) const XSD_BYTE: u8 = 0x88;
pub(super) const XSD_UNSIGNED_SHORT: u8 = 0x89;
pub(super) const XSD_UNSIGNED_INT: u8 = 0x8A;
pub(super) const XSD_UNSIGNED_LONG: u8 = 0x8B;
pub(super) const XSD_QNAME: u8 = 0x8C;
pub(super) const FLUSH_DEFINED_NAME_TOKENS: u8 = 0xE9;
pub(super) const EXTEN_TOKEN: u8 = 0xEA;
pub(super) const ENDNEST_TOKEN: u8 = 0xEB;
pub(super) const NEST_TOKEN: u8 = 0xEC;
pub(super) const QNAME_DEF_TOKEN: u8 = 0xEF;
pub(super) const NAME_DEF_TOKEN: u8 = 0xF0;
pub(super) const CDATA_END_TOKEN: u8 = 0xF1;
pub(super) const CDATA_TOKEN: u8 = 0xF2;
pub(super) const COMMENT_TOKEN: u8 = 0xF3;
pub(super) const PI_TOKEN: u8 = 0xF4;
pub(super) const END_ATTRIBUTES_TOKEN: u8 = 0xF5;
pub(super) const ATTRIBUTE_TOKEN: u8 = 0xF6;
pub(super) const END_ELEMENT_TOKEN: u8 = 0xF7;
pub(super) const ELEMENT_TOKEN: u8 = 0xF8;
pub(super) const SUB_SET_TOKEN: u8 = 0xF9;
pub(super) const PUBLIC_TOKEN: u8 = 0xFA;
pub(super) const SYSTEM_TOKEN: u8 = 0xFB;
pub(super) const DOCTYPE_DECL_TOKEN: u8 = 0xFC;
pub(super) const ENCODING_TOKEN: u8 = 0xFD;
pub(super) const XML_DECL_TOKEN: u8 = 0xFE;

pub(super) const SIGNATURE: u16 = 0xDFFF;

// 1200 little-endian = UTF-16LE
pub(super) const ENCODING: u16 = 0xB004;
