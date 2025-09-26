CREATE TABLE IF NOT EXISTS pages
(
    page_id               INTEGER             PRIMARY KEY NOT NULL,
    created_at            DATETIME            NOT NULL,
    updated_at            DATETIME            NOT NULL,
    page_uuid             TEXT                UNIQUE NOT NULL,
    page_name             TEXT                UNIQUE NOT NULL,
    page_content          TEXT                NULL,
    page_text             TEXT                NULL
);

CREATE TABLE IF NOT EXISTS files
(
    file_id               INTEGER             PRIMARY KEY NOT NULL,
    created_at            DATETIME            NOT NULL,
    updated_at            DATETIME            NOT NULL,
    file_uuid             TEXT                UNIQUE NOT NULL,
    file_name             TEXT                UNIQUE NOT NULL,
    content_type          TEXT                NULL,
    file_content          TEXT                NULL
);
