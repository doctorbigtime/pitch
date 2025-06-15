import sys
import xml.etree.ElementTree as ET

def do_base_typedefs(defs):
    defs['type_map'] = {'u8_t': 'u8'
        , 'char_t': 'u8'
        , 'u64_t': 'u64'
        , 'u48_t': 'u64'
        , 'u32_t': 'u32'
        , 'u16_t': 'u16'
        , 'char_8_t': '[u8; 8]'
        , 'price_2_t': 'u16'
        , 'price_4_t': 'u32'
        , 'price_8_t': 'u64'
    }

def camel_case_name(orig):
    return ''.join(map(lambda n: n[0].upper() + n[1:], orig.split('_')))

def get_rust_type(defs, base_type):
    if base_type in defs['type_map']:
        return defs['type_map'][base_type]
    raise ValueError('Unknown type')

def type_is_enum(defs, tipe):
    return tipe in defs['enums']

def type_is_array(tipe):
    return tipe.startswith('[')

def type_is_integer(tipe):
    return tipe in ['i16', 'u16', 'i32', 'u32', 'i64', 'u64']

def array_size(tipe):
    def remove_the_fucking_suffix_fucking_python(haystack, suffix):
        if suffix and haystack.endswith(suffix):
            return haystack[:-len(suffix)]
        return haystack
    if not type_is_array(tipe):
        raise RuntimeError('{} is not an array'.format(tipe))
    #return int(tipe.split(';')[1].removesuffix(']'))
    return int(remove_the_fucking_suffix_fucking_python(tipe.split(';')[1], ']'))

def create_datadef(xml):
    defs = { 'enums': {}, 'structs': {}, 'type_map': {}, 'bitfields': {} }

    do_base_typedefs(defs)

    for item in xml.find('Typedefs'):
        type_name = item.get('name')
        type_type = item.get('type')
        defs['type_map'][type_name] = type_type

    for item in xml.find('Enums'):
        enum_name = item.get('name')
        enum_underlying = get_rust_type(defs, item.get('type'))
        defs['enums'][enum_name] = {'underlying_type': enum_underlying, 'values':[]}
        for value in item:
            defs['enums'][enum_name]['values'].append((value.get('name'), value.get('value')))
        defs['type_map'][enum_name] = enum_name

    for item in xml.find('Bitfields'):
        bitfield_len = item.get('len')
        bitfield_snake_name = item.get('name')
        bitfield_name = camel_case_name(bitfield_snake_name)
        defs['bitfields'][bitfield_snake_name] = {'name': bitfield_name, 'num_bytes':bitfield_len, 'bytes':[]}
        for bit in item:
            bit_name = bit.get('name')
            bit_byte = int(bit.get('byte')) - 1
            bit_bit = 2 ** int(bit.get('offset'))
            # bit_type = get_rust_type(defs, bit.get('type'))
            if bit_byte >= len(defs['bitfields'][bitfield_snake_name]['bytes']) :
                defs['bitfields'][bitfield_snake_name]['bytes'].append([])
            defs['bitfields'][bitfield_snake_name]['bytes'][bit_byte].append({'name':bit_name, 'bit':bit_bit})
        defs['type_map'][bitfield_snake_name] = bitfield_name

    for item in xml.find('Structs'):
        struct_lower_name = item.get('name')
        struct_name = camel_case_name(struct_lower_name)
        struct_id = item.get('id')
        defs['structs'][struct_name] = {'lower_name': struct_lower_name, 'id': struct_id, 'fields': []}
        offset = 0
        for field in item:
            field_name = field.get('name')
            field_type = get_rust_type(defs, field.get('type'))
            field_comment = field.get('comment')
            field_offset = int(field.get('offset'))
            field_len = int(field.get('len'))
            if field_offset != offset:
                raise ValueError('this xml isn\'t right: {}::{} is at offset {}, but xml says {}'.format(struct_name, field_name, offset, field_offset))
            defs['structs'][struct_name]['fields'].append({'name':field_name, 'type': field_type, 'comment': field_comment, 'offset': field_offset, 'length': field_len})
            offset += field_len


    return defs

def do_typedefs(datadef):
    pass

def do_header(datadef):
    print('use std::fmt;')
    # print('use std::io::{Cursor, Read, Write};')
    print('use std::default::Default;')
    # print('use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};')
    print('')

def do_constants(datadef):
    pass

def do_enums(datadef):
    for enum_name, enum in datadef['enums'].items():
        enum_rust_type = enum['underlying_type']
        print('#[derive(PartialEq, Default, Clone, Copy, Debug)]')
        print('pub struct {}(pub {});'.format(enum_name, enum['underlying_type']))
        print('impl {} {{'.format(enum_name))
        for value_name, value_value in enum['values']:
            print('  pub const {} : {} = \'{}\' as {};'.format(value_name.upper(), enum_rust_type, value_value, enum_rust_type))
        print('}} // {}'.format(enum_name))
        print('')
        stringize = lambda x: x.replace('_', ' ')
        print('impl fmt::Display for {} {{'.format(enum_name))
        print('  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {')
        print('    let s = match self.0 {')
        for value_name, value_value in enum['values']:
            print('      {}::{} => "\'{}\' ({})".to_string(),'.format(enum_name, value_name.upper(), value_value, stringize(value_name)))
        print('      _ => "Unknown".to_string(),')
        print('    };')
        print('    write!(f, "{}", s)')
        print('  }')
        print('}} // fmt::Display for {}'.format(enum_name))
        print('')

