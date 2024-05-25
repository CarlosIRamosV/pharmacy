\c pharmacy


/* Branches table */

CREATE ROLE branches_user WITH LOGIN PASSWORD 'branches_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON TABLE branches, branches_view
    TO branches_user;

GRANT USAGE, SELECT
    ON SEQUENCE branches_id_seq
    TO branches_user;

GRANT USAGE
    ON SCHEMA logs
    TO branches_user;

GRANT INSERT, SELECT
    ON TABLE logs.branches
    TO branches_user;

GRANT USAGE, SELECT
    ON SEQUENCE logs.branches_id_seq
    TO branches_user;


/* Products table */
CREATE ROLE products_user WITH LOGIN PASSWORD 'products_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON TABLE products, products_view
    TO products_user;

GRANT USAGE, SELECT
    ON SEQUENCE products_id_seq
    TO products_user;

GRANT USAGE
    ON SCHEMA logs
    TO products_user;

GRANT INSERT, SELECT
    ON TABLE logs.products
    TO products_user;

GRANT USAGE, SELECT
    ON SEQUENCE logs.products_id_seq
    TO products_user;