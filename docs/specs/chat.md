#### chat

payload must be laid out as follows
    
- first byte is 2 signifiying chat text route
- size of text is stated as the second two bytes as u16 in bigendian format
- following bytes are text payload (expects utf8)

TBD: usernames/user-id references