create table employees
(
    id int not null auto_increment,
    first_name varchar(20) not null,
    last_name varchar(20) not null,
    dob date not null,
    address int not null,
    contact_number int not null,
    position varchar(50) not null,
    authn int not null,
    authz bool not null,
    pending bool not null,
    constraint employees_pk
        primary key (id),
    constraint employees_address
        foreign key (address) references addresses (address_id),
    constraint employees_auth
        foreign key (authn) references authentication (auth_id)
);

create index fn_comp_index
    on employees (first_name, last_name);

create index ln_index
    on employees (last_name);

create index pos_index
    on employees (position);