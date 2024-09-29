# msgpackp

Fast msgpack implementation written in javascript and made specifically for moomoo
Prone to most common packets used for lagging or crashing servers that use msgpack-lite

# Documentation 

## _encode(data: any[] | any except function) -> Uint8Array
Synchronously packs data with msgpack format (not recommended)

```js
const packet = msgpack._encode({
  thing: 5.88283882919
});
socket.send(packet);
```

## _decode(data: any[] | ArrayBuffer) -> any
Synchronously unpacks data with msgpack format (not recommended)

```js
socket.on("message", ({ data }) => {
  console.log(msgpack._decode(data));
});
```

## async pack(data: any[] | any except function) -> Uint8Array 
Packs the data with msgpack format. Recommended for most of cases, since it's easier to control exceptions that happen while packing data

```js
const socket = new WebSocketStream("wss://127.0.0.1");
const { readable, writable, extensions, protocol } = await wss.opened;

const writer = writable.getWriter();

msgpack.pack("hello").then(raw =>
  writer.write(raw)).catch(console.log);

```

## async unpack(data: any[] | ArrayBuffer) -> any
Unpacks the data with msgpack format. Recommended for most of cases, since it's easier to control exceptions that happen while packing data.

```js
const socketWorker = new Worker("ws.js");

socketWorker.onmessage = async ({ data }) => {
  const [ packetSid, packetData ] = await msgpack.unpack(data);

  console.log(packetSid, packetData);
}

socketWorker.postMessage("ready");
```

# Limitations 

Completely unsupported types: timestamp, bin, ext

Partial support: str(8, 16, 32), arr(16, 32), map(16, 32) - not supported for decoding

# Bugs

If you have found any issue, report it in issues tab
