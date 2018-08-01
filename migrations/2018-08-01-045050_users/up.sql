-- Your SQL goes here
CREATE TABLE users (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  `name` VARCHAR(128) NOT NULL,
  `email` VARCHAR(128) NOT NULL,
  `password_hash` VARCHAR(128) NOT NULL
);
