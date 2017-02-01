#### data store

If persistent storage is desired the server must be configured for this. Check the SQL folder for sql-scripts to run

- make sure postgres is installed
- create a login called stratis, with password stratis with LOGIN permission
- create database called stratis with owner as user stratis
- grant full privilege to stratis for stratis user
- create table called 'clients' with column uuid as primary indexed with UUID data type and secondary column key as a BYTEA
- create table called 'msg' with fields uuid as Uuid, and msg as BYTEA
