\c pharmacy
CREATE TABLE product
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100)   NOT NULL,
    description TEXT,
    price       DECIMAL(10, 2) NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE image
(
    id         SERIAL PRIMARY KEY,
    product_id INTEGER      NOT NULL UNIQUE,
    image      BYTEA        NOT NULL,
    hash       VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (product_id) REFERENCES product (id)
);

CREATE TABLE branch
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100) NOT NULL UNIQUE,
    address    VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE product_branch
(
    product_id INTEGER NOT NULL,
    branch_id  INTEGER NOT NULL,
    PRIMARY KEY (product_id, branch_id),
    FOREIGN KEY (product_id) REFERENCES product (id),
    FOREIGN KEY (branch_id) REFERENCES branch (id)
);

CREATE TABLE profile
(
    id         SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE "user"
(
    id         SERIAL PRIMARY KEY,
    profile_id INTEGER      NOT NULL,
    username   VARCHAR(100) NOT NULL,
    password   VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (profile_id) REFERENCES profile (id)
);

CREATE TABLE role
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE user_role
(
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES "user" (id),
    FOREIGN KEY (role_id) REFERENCES role (id)
);

CREATE TABLE "order"
(
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER        NOT NULL,
    branch_id  INTEGER        NOT NULL,
    total      DECIMAL(10, 2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP               DEFAULT NOW(),
    updated_at TIMESTAMP               DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES "user" (id),
    FOREIGN KEY (branch_id) REFERENCES branch (id)
);

CREATE TABLE order_product
(
    id         SERIAL PRIMARY KEY,
    order_id   INTEGER        NOT NULL,
    product_id INTEGER        NOT NULL,
    quantity   INTEGER        NOT NULL,
    total      DECIMAL(10, 2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP               DEFAULT NOW(),
    updated_at TIMESTAMP               DEFAULT NOW(),
    FOREIGN KEY (order_id) REFERENCES "order" (id),
    FOREIGN KEY (product_id) REFERENCES product (id)
);