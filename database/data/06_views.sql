\c pharmacy

/* Products view */
CREATE VIEW product_view AS
SELECT p.id,
       p.name,
       p.description,
       i.image,
       p.price,
       p.created_at,
       p.updated_at
FROM product p
         LEFT JOIN image i ON p.id = i.product_id
ORDER BY p.id;

/* Branches view */
CREATE VIEW branch_view AS
SELECT b.id,
       b.name,
       b.address,
       b.created_at,
       b.updated_at
FROM branch b
ORDER BY b.id;

/* Profiles view */
CREATE VIEW profile_view AS
SELECT p.id,
       p.first_name,
       p.last_name,
       p.email,
       p.created_at,
       p.updated_at
FROM profile p
ORDER BY p.id;

/* Users view */
CREATE VIEW user_view AS
SELECT u.id,
       p.first_name,
       p.last_name,
       p.email,
       u.username,
       u.created_at,
       u.updated_at
FROM "user" u
         JOIN profile p ON u.profile_id = p.id
ORDER BY u.id;

/* Products Branches view */
CREATE VIEW product_branch_view AS
SELECT pb.product_id,
       pb.branch_id,
       p.name AS product_name,
       b.name AS branch_name
FROM product_branch pb
         JOIN product p ON pb.product_id = p.id
         JOIN branch b ON pb.branch_id = b.id
ORDER BY pb.product_id, pb.branch_id;

/* Orders view */
CREATE VIEW order_view AS
SELECT o.id,
       o.user_id,
       o.branch_id,
       o.total,
       o.created_at,
       o.updated_at
FROM "order" o
ORDER BY o.id;

/* Order Products view */
CREATE VIEW order_product_view AS
SELECT op.order_id,
       op.product_id,
       p.name,
       op.quantity,
       op.total,
       op.created_at,
       op.updated_at
FROM order_product op
         JOIN product p ON op.product_id = p.id
ORDER BY op.order_id, op.product_id;