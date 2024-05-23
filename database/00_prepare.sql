/* Create database */
CREATE DATABASE pharmacy
    WITH OWNER = postgres
    ENCODING = 'UTF8'
    CONNECTION LIMIT = -1;

\c pharmacy
CREATE SCHEMA logs;