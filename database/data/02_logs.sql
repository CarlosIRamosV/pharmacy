\c pharmacy
CREATE TABLE log.product (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    product_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.branch (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    branch_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.product_branch (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    product_id INTEGER NOT NULL,
    branch_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.profile (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    profile_id INTEGER NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.user (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL,
    profile_id INTEGER NOT NULL,
    username VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.role (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    role_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.user_role (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.order (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    order_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    branch_id INTEGER NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE log.order_product (
    id SERIAL PRIMARY KEY,
    action VARCHAR(255) NOT NULL,
    order_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
