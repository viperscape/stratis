#### event stream

##### network

- clients connected will receive data packed manually as per spec, each buffer to be parsed is prefixed with an opcode
- opcode dictates parsing logic and action

##### ffi

- events parsed from network will be packaged into an event stream using a channel
- channel events are in the form of basic opcodes with uuids referencing object
- object is stored in rust-side as a cache
- ffi polls channel on game tick
- ffi copies cache when needed
