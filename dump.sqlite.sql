-- TABLE
CREATE TABLE owner (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT,
    password TEXT
); 

CREATE TABLE cat (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    breed TEXT,
    owner_id INTEGER,
    FOREIGN KEY (owner_id) REFERENCES owner(id)
);

-- INDEX
CREATE INDEX idx_cat_owner_id ON cat (owner_id);
 
-- Insert data into the 'owner' table
INSERT INTO owner (username, password) VALUES
    ('Silvan', 'password123'),


-- Insert data into the 'cat' table
INSERT INTO cat (name, breed, owner_id) VALUES
    ('Garfield', 'Siamese', 1),
    ('Tiger', 'Persian', 1),