def do_bitfields(datadef):
    print('// Bitfields')
    for bitfield_name, bitfield in datadef['bitfields'].items():
        byte_funcs = []
        for byteno in range(len(bitfield['bytes'])):
            bytename = '{}Byte{}'.format(bitfield['name'][:-1], byteno+1)
            print('#[derive(Default, Clone)]')
            print('pub struct {}(pub u8);'.format(bytename))
            print('impl {} {{'.format(bytename))
            for bit in bitfield['bytes'][byteno]:
                bitname = bit['name']
                print('  pub const {}: u8 = {};'.format(bit['name'].upper(), bit['bit']))
                setter = 'set_has_{}'.format(bitname.lower())
                getter = 'get_{}'.format(bitname.lower())
                print('  pub fn {}(&self) -> bool {{ self.0 & {}::{} == {}::{} }}'.format(getter, bytename, bitname.upper(), bytename, bitname.upper()))
                print('  pub fn {}(&mut self) {{ self.0 |= {}::{}; }}'.format(setter, bytename, bitname.upper()))
                byte_funcs.append((byteno+1, getter, setter))
            print('}')
            print('')
        print('#[derive(Default, Clone)]')
        print('pub struct {} {{'.format(bitfield['name']))
        for byteno in range(len(bitfield['bytes'])):
            print('  pub byte{} : {}'.format(byteno + 1, '{}Byte{}'.format(bitfield['name'][:-1], byteno + 1)))
        print('}} // {}'.format(bitfield['name']))
        print('impl {} {{'.format(bitfield['name']))
        for byteno, getter, setter in byte_funcs:
            print('  pub fn {}(&self) -> bool {{ self.byte{}.{}() }}'.format(getter, byteno, getter))
            print('  pub fn {}(&mut self) {{ self.byte{}.{}() }}'.format(setter, byteno, setter))
        # print('  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<{}> {{'.format(bitfield['name']))
        # print('    let obj = {}::default();'.format(bitfield['name']))
        # # FIXME: support bitfields > 1 byte
        # print('    rdr.read_exact(std::slice::from_mut(&mut obj.byte1.0))?;')
        # print('    Ok(obj)')
        # print('  }')
        print('}} // impl {}'.format(bitfield['name']))
        print('')
        print('impl fmt::Display for {} {{'.format(bitfield['name']))
        print('  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {')
        print('    write!(f, "{:#02x}", self.byte1.0)')
        print('  }')
        print('}} // impl fmt::Display for {}'.format(bitfield['name']))
        print('')



def do_structs(datadef):
    print('// Structs')
    for struct_name, struct in datadef['structs'].items():
        print('#[derive(Clone, Default)]')
        print('pub struct {}<\'a> {{'.format(struct_name))
        print('  data: &\'a [u8],')
        # for field in struct['fields']:
        #     print('  pub {}: {}, // {}'.format(field['name'], field['type'], field['comment']))
        print('}} // {}'.format(struct_name))
        print('')

        print('impl<\'a> {}<\'a> {{'.format(struct_name))
        if struct['id']:
            print('  pub const TYPE : u8 = {} as u8;'.format(struct['id']))
        print('  pub fn new(data: &\'a [u8]) -> {}<\'a> {{ Self{{ data }} }}'.format(struct_name))
        for field in struct['fields']:
            tipe = field['type']
            name = field['name']
            offset = field['offset']
            length = field['length']
            print('  pub fn {}(&self) -> {} {{'.format(name, tipe))
            if tipe == 'u8':
                print('    self.data[{}]'.format(offset))
            elif type_is_enum(datadef, tipe):
                print('    {}(self.data[{}])'.format(tipe, offset))
            elif type_is_integer(tipe):
                print('    {}::from_le_bytes(self.data[{}..{}].try_into().unwrap())'.format(tipe, offset, offset+length))
            elif type_is_array(tipe):
                print('    self.data[{}..{}].try_into().unwrap()'.format(offset, offset+length))
            elif 'Flags' in tipe:
                print('    {}{{ byte1: {}Byte1(self.data[{}]) }}'.format(tipe, tipe[:-1], offset))
            else:
                print('     // unsupported type {}'.format(tipe))

            print('  }')
        print('')
        print('}} // impl {}'.format(struct_name))
        print('')

        def maybe_transform(field_name, field_type):
            if type_is_array(field_type):
                return 'String::from_utf8_lossy(&self.{}()[..])'.format(field_name)
            return 'self.' + field_name + '()'

        print('impl<\'a> fmt::Display for {}<\'a> {{'.format(struct_name))
        print('  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {')
        print('    write!(f, "{}('.format(struct_name), end='')
        for i, field in enumerate(struct['fields']):
            if i != 0:
                print(',', end='')
            print('{}:{{}}'.format(field['name']), end='')
        print(')"', end='')
        for field in struct['fields']:
            print(', {}'.format(maybe_transform(field['name'], field['type'])), end='')
        print(')')
        print('  }')
        print('}')
        print('')


