\c pharmacy

/* Products */

/* Branches */

/* Stocks */
-- Trigger function
CREATE FUNCTION logs.fn_stocks_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO logs.stocks (action, stock_id, product_id, branch_id, quantity)
        VALUES ('INSERT', NEW.id, NEW.product_id, NEW.branch_id, NEW.quantity);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO logs.stocks (action, stock_id, product_id, branch_id, quantity)
        VALUES ('UPDATE', NEW.id, NEW.product_id, NEW.branch_id, NEW.quantity);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO logs.stocks (action, stock_id, product_id, branch_id, quantity)
        VALUES ('DELETE', OLD.id, OLD.product_id, OLD.branch_id, OLD.quantity);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Insert stock and return the stock
CREATE FUNCTION fn_stocks_insert(p_product_id INTEGER, p_branch_id INTEGER, p_quantity INTEGER)
    RETURNS TABLE
            (
                id           INTEGER,
                product_id   INTEGER,
                product_name VARCHAR(100),
                branch_id    INTEGER,
                branch_name  VARCHAR(100),
                quantity     INTEGER,
                created_at   TIMESTAMP,
                updated_at   TIMESTAMP
            )
AS
$$
DECLARE
    v_stock stocks%ROWTYPE;
BEGIN

    IF NOT EXISTS (SELECT 1 FROM products WHERE id = p_product_id AND branch_id = p_branch_id) THEN
        RAISE EXCEPTION 'Product not found';
    END IF;

    INSERT INTO stocks (product_id, branch_id, quantity)
    VALUES (p_product_id, p_branch_id, p_quantity)
    RETURNING * INTO v_stock;

    RETURN QUERY
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
                 JOIN branches b ON s.branch_id = b.id
        WHERE s.id = v_stock.id;
END;
$$ LANGUAGE plpgsql;

-- Update stock and return the stock
CREATE FUNCTION fn_stocks_update(p_id INTEGER, p_quantity INTEGER)
    RETURNS TABLE
            (
                id           INTEGER,
                product_id   INTEGER,
                product_name VARCHAR(100),
                branch_id    INTEGER,
                branch_name  VARCHAR(100),
                quantity     INTEGER,
                created_at   TIMESTAMP,
                updated_at   TIMESTAMP
            )
AS
$$
DECLARE
    v_stock stocks%ROWTYPE;
BEGIN
    UPDATE stocks
    SET quantity   = p_quantity,
        updated_at = NOW()
    WHERE stocks.id = p_id
    RETURNING * INTO v_stock;

    RETURN QUERY
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
                 JOIN branches b ON s.branch_id = b.id
        WHERE s.id = v_stock.id;
END;
$$ LANGUAGE plpgsql;

-- Delete stock and return the stock
CREATE FUNCTION fn_stocks_delete(p_id INTEGER)
    RETURNS TABLE
            (
                id           INTEGER,
                product_id   INTEGER,
                product_name VARCHAR(100),
                branch_id    INTEGER,
                branch_name  VARCHAR(100),
                quantity     INTEGER,
                created_at   TIMESTAMP,
                updated_at   TIMESTAMP
            )
AS
$$
DECLARE
    v_stock stocks%ROWTYPE;
BEGIN
    DELETE
    FROM stocks
    WHERE stocks.id = p_id
    RETURNING * INTO v_stock;

    RETURN QUERY
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
                 JOIN branches b ON s.branch_id = b.id
        WHERE s.id = v_stock.id;
END;
$$ LANGUAGE plpgsql;


