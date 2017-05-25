create table if not exists clients (
    uuid UUID Primary Key,
    key BYTEA,
    is_admin boolean
    )
