create table telephone_numbers
(
    telephone_number_id int auto_increment,
    telephone_number int not null,
    constraint telephoneNumbers_pk
        primary key (telephone_number_id)
);

create unique index telephone_numbers_uindex
    on telephone_numbers (telephone_number);