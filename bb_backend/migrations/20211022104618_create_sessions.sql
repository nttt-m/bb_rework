create table sessions
(
    session_id int not null auto_increment,
    user_id int not null,
    token varchar(32) not null,
    constraint session_pk
        primary key (session_id)
);

create index session_index
    on sessions (token, user_id);