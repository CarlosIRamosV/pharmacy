\c pharmacy
CREATE TABLE logs.products
(
    id          SERIAL PRIMARY KEY,
    action      VARCHAR(100)   NOT NULL,
    product_id  INTEGER        NOT NULL,
    name        VARCHAR(100)   NOT NULL,
    description TEXT,
    price       DECIMAL(10, 2) NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE logs.branches
(
    id         SERIAL PRIMARY KEY,
    action     VARCHAR(100) NOT NULL,
    branch_id  INTEGER      NOT NULL,
    name       VARCHAR(100) NOT NULL,
    address    VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);