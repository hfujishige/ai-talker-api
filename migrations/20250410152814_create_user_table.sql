/*
 This SQL script creates a table named "users" with the following columns:
 - id: an auto-incrementing integer that serves as the primary key
 - username: a string that cannot be null
 - email: a string that cannot be null
 - password: a string that cannot be null
 - created_at: a timestamp that defaults to the current timestamp
 - updated_at: a timestamp that defaults to the current timestamp and updates on row modification
 It also creates two indexes on the username and email columns for faster lookups.
-- PostgreSQL script to create a users table with indexes   
*/
-- Drop the table if it exists
DROP TABLE IF EXISTS users;
-- Create the users table
CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   login_id VARCHAR(50) NOT NULL,
   name VARCHAR(100) NOT NULL,
   email VARCHAR(320) NOT NULL,  -- RFC 5321, 5322 defined max length
   password VARCHAR(255) NOT NULL,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create an index on the username column for faster lookups
CREATE INDEX idx_username ON users(name);
-- Create an index on the email column for faster lookups
CREATE INDEX idx_email ON users(email);