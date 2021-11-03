create table authentication
(
    auth_id int not null auto_increment,
    username varchar(20) not null,
    salt varchar(64) not null,
    verifier VARBINARY(64) not null,
    constraint authentication_pk
        primary key (auth_id)
);

create index auth_index
    on authentication (username);