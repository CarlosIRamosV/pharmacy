\c pharmacy

/* Products */
INSERT INTO product (name, description, price)
VALUES ('Paracetamol', 'Painkiller', 10.00),
       ('Ibuprofen', 'Painkiller', 15.00),
       ('Aspirin', 'Painkiller', 5.00),
       ('Omeprazole', 'Antacid', 20.00),
       ('Lansoprazole', 'Antacid', 25.00),
       ('Ranitidine', 'Antacid', 30.00);

/* Branches */
INSERT INTO branch (name, address)
VALUES ('Branch 1', 'Address 1'),
       ('Branch 2', 'Address 2'),
       ('Branch 3', 'Address 3');

/* Product Branches */
INSERT INTO product_branch (product_id, branch_id)
VALUES (1, 1),
       (2, 1),
       (3, 1),
       (1, 2),
       (2, 2),
       (3, 2),
       (4, 2),
       (5, 2),
       (6, 2);

/* Profiles */
INSERT INTO profile (first_name, last_name, email)
VALUES ('John', 'Doe', 'john@aa.com'),
       ('Alice', 'Smith', 'alice@aa.com'),
       ('Bob', 'Brown', 'bob@aa.com');

/* Users */
INSERT INTO "user" (profile_id, username, password)
VALUES (1, 'john', 'john'),
       (2, 'alice', 'alice'),
       (3, 'bob', 'bob');

/* Roles */
INSERT INTO role (name)
VALUES ('Admin'),
       ('User');

/* User Roles */
INSERT INTO user_role (user_id, role_id)
VALUES (1, 1),
       (2, 2),
       (3, 2);

/* Orders */
INSERT INTO "order" (user_id, branch_id)
VALUES (1, 1),
       (2, 2),
       (3, 3);

/* Order Products */
INSERT INTO order_product (order_id, product_id, quantity)
VALUES (1, 1, 1),
       (1, 2, 2),
       (2, 3, 3),
       (2, 4, 4),
       (3, 5, 5),
       (3, 6, 6);

UPDATE "order"
SET total = DEFAULT;


