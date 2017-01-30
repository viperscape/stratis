#### login/registration process

- all communication is prefixed with an op-code as a single byte
- client id is a uuid-v4 as 20 bytes
- private key and hmac are both 16 bytes in length


##### registration
- new install generates uuid-v4 pair locally
- one of the uuid is the client id
- the other uuid is then applied to hmac with the message as the client id, this becomes the private key (private-key = hmac-sha1 [ key: uuid-v4, message: client-id ])
- save locally the key-pair, this will authenticate the client to the server
- connect to server
- client sends first byte as 1u8, followed by private key and then client id



##### login (using HMAC-SHA1)
- connect to server
- read in first 16 bytes (this is our message to be authenticated)
- use client private key to hmac-sha1 the message received
- send to server: first byte as 0u8 followed by hmac message and then client id
- server compares hmac received and recreated hmac to authenticate user (warning: is not a constant time operation)
