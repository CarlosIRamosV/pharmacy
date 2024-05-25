\c pharmacy

/* Products view */
CREATE VIEW products_view AS
SELECT p.id,
       p.name,
       p.description,
       p.price,
       p.created_at,
       p.updated_at
FROM products p
ORDER BY p.id;

/* Branches view */
CREATE VIEW branches_view AS
SELECT b.id,
       b.name,
       b.address,
       b.created_at,
       b.updated_at
FROM branches b
ORDER BY b.id;
