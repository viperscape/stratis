#### data store

If persistent storage is desired the server must be configured for this.

- make sure postgres is installed
- create a login called stratis, with password stratis with LOGIN permission
- create database called stratis with owner as user stratis
- create table called 'clients' with column uuid as primary indexed with UUID data type and secondary column key as a BYTEA
