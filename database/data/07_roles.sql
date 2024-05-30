\c pharmacy


/* Branches table */
CREATE ROLE branches_user WITH LOGIN PASSWORD 'branches_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON TABLE branch, branch_view
    TO branches_user;

GRANT USAGE, SELECT
    ON SEQUENCE branch_id_seq
    TO branches_user;

GRANT USAGE
    ON SCHEMA log
    TO branches_user;

GRANT INSERT, SELECT
    ON TABLE log.branch
    TO branches_user;

GRANT USAGE, SELECT
    ON SEQUENCE log.branch_id_seq
    TO branches_user;


/* Products table */
CREATE ROLE products_user WITH LOGIN PASSWORD 'products_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON TABLE product, product_view
    TO products_user;

GRANT USAGE, SELECT
    ON SEQUENCE product_id_seq
    TO products_user;

GRANT USAGE
    ON SCHEMA log
    TO products_user;

GRANT INSERT, SELECT
    ON TABLE log.product
    TO products_user;

GRANT USAGE, SELECT
    ON SEQUENCE log.product_id_seq
    TO products_user;

/* Images role */
CREATE ROLE images_user WITH LOGIN PASSWORD 'images_password';
GRANT INSERT, SELECT, UPDATE, DELETE
    ON TABLE image
    TO images_user;

GRANT USAGE, SELECT
    ON SEQUENCE image_id_seq
    TO images_user;
