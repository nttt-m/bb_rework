create table roles
(
    role_id int not null auto_increment,
    role_name varchar(20) not null,
    constraint roles_pk
        primary key (role_id)
)