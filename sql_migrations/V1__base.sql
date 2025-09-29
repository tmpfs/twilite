CREATE TABLE IF NOT EXISTS pages
(
    page_id               INTEGER             PRIMARY KEY NOT NULL,
    created_at            DATETIME            NOT NULL,
    updated_at            DATETIME            NOT NULL,
    page_uuid             TEXT                UNIQUE NOT NULL,
    page_name             TEXT                UNIQUE NOT NULL,
    page_content          TEXT                NULL,
    page_text             TEXT                NULL,
    page_toc              TEXT                NULL
);

CREATE INDEX IF NOT EXISTS page_uuid
  ON pages (page_uuid);
CREATE INDEX IF NOT EXISTS page_name 
  ON pages (page_name);

CREATE TABLE IF NOT EXISTS files
(
    file_id               INTEGER             PRIMARY KEY NOT NULL,
    created_at            DATETIME            NOT NULL,
    updated_at            DATETIME            NOT NULL,
    file_uuid             TEXT                UNIQUE NOT NULL,
    file_name             TEXT                NOT NULL,
    file_size             INTEGER             NOT NULL,
    content_type          TEXT                NOT NULL,
    file_content          BLOB                NOT NULL
);

CREATE INDEX IF NOT EXISTS file_uuid
  ON files (file_uuid);
CREATE INDEX IF NOT EXISTS file_name
  ON files (file_name);

CREATE TABLE IF NOT EXISTS page_files
(
    page_id           INTEGER             NOT NULL,
    file_id           INTEGER             NOT NULL
);

CREATE VIRTUAL TABLE pages_fts USING fts5(
    page_name, 
    page_text, 
    content='pages', 
    content_rowid='page_id',
    tokenize = 'trigram'
);
CREATE TRIGGER pages_ai AFTER INSERT ON pages BEGIN
  INSERT INTO pages_fts(rowid, page_name, page_text)
  VALUES (new.page_id, new.page_name, new.page_text);
END;
CREATE TRIGGER pages_ad AFTER DELETE ON pages BEGIN
  INSERT INTO pages_fts(pages_fts, rowid, page_name, page_text)
  VALUES('delete', old.page_id, old.page_name, old.page_text);
END;
CREATE TRIGGER pages_au AFTER UPDATE ON pages BEGIN
  -- First delete old entry
  INSERT INTO pages_fts(pages_fts, rowid, page_name, page_text)
  VALUES('delete', old.page_id, old.page_name, old.page_text);
  -- Then insert updated entry
  INSERT INTO pages_fts(rowid, page_name, page_text)
  VALUES (new.page_id, new.page_name, new.page_text);
END;
