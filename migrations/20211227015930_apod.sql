-- Add migration script here
create table astra.apod
(
    id           serial
        constraint apod_pk
            primary key,
    publish_date date                  not null,
    explanation  text,
    title        text                  not null,
    hdurl        text,
    copyright    text    default 'NASA'::text,
    dispatched   boolean default false not null
);

alter table astra.apod
    owner to postgres;

create unique index apod_id_uindex
    on astra.apod (id);

create unique index apod_publish_date_uindex
    on astra.apod (publish_date);