def do_iterators(defs):
    print('pub enum PitchMessage<\'a> {')
    for struct_name, struct in defs['structs'].items():
        if struct.get('id'):
            print('  {}({}<\'a>),'.format(struct_name, struct_name))
    print('} // PitchMessage')
    print('')
    print('impl<\'a> fmt::Display for PitchMessage<\'a> {')
    print('  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {')
    print('    write!(f, "PitchMessage(")?;')
    print('    match self {')
    for struct_name, struct in defs['structs'].items():
        if struct.get('id'):
            print('      PitchMessage::{}(msg) => {{ write!(f, "{{}}", msg)? }},'.format(struct_name))
    print('    }')
    print('    write!(f, ")")')
    print('  }')
    print('}')
    print('')
    print('pub struct PitchItem<\'a> {')
    print('  pub msg: PitchMessage<\'a>,')
    print('  pub unit: u8,')
    print('  pub seqno: u32,')
    print('  pub msg_len: usize,')
    print('}')
    print('')
    print('pub struct PitchReader<\'a> {')
    print('  data: &\'a [u8],')
    print('}')
    print('')
    print('pub struct PitchIterator<\'a> {')
    print('  data: &\'a [u8],')
    print('  bytes_eaten: usize,')
    print('  total_bytes: u16,')
    print('  num_messages: u8,')
    print('  unit: u8,')
    print('  seqno: u32,')
    print('}')
    print('')
    print('impl<\'a> PitchReader<\'a> {')
    print('  pub fn new(data: &\'a [u8]) -> Self { Self{data} }')
    print('  pub fn iter(&self) -> PitchIterator {')
    print('    PitchIterator{data: self.data, bytes_eaten: 0, total_bytes: 0, num_messages: 0, unit: 0, seqno: 0}')
    print('  }')
    print('}')
    print('')
    print('impl<\'a> IntoIterator for &\'a PitchReader<\'a> {')
    print('  type Item = PitchItem<\'a>;')
    print('  type IntoIter = PitchIterator<\'a>;')
    print('  fn into_iter(self) -> Self::IntoIter { self.iter() }')
    print('}')
    print('')
    print('impl<\'a> Iterator for PitchIterator<\'a> {')
    print('  type Item = PitchItem<\'a>;')
    print('  fn next(&mut self) -> Option<Self::Item> {')
    print('    if self.bytes_eaten == 0 {')
    print('      // do header')
    print('      if self.data.len() < 8 { return None; }')
    print('      let header = SequencedUnitHeader::new(self.data);')
    print('      self.total_bytes = header.hdr_length();')
    print('      self.num_messages = header.hdr_count();')
    print('      self.unit = header.hdr_unit();')
    print('      self.seqno = header.hdr_sequence();')
    print('      self.bytes_eaten += 8;')
    print('    }')
    print('    if self.num_messages == 0 { return None; }')
    print('    let bytes_remaining = self.data.len() - self.bytes_eaten;')
    print('    if bytes_remaining < 2 { return None; }')
    print('    let offset = self.bytes_eaten;')
    print('    let msg_len = self.data[offset] as usize;')
    print('    let msg_type = self.data[offset+1];')
    print('    if self.data.len() - self.bytes_eaten < msg_len {')
    print('      return None;')
    print('    }')
    print('    let seqno = self.seqno;')
    print('    self.seqno += 1;')
    print('    self.num_messages -= 1;')
    print('    self.bytes_eaten += msg_len;')
    print('    match msg_type {')
    for struct_name, struct in defs['structs'].items():
        if struct.get('id') is None:
            continue
        print('      {}::TYPE => {{'.format(struct_name))
        print('        return Some(PitchItem{{msg: PitchMessage::{}({}::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize}});'.format(struct_name, struct_name))
        print('      },')
    print('      _ => unimplemented!("message type unknown"),')
    print('    }')
    print('  }')
    print('}')
    print('')
    print('')
    print('')


def do_cracker_functions(defs):
    pass

def do_functions(datadef):
    do_cracker_functions(datadef)

def do_utils(datadef):
    pass

def do_footer(datadef):
    pass

def main():
    tree = ET.parse(sys.argv[1])
    root = tree.getroot()

    defs = create_datadef(root)
    do_typedefs(defs)
    do_header(defs)
    do_constants(defs)
    do_enums(defs)
    do_bitfields(defs)
    do_structs(defs)
    do_iterators(defs)
    do_functions(defs)
    do_utils(defs)
    do_footer(defs)
    
main()