/*
/* Products */
-- Trigger function
CREATE FUNCTION logs.fn_products_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO logs.products (action, product_id, name, description, price)
        VALUES ('INSERT', NEW.id, NEW.name, NEW.description, NEW.price);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO logs.products (action, product_id, name, description, price)
        VALUES ('UPDATE', NEW.id, NEW.name, NEW.description, NEW.price);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO logs.products (action, product_id, name, description, price)
        VALUES ('DELETE', OLD.id, OLD.name, OLD.description, OLD.price);
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Select products by id
CREATE FUNCTION fn_products_view_select_by_id(p_id INTEGER)
    RETURNS TABLE
            (
                id          INTEGER,
                name        VARCHAR(100),
                description TEXT,
                price       DECIMAL(10, 2)
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT p.id,
               p.name,
               p.description,
               p.price
        FROM products p
        WHERE p.id = p_id;
END;
$$ LANGUAGE plpgsql;

-- Search products by name and price
CREATE FUNCTION fn_products_view_filters_name_price(p_name VARCHAR(100), p_min_price DECIMAL, p_max_price DECIMAL,
                                                    p_limit INTEGER, p_offset INTEGER)
    RETURNS TABLE
            (
                id          INTEGER,
                name        VARCHAR(100),
                description TEXT,
                price       DECIMAL(10, 2)
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT p.id,
               p.name,
               p.description,
               p.price
        FROM products p
        WHERE p.name ILIKE '%' || p_name || '%'
          AND p.price BETWEEN p_min_price AND p_max_price
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;


-- Search products by name, price and branch
CREATE FUNCTION fn_products_view_filters_name_price_branch(p_name VARCHAR(100), p_min_price DECIMAL,
                                                           p_max_price DECIMAL, p_branch_id INTEGER, p_limit INTEGER,
                                                           p_offset INTEGER)
    RETURNS TABLE
            (
                id          INTEGER,
                name        VARCHAR(100),
                description TEXT,
                price       DECIMAL(10, 2)
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT p.id,
               p.name,
               p.description,
               p.price
        FROM products p
                 JOIN stocks s ON p.id = s.product_id
        WHERE p.name ILIKE '%' || p_name || '%'
          AND p.price BETWEEN p_min_price AND p_max_price
          AND s.branch_id = p_branch_id
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;


/* Branches */
-- Trigger function
CREATE FUNCTION logs.fn_branches_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO logs.branches (action, branch_id, name, address)
        VALUES ('INSERT', NEW.id, NEW.name, NEW.address);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO logs.branches (action, branch_id, name, address)
        VALUES ('UPDATE', NEW.id, NEW.name, NEW.address);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO logs.branches (action, branch_id, name, address)
        VALUES ('DELETE', OLD.id, OLD.name, OLD.address);
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Select branches by id
CREATE FUNCTION fn_branches_view_select_by_id(p_id INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                name       VARCHAR(100),
                address    VARCHAR(100),
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT b.id,
               b.name,
               b.address,
               b.created_at,
               b.updated_at
        FROM branches b
        WHERE b.id = p_id;
END;
$$ LANGUAGE plpgsql;

-- Search branches by name
CREATE FUNCTION fn_branches_view_filters_name(p_name VARCHAR(100), p_limit INTEGER, p_offset INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                name       VARCHAR(100),
                address    VARCHAR(100),
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT b.id,
               b.name,
               b.address,
               b.created_at,
               b.updated_at
        FROM branches b
        WHERE b.name ILIKE '%' || p_name || '%'
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- Search branches by address
CREATE FUNCTION fn_branches_view_filters_address(p_address VARCHAR(100), p_limit INTEGER, p_offset INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                name       VARCHAR(100),
                address    VARCHAR(100),
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT b.id,
               b.name,
               b.address,
               b.created_at,
               b.updated_at
        FROM branches b
        WHERE b.address ILIKE '%' || p_address || '%'
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;


/* Stocks */
-- Trigger function
CREATE FUNCTION logs.fn_stocks_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO logs.stocks (action, stock_id, product_id, branch_id, quantity)
        VALUES ('INSERT', NEW.id, NEW.product_id, NEW.branch_id, NEW.quantity);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO logs.stocks (action, stock_id, product_id, branch_id, quantity)
        VALUES ('UPDATE', NEW.id, NEW.product_id, NEW.branch_id, NEW.quantity);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO logs.stocks (action, stock_id, product_id, branch_id, quantity)
        VALUES ('DELETE', OLD.id, OLD.product_id, OLD.branch_id, OLD.quantity);
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Select stocks by id
CREATE FUNCTION fn_stocks_view_select_by_id(p_id INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                product_id INTEGER,
                branch_id  INTEGER,
                quantity   INTEGER,
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT s.id,
               s.product_id,
               s.branch_id,
               s.quantity,
               s.created_at,
               s.updated_at
        FROM stocks s
        WHERE s.id = p_id;
END;
$$ LANGUAGE plpgsql;

-- Search stocks by product
CREATE FUNCTION fn_stocks_view_filters_product(p_product_id INTEGER, p_limit INTEGER, p_offset INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                product_id INTEGER,
                branch_id  INTEGER,
                quantity   INTEGER,
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT s.id,
               s.product_id,
               s.branch_id,
               s.quantity,
               s.created_at,
               s.updated_at
        FROM stocks s
        WHERE s.product_id = p_product_id
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- Search stocks by branch
CREATE FUNCTION fn_stocks_view_filters_branch(p_branch_id INTEGER, p_limit INTEGER, p_offset INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                product_id INTEGER,
                branch_id  INTEGER,
                quantity   INTEGER,
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT s.id,
               s.product_id,
               s.branch_id,
               s.quantity,
               s.created_at,
               s.updated_at
        FROM stocks s
        WHERE s.branch_id = p_branch_id
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- Search stocks by product and branch
CREATE FUNCTION fn_stocks_view_filters_product_branch(p_product_id INTEGER, p_branch_id INTEGER, p_limit INTEGER,
                                                      p_offset INTEGER)
    RETURNS TABLE
            (
                id         INTEGER,
                product_id INTEGER,
                branch_id  INTEGER,
                quantity   INTEGER,
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT s.id,
               s.product_id,
               s.branch_id,
               s.quantity,
               s.created_at,
               s.updated_at
        FROM stocks s
        WHERE s.product_id = p_product_id
          AND s.branch_id = p_branch_id
        LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;*/


