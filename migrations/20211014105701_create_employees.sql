create table employees
(
    id int not null auto_increment,
    first_name varchar(20) not null,
    last_name varchar(20) not null,
    position varchar(50) not null,
    auth int not null,
    address int not null,
    contact_number int not null,
    constraint employees_pk
        primary key (id),
    constraint employees_address
        foreign key (address) references addresses (address_id),
    constraint employees_telephone
        foreign key (contact_number) references telephone_numbers (telephone_number_id),
    constraint employees_auth
        foreign key (auth) references authentication (auth_id)
);

create index fn_comp_index
    on employees (first_name, last_name);

create index ln_index
    on employees (last_name);