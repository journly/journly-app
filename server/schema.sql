CREATE DATABASE IF NOT EXISTS 'journaly_db';
USE 'journaly_db';

CREATE USER IF NOT EXISTS 'journaly'@'localhost' IDENTIFIED BY 'user';
GRANT SELECT, INSERT, UPDATE, DELETE ON 'journaly_db'.* TO 'journaly'@'localhost';
