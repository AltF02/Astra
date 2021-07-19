CREATE TABLE IF NOT EXISTS astra.launches
(
    launch_id   TEXT                     NOT NULL
        constraint launches_pk
            primary key,
    name        TEXT                     NOT NULL,
    net         TIMESTAMP WITH TIME ZONE NOT NULL,
    vid_url     TEXT,
    image_url   TEXT,
    dispatched  boolean default false    NOT NULL,
    status      integer                  NOT NULL,
    description TEXT
);

ALTER TABLE astra.launches
    OWNER TO CURRENT_USER;

CREATE UNIQUE INDEX IF NOT EXISTS launches_launch_id_uindex
    ON astra.launches (launch_id);
