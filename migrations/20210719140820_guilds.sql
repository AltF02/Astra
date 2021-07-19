CREATE TABLE IF NOT EXISTS astra.guilds
(
    guild_id   BIGINT                NOT NULL
        CONSTRAINT guilds_pk
            PRIMARY KEY,
    channel_id BIGINT                NOT NULL,
    active     BOOLEAN DEFAULT TRUE  NOT NULL,
    launches   BOOLEAN DEFAULT TRUE  NOT NULL,
    apod       BOOLEAN DEFAULT TRUE  NOT NULL,
    events     BOOLEAN DEFAULT false NOT NULL
);

ALTER TABLE astra.guilds
    OWNER TO CURRENT_USER;

CREATE UNIQUE INDEX IF NOT EXISTS guilds_guild_id_uindex
    ON astra.guilds (guild_id);
