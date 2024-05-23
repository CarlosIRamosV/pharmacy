\c pharmacy

/* Products view */
CREATE VIEW products_view AS
SELECT p.id,
       p.name,
       p.description,
       p.price
FROM products p;

/* Branches view */
CREATE VIEW branches_view AS
SELECT b.id,
       b.name,
       b.address,
       b.created_at,
       b.updated_at
FROM branches b;

/* Stocks view */
CREATE VIEW stocks_view AS
SELECT s.id,
       s.product_id,
       p.name AS product_name,
       s.branch_id,
       b.name AS branch_name,
       s.quantity,
       s.created_at,
       s.updated_at
FROM stocks s
         JOIN products p ON s.product_id = p.id
         JOIN branches b ON s.branch_id = b.id;
