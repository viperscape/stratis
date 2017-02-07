#### postgres as a datastore

It is currently [assumed][] postgres is the datastore for STRATIS. To setup the database either run the scripts manually or run the build program Pan. Once Pan is built, run it with either -b build flag (if the user and database are created already) or the -bi flags to also initialize the database/user login.  
NOTE: this requires the postgres admin account credentials.  

example build using postgres username and password:
```pan -bi -p postgres```


[assumed]: # "although if the DataStore trait is fulfilled for something like Redis it doesn't have to be"
