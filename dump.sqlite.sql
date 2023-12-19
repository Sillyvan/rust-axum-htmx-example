
-- Create the 'owner' table if not exists
CREATE TABLE IF NOT EXISTS owner (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE,
    password TEXT,
    salt TEXT
);

-- Create the 'cat' table if not exists
CREATE TABLE IF NOT EXISTS cat (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    breed TEXT,
    owner_id INTEGER,
    FOREIGN KEY (owner_id) REFERENCES owner(id)
);

-- Create the 'idx_cat_owner_id' index if not exists
CREATE INDEX IF NOT EXISTS idx_cat_owner_id ON cat (owner_id);

-- insert fake user
INSERT INTO owner (username, password, salt) VALUES ('admin', 'admin', 'admin');

-- Insert data into the 'cat' table
INSERT INTO cat (name, breed, owner_id) VALUES
    ('Garfield', 'Siamese', 1),
    ('Tiger', 'Persian', 1),
    ('Tiger', 'Persian', 1),
    ('Tiger3', 'Persian', 1),
    ('Tiger4', 'Persian', 1),
    ('Meow', 'Cat', 2);