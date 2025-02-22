-- Create the following tables
-- 1. Resolver :: key = string, value = json
-- 2. Program :: key = string, value = bytea
-- 3. Driver  :: key = string, value = bytea

CREATE TABLE IF NOT EXISTS Resolver (
    path TEXT NOT NULL PRIMARY KEY,
    path_info TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Program (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    component BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS Driver (
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    component BLOB NOT NULL,
    PRIMARY KEY (name, version)
);

CREATE TABLE IF NOT EXISTS User (
    user_name TEXT NOT NULL,
    user_id TEXT NOT NULL PRIMARY KEY,
    password TEXT NOT NULL,
    account_address TEXT
);
