#### chat logic

- client sends bytes of chat to server without uuid
- server parses stream of chat from client
- server sends chat with uuid of client to all client streams
- client parses steam of chat from server along with uuid specifying other client


#### chat layout

- first byte is opcode specifying it's chat
- second two bytes, as big endian, state length of text string
- following bytes up to length are to be parsed as string
- UTF8 encoded
- optionally end with uuid of client
