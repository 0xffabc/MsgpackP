# MsgpackP 

Fast msgpack implementation written in javascript and made specifically for moomoo.

Prone to most common packets used for lagging or crashing servers that use msgpack-lite.

Try it at https://0xffabc.github.io/MsgpackP/msgpackp-example.html

# Installation 

Head over to https://0xffabc.github.io/MsgpackP/codec.js to see a build of this script. Copypaste it in your website and modify it as in next patches for your needs;

## Browser usage

```patch
- let msgpack = {
+ window.msgpack = {
```

## Require.js usage

```patch
- let msgpack = {
+ module.exports = {
```

## ES6 Module usage

```patch

+ export default msgpack
```

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
Packs the data with msgpack format. Recommended for most of cases, since it's easier to control exceptions that happen while packing data.

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

Completely unsupported types: timestamp, bin, ext, float32

# Bugs

If you have found any issue, report it in issues tab
