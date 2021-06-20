create table astra.launches
(
    launch_id   text                     not null
        constraint launches_pk
            primary key,
    name        text                     not null,
    net         timestamp with time zone not null,
    vid_url     text,
    image_url   text,
    dispatched  boolean default false    not null,
    status      integer                  not null,
    description text
);

alter table astra.launches
    owner to current_user;

create unique index launches_launch_id_uindex
    on astra.launches (launch_id);

create table astra.reminders
(
    reminder_id bigserial not null
        constraint table_name_pk
            primary key,
    launch_id   text      not null
        constraint table_name_launches_launch_id_fk
            references astra.launches,
    user_id     bigint    not null
);

alter table astra.reminders
    owner to current_user;

create unique index table_name_reminder_id_uindex
    on astra.reminders (reminder_id);

create table astra.guilds
(
    guild_id   bigint                not null
        constraint guilds_pk
            primary key,
    channel_id bigint                not null,
    active     boolean default true  not null,
    launches   boolean default true  not null,
    apod       boolean default true  not null,
    events     boolean default false not null
);

alter table astra.guilds
    owner to current_user ;

create unique index guilds_guild_id_uindex
    on astra.guilds (guild_id);



