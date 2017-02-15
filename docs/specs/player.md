#### player logic

- client sends update to server of player struct, encoded but without uuid -- it's assumed
- on player update player struct is encoded followed by uuid of client, and sent to all players
- client updates local players cache upon receiving

#### player layout

- first byte is opcode for player
- second chunk corresponds to nickname
- use same method as chat to encode string with its length
- optionally end with uuid of client
