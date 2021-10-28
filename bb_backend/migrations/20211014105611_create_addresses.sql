create table addresses
(
    address_id int not null auto_increment,
    house_name_number varchar(60) not null,
    street varchar(50) null,
    town_city varchar(50) null,
    region varchar(50) null,
    postal_code varchar(10) not null,
    country varchar(15) not null,
    constraint addresses_pk
        primary key (address_id),
    constraint addresses_uniq
        unique (house_name_number, postal_code, country)
);

create index address_index
    on addresses (house_name_number, postal_code, country);