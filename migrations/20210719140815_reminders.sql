CREATE TABLE IF NOT EXISTS astra.reminders
(
    reminder_id BIGSERIAL NOT NULL 
        CONSTRAINT table_name_pk
            PRIMARY KEY,
    launch_id   TEXT      NOT NULL
        CONSTRAINT table_name_launches_launch_id_fk
            REFERENCES astra.launches,
    user_id     BIGINT    NOT NULL
);

ALTER TABLE astra.reminders
    OWNER TO CURRENT_USER;

CREATE UNIQUE INDEX IF NOT EXISTS table_name_reminder_id_uindex
    ON astra.reminders (reminder_id);