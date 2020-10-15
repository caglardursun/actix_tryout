
drop table if exists my_list_details;
drop table if exists my_list;

create table my_list 
(
    id serial primary key,
    title varchar(150) not null
);

create table my_list_details
(
    id serial primary key,
    title varchar(150) not null,    
    checked boolean not null default false,
    list_id integer not null,
    foreign key (my_list_id) references my_list(id)

);

insert into my_list (title) values ('List A'), ('List B'), ('List My Ass');
insert into my_list_details (title, list_id) 
    values ('Connect 2 database', 1), ('Some queries comming', 1);