#### chat

payload must be laid out as follows
    
- first byte is 2 signifiying chat text route
- size of text is stated as the second two bytes as u16 in bigendian format
- following bytes are text payload (expects utf8)
- if client is receiving then following 16 bytes represent UUID of player, sent from server
