#### pan build/service tool

Pan is designed to build the required database and user for STRATIS to run. Eventually Pan will also startup and manage lifecycle of the STRATIS server and related services.

##### why?

Pan's purpose is to centralize and simplify management of STRATIS on multiple platforms, without necessitating to OS specific scripts. It originally was born with the [VNDF][] game project and [@hannobraun][] creative mind, and while the code is not the same the concept is.

##### example

build database using postgres username and password:
```pan -bi -p postgres```

[@hannobraun]: https://github.com/hannobraun
[VNDF]: https://github.com/hannobraun/vndf
