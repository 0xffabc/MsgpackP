
mod msgpack_packer {

  #[allow(dead_code)]
  pub enum Lengths {
    U8(u8),
    U16(u16),
    U32(u32),
  }

  #[allow(dead_code)]
  pub enum SupportedNumbers {
    F32(f32),
    F64(f64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
  }
  
  #[allow(dead_code)]
  pub struct CoreEncode {
    pub buffer: Vec<u8>,
  }

  impl CoreEncode {
    #[allow(dead_code)]
    pub fn new() -> CoreEncode {
      CoreEncode { buffer: Vec::new() }
    }

    pub fn write(&mut self, bytes: Vec<u8>) {
      self.buffer.extend(bytes);
    }

    #[allow(dead_code)]
    pub fn start_arr(&mut self, length: Lengths) -> &mut Self {
      match length {
        Lengths::U8(len) => {
          if len < 15 {
            self.write(vec![0x90 + len]);
          } else {
            self.write(vec![0xdc, 0x00, len]);
          }
        }
        Lengths::U16(len) => {
          self.write(vec![0xdc, (len >> 8) as u8, len as u8]);
        }
        Lengths::U32(len) => {
          self.write(vec![
            0xdd,
            (len >> 24) as u8,
            (len >> 16) as u8,
            (len >> 8) as u8,
            len as u8,
          ]);
        }
      }

      self
    }

    #[allow(dead_code)]
    pub fn start_map(&mut self, length: Lengths) -> &mut Self {
      match length {
        Lengths::U8(len) => {
          if len < 15 {
            self.write(vec![0x80 + len]);
          } else {
            self.write(vec![0xde, 0x00, len]);
          }
        }
        Lengths::U16(len) => {
          self.write(vec![0xde, (len >> 8) as u8, len as u8]);
        }
        Lengths::U32(len) => {
          self.write(vec![
            0xdf,
            (len >> 24) as u8,
            (len >> 16) as u8,
            (len >> 8) as u8,
            len as u8,
          ]);
        }
      }

      self
    }

    #[allow(dead_code)]
    pub fn string(&mut self, _str: String) -> &mut Self {
      let bytes: Vec<u8> = _str.into_bytes();
      let length = bytes.len();

      match length {
        len if len <= 0xFF => {
          self.write(vec![0xd9, len as u8]);
          self.write(bytes);
        }
        len if len <= 0xFFFF => {
          let len_bytes = len.to_be_bytes();
          self.write(vec![0xda]);
          self.write(len_bytes.to_vec());
          self.write(bytes);
        }
        len if len <= 0xFFFFFFFF => {
          let len_bytes = len.to_be_bytes();
          self.write(vec![0xdb]);
          self.write(len_bytes.to_vec());
          self.write(bytes);
        }
        0_usize.. => todo!(),
      }

      self
    }

