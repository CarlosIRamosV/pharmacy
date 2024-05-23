\c pharmacy

CREATE ROLE branches_user WITH LOGIN PASSWORD 'branches_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON ALL TABLES IN SCHEMA public
    TO branches_user;

/* Products table */
CREATE ROLE products_user WITH LOGIN PASSWORD 'products_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON ALL TABLES IN SCHEMA public
    TO products_user;

/* Create schema */
CREATE USER stocks_user WITH LOGIN PASSWORD 'stocks_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON ALL TABLES IN SCHEMA public
    TO stocks_user;
