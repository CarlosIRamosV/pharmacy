\c pharmacy
CREATE TABLE products
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100)   NOT NULL,
    description TEXT,
    price       DECIMAL(10, 2) NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE branches
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    address    VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);