
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

-- Fake user with secret:secret
-- fake login: admin:admin 
 INSERT INTO owner (username, password, salt) VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$SXk2cwy8pO2OMW9rnQdbAA$nzHrmzS3ima4uYqE7cxQ6TCJasuBF+OELpxxxNh8/g8', 'SXk2cwy8pO2OMW9rnQdbAA');

-- Insert data into the 'cat' table
INSERT INTO cat (name, breed, owner_id) VALUES
    ('Garfield123', 'Siamese', 1),
    ('Tiger', 'Persian', 1)
