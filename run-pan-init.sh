#!/bin/sh

## this will start the pan app service and intialize the postgres database (assuming you have it installed)
## see ./pan/readme.md documentation for more about this
## if you have Windows, download git tools which includes gitbash, and can run shell scripts

pan -bi -p postgres
