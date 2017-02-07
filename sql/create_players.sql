-- note: this may need to be dropped and re-added as the fields grow!

create table if not exists players (
    uuid UUID Primary Key,
    nick text
    )