    #[allow(dead_code)]
    pub fn number(&mut self, _number: SupportedNumbers) -> &mut Self {
      match _number {
        SupportedNumbers::U8(num) => {
          self.write(vec![0xcc]);
          self.write(vec![num]);
        }
        SupportedNumbers::I8(num) => {
          self.write(vec![0xd0]);
          self.write(vec![num as u8]);
        }
        SupportedNumbers::F32(num) => {
          self.write(vec![0xca]);
          self.write(num.to_be_bytes().to_vec());
        }
        SupportedNumbers::F64(num) => {
          self.write(vec![0xcb]);
          self.write(num.to_be_bytes().to_vec());
        }
        SupportedNumbers::U16(num) => {
          self.write(vec![0xcd]);
          self.write(num.to_le_bytes().to_vec());
        }
        SupportedNumbers::U32(num) => {
          self.write(vec![0xce]);
          self.write(num.to_le_bytes().to_vec());
        }
        SupportedNumbers::U64(num) => {
          self.write(vec![0xcf]);
          self.write(num.to_le_bytes().to_vec());
        }
        SupportedNumbers::U128(num) => self.write(num.to_le_bytes().to_vec()),
        SupportedNumbers::I16(num) => {
          self.write(vec![0xd1]);
          self.write(num.to_le_bytes().to_vec());
        }
        SupportedNumbers::I32(num) => {
          self.write(vec![0xd2]);
          self.write(num.to_le_bytes().to_vec());
        }
        SupportedNumbers::I64(num) => {
          self.write(vec![0xd3]);
          self.write(num.to_le_bytes().to_vec());
        }
        SupportedNumbers::I128(num) => self.write(num.to_le_bytes().to_vec()),
      }

      self
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_codec() {
        let mut msgpack_p = msgpack_packer::CoreEncode::new();
        
        msgpack_p.start_arr(msgpack_packer::Lengths::U8(1));
        msgpack_p.string("testing string".to_owned());
        
        assert_eq!(msgpack_p.buffer, vec![145, 217, 14, 116, 101, 115, 116, 105, 110, 103, 32, 115, 116, 114, 105, 110, 103]);
    }
    
    #[test]
    fn test_number_codec() {
        let mut msgpack_p = msgpack_packer::CoreEncode::new();
        
        msgpack_p.start_arr(msgpack_packer::Lengths::U8(10));
        
        msgpack_p.number(msgpack_packer::SupportedNumbers::U8(1));
        msgpack_p.number(msgpack_packer::SupportedNumbers::U16(16555u16));
        msgpack_p.number(msgpack_packer::SupportedNumbers::U32(1424124124));
        msgpack_p.number(msgpack_packer::SupportedNumbers::U64(11512512412451224));
        
        msgpack_p.number(msgpack_packer::SupportedNumbers::I8(-1));
        msgpack_p.number(msgpack_packer::SupportedNumbers::I16(-1231));
        msgpack_p.number(msgpack_packer::SupportedNumbers::I32(-1123144));
        msgpack_p.number(msgpack_packer::SupportedNumbers::I64(-11231254112324));
        
        msgpack_p.number(msgpack_packer::SupportedNumbers::F32(0.2));
        msgpack_p.number(msgpack_packer::SupportedNumbers::F32(-0.2));
        
        msgpack_p.number(msgpack_packer::SupportedNumbers::F64(0.2));
        msgpack_p.number(msgpack_packer::SupportedNumbers::F64(-0.2));
        
        assert_eq!(vec![154, 204, 1, 205, 171, 64, 206, 220, 104, 226, 84, 207, 152, 1, 209, 172, 145, 230, 40, 0, 208, 255, 209, 49, 251, 210, 184, 220, 238, 255, 211, 188, 151, 22, 5, 201, 245, 255, 255, 202, 62, 76, 204, 205, 202, 190, 76, 204, 205, 203, 63, 201, 153, 153, 153, 153, 153, 154, 203, 191, 201, 153, 153, 153, 153, 153, 154], msgpack_p.buffer);
    }
    
    #[test]
    
    fn test_map() {
        let mut msgpack_p = msgpack_packer::CoreEncode::new();
        
        msgpack_p.start_map(msgpack_packer::Lengths::U8(3));
        
        msgpack_p.string("hi testing".to_owned());
        
        msgpack_p.start_arr(msgpack_packer::Lengths::U8(2));
        
        msgpack_p.number(msgpack_packer::SupportedNumbers::F32(0.2));
        msgpack_p.number(msgpack_packer::SupportedNumbers::F32(-0.2));
        
        msgpack_p.string("test string op".to_owned());
        msgpack_p.string("sed;alfgvknmal;eksfrgnvajkenrfgjseadfjnkrgjnsldjkefgjkndlkjnfjklgjksndfgjlkdgfkjnkljndfgkjndfgjnkdfljnkdfsglnjjnk".to_owned());
        
        msgpack_p.string("test passed? OP".to_owned());
        msgpack_p.number(msgpack_packer::SupportedNumbers::F64(-3.141592124124124124124));
        
        assert_eq!(vec![131, 217, 10, 104, 105, 32, 116, 101, 115, 116, 105, 110, 103, 146, 202, 62, 76, 204, 205, 202, 190, 76, 204, 205, 217, 14, 116, 101, 115, 116, 32, 115, 116, 114, 105, 110, 103, 32, 111, 112, 217, 113, 115, 101, 100, 59, 97, 108, 102, 103, 118, 107, 110, 109, 97, 108, 59, 101, 107, 115, 102, 114, 103, 110, 118, 97, 106, 107, 101, 110, 114, 102, 103, 106, 115, 101, 97, 100, 102, 106, 110, 107, 114, 103, 106, 110, 115, 108, 100, 106, 107, 101, 102, 103, 106, 107, 110, 100, 108, 107, 106, 110, 102, 106, 107, 108, 103, 106, 107, 115, 110, 100, 102, 103, 106, 108, 107, 100, 103, 102, 107, 106, 110, 107, 108, 106, 110, 100, 102, 103, 107, 106, 110, 100, 102, 103, 106, 110, 107, 100, 102, 108, 106, 110, 107, 100, 102, 115, 103, 108, 110, 106, 106, 110, 107, 217, 15, 116, 101, 115, 116, 32, 112, 97, 115, 115, 101, 100, 63, 32, 79, 80, 203, 192, 9, 33, 251, 13, 51, 223, 209], msgpack_p.buffer);
    }
}