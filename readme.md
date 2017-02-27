#### STRATIS

Strategic Interstellar System (acronym may change), or STRATIS for short, is an online game combining strategy, resource management, trade, and factions. Currently under development of the full span of what STRATIS is and will be.

##### immediate planned features

- online gameplay
- messaging/chat direct to user, factions, regions' trade-post bulletin boards
- minimally scripted AI interaction (eg: press 1 for asking '...')
- fleet inventory management and deployment
- trade with AI trading posts, between players *at trade posts*, faction trading contracts

##### future planned features

- online battle with battle ships, ground destroyers
- scripted events (natural disasters: planetary and stellar)
- defense, scouts, probes
- analytics of resources

##### start-here

To get going building this project, head over to the sub project Pan and run ```cargo build``` then from the root directory run Pan. Once you have set up your postgres database (use pan for this), build and run the server sub project. If you run pan with the -r flag, it will automatically build and run the server for you. If interested in using the ffi, say from Unity3d, then head over to ffi sub project and build it. Pan should copy the output dll to unity_ffi sub project and then follow directions accordingly. The ffi could also be used from UE4, head over to misc to see an example.
