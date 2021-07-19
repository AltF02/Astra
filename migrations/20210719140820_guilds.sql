CREATE TABLE IF NOT EXISTS astra.guilds
(
    guild_id   bigint                NOT NULL
        constraint guilds_pk
            primary key,
    channel_id bigint                NOT NULL,
    active     boolean DEFAULT TRUE  NOT NULL,
    launches   boolean DEFAULT TRUE  NOT NULL,
    apod       boolean DEFAULT TRUE  NOT NULL,
    events     boolean DEFAULT false NOT NULL
);

ALTER TABLE astra.guilds
    OWNER TO CURRENT_USER;

CREATE UNIQUE INDEX IF NOT EXISTS guilds_guild_id_uindex
    ON astra.guilds (guild_id);
