CREATE TABLE IF NOT EXISTS astra.launches
(
    launch_id   TEXT                     NOT NULL
        CONSTRAINT launches_pk
            PRIMARY KEY,
    name        TEXT                     NOT NULL,
    net         TIMESTAMP WITH TIME ZONE NOT NULL,
    vid_url     TEXT,
    image_url   TEXT,
    dispatched  BOOLEAN DEFAULT FALSE    NOT NULL,
    status      INTEGER                  NOT NULL,
    description TEXT
);

ALTER TABLE astra.launches
    OWNER TO CURRENT_USER;

CREATE UNIQUE INDEX IF NOT EXISTS launches_launch_id_uindex
    ON astra.launches (launch_id);
