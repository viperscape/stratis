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

To run the watcher for the debug build:  
```pan -wd```

If you are building the client for Unity3d, make sure ```STRATIS_UNITY``` points to your unity3d project, and you can run ```pan -rwd``` to have it build and run the server, then when you build the unity subproject the dynamic library will be copied into the unity_ffi subproject. At this point you must manually copy this Assets folder to your unity game project.

[@hannobraun]: https://github.com/hannobraun
[VNDF]: https://github.com/hannobraun/vndf
