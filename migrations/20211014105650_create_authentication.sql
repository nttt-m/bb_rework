create table authentication
(
    auth_id int not null auto_increment,
    username varchar(20) not null,
    password varchar(20) not null,
    constraint authentication_pk
        primary key (auth_id)
);