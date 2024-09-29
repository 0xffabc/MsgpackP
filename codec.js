class CoreEncode {
  buffer = [];
  
  byteSeq(base, bytes) {
    this.write(base + bytes.length,
                    ...bytes);
    return this;
  }
  
  write(...bytes) {
    this.buffer.push(...bytes);
  }
  
  boolean(type) {
    this.write(type ? 0xC3 : 0xC2);
    return this;
  }
  
  nil() {
    this.write(0xc0);
    return this;
  }
  
  start_arr(len) {
    if (len > 15 && len < (2 ** 16) - 1) {
      this.write(0xdc, len >>> 8, len);
    } else if (len < 15)
      this.write(0x90 + len);
    else {
      this.write(0xdd, len >>> 24, len >>> 16, len >>> 8, len);
    }
    
    return this;
  }
  
  fixstr(str) {
    const strenc = new TextEncoder("utf8").encode(str);
    if (strenc.byteLength <= 31) {
      return this.byteSeq(0xa0, strenc);
    } else if (strenc.byteLength < (2 ** 8) - 1) {
      this.write(0xd9, strenc.byteLength, ...strenc);
    } else if (strenc.byteLength < (2 ** 16) - 1) {
      this.write(0xda, strenc.byteLength >>> 8, strenc.byteLength, ...strenc);
    } else if (strenc.byteLength < (2 ** 32) - 1) {
      this.write(0xdb, strenc.byteLength >>> 24, strenc.byteLength >>> 16, strenc.byteLength >>> 8, strenc.byteLength, ...strenc);
    }
    
    return this;
  }
  
  map(length) {
    if (length > 15 && length < (2 ** 16) - 1) {
      this.write(0xde, length >>> 8, length);
    } else if (length < 15)
      this.write(0x80 + length);
    else {
      this.write(0xdf, num >>> 24, num >>> 16, num >>> 8, data);
    }
    
    return this;
  }
  
  add_num(num) {
    if (!Number.isInteger(num)) {
      const buffer = new ArrayBuffer(8);
      const view = new DataView(buffer);
      view.setFloat64(0, num);
        
      this.write(0xcb, ...new Uint8Array(view.buffer));
      
      return this;
    }

    if (num == 0) this.write(0);
    else if (num > 0) { 
      if (num < 127) {
        this.write(num);
      } else if (num < 256) {
        this.write(0xcc, num);
      } else if (num < 65535) {
        this.write(0xcd, num >>> 8, num);
        } else if (num < 4294967296) {
          this.write(0xce, num >>> 24, num >>> 16, num >>> 8, num);
        } else if (num <= 18446744073709552000) {
          const buffer = new ArrayBuffer(8);
          const view = new DataView(buffer);
          view.setBigInt64(0, BigInt(num));
          
          this.write(0xcf, ...new Uint8Array(view.buffer));
        } else this.write(0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
      } else {
      if (num > -128) {
        this.write(0xd0, num);
      } else if (num > -32768) {
        this.write(0xd1, num >>> 8, num);
      } else if (num > -4294967296) {
          this.write(0xd2, num >>> 24, num >>> 16, num >>> 8, data);
        } else if (num >= -18446744073709552000) {
          const buffer = new ArrayBuffer(8);
          const view = new DataView(buffer);
          view.setBigInt64(0, BigInt(num));
          
          this.write(0xd3, ...new Uint8Array(view.buffer));
        } else this.write(0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    }
    
    return this;
  }
  
  encode(data, noReset) {
    if (!noReset) this.buffer.length = 0;
    if (data?.constructor?.name == "Array") {
      this.start_arr(data.length);
      data.forEach(_ => this.encode(_, true));
    } else if (typeof data == "object" && !!data) {
      const keys = Object.keys(data);
      const vals = Object.values(data);
      this.map(keys.length);
      
      for (let i = 0; i < keys.length; i++) {
        this.encode(keys[i], true);
        this.encode(vals[i], true);
      }
    } else if (typeof data == "string") this.fixstr(data);
    else if (typeof data == "number") this.add_num(data);
    else if (typeof data == "boolean") this.boolean(data);
    else if (typeof data == "undefined" || isNaN(data) || data == null) this.nil();
    else throw new TypeError("Unknown type: " + typeof data);
    
    return new Uint8Array(this.buffer);
  }
}

class CoreDecode {
  buffer = [];
  offset = 0;
  
  readByte() {
    return this.buffer[this.offset++];
  }
  
  readBytes(amount) {
    return this.buffer.slice(this.offset, this.offset += amount);
  }
  
  skip(amount) {
    this.offset += amount;
  }
  
  decode(buff) {
    if (buff) {
      this.buffer = buff;
      this.view = new DataView(new Uint8Array(this.buffer).buffer);
      this.offset = 0;
    }
    
    const byte = this.readByte();
    if (byte >= 0xa0 && byte <= 0xbf) {
      const length = byte - 0xa0;
      return String.fromCharCode(...this.readBytes(length));
    } else if (byte >= 0x90 && byte <= 0x9f) {
      const length = byte - 0x90;
      const array = [];
      for (let i = 0; i < length; i++) {
        array.push(this.decode());
      }
      return array;
    } else if (byte >= 0x80 && byte <= 0x8f) {
      const length = byte - 0x80;
      const map = {};
      for (let i = 0; i < length; i++) {
        const key = this.decode();
        const value = this.decode();
        map[key] = value;
      }
      return map;
    } else if (byte > 0 && byte < 0x7f) {
      return byte;
    } else if (byte == 0xcc) {
      return byte;
    } else if (byte == 0xcd) {
      const res = this.view.getUint16(this.offset);
      this.skip(2);
      return res;
    } else if (byte == 0xce) {
      const res = this.view.getUint32(this.offset);
      this.skip(4);
      return res;
    } else if (byte == 0xcf) {
      const res = this.view.getBigInt64(this.offset);
      this.skip(8);
      return Number(res);
    } else if (byte > 0xe0 && byte < 0xff) {
      return -(0xe0 + byte);
    } else if (byte == 0xd0) {
      const res = -this.readByte();
      //this.skip(1);
      return res;
    } else if (byte == 0xd1) {
      const res = this.view.getInt16(this.offset);
      this.skip(2);
      return res;
    } else if (byte == 0xd2) {
      const res = this.view.getInt32(this.offset);
      this.skip(4);
      return res;
    } else if (byte == 0xd3) {
      const res = this.view.getBigInt64(this.offset);
      this.skip(8);
      return Number(res);
    } else if (byte == 0xcb) {
      const res = this.view.getFloat64(this.offset);
      this.skip(8);
      return res;
    } else if (byte == 0xc3) return true;
    else if (byte == 0xc2) return false;
    else if (byte == 0xc0) return null;
    else return byte;
  }
}

const encoder = new CoreEncode();
const decoder = new CoreDecode();

const msgpack = {
  pack(data) {
    return new Promise((accept, reject) => {
      try {
        accept(this._encode(data));
      } catch(e) {
        reject(e);
      }
    })
  },
  unpack(buffer) {
    return new Promise((accept, reject) => {
      try {
        accept(this._decode(data));
      } catch(e) {
        reject(e);
      }
    });
  },
  _encode: encoder.encode.bind(encoder),
  _decode: decoder.decode.bind(decoder)
};
