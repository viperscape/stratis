#### pan build/service tool

Pan is designed to build the required database and user for STRATIS to run. Pan will also startup and manage lifecycle of the STRATIS server (DEBUG) and related services.

##### why?

Pan's purpose is to centralize and simplify management of STRATIS on multiple platforms, without necessitating to OS specific scripts. It originally was born with the [VNDF][] game project and [@hannobraun][] creative mind, and while the code is not the same the concept is.

##### build database

build database using postgres username and password:  
```pan -bi -p postgres```

##### lifecycle management

In order to manage the STRATIS server, Environment Variable must first be present:  
```STRATIS``` with a value of the folder where the stratis source is

to run the watcher for the debug build:  
```pan -wd```

[@hannobraun]: https://github.com/hannobraun
[VNDF]: https://github.com/hannobraun/vndf
