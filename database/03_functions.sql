\c pharmacy

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
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create product and return the product
CREATE FUNCTION fn_create_product(p_name VARCHAR(100), p_description TEXT, p_price DECIMAL(10, 2))
    RETURNS TABLE
            (
                id          INTEGER,
                name        VARCHAR(100),
                description TEXT,
                price       DECIMAL(10, 2),
                created_at  TIMESTAMP,
                updated_at  TIMESTAMP
            )
AS
$$
DECLARE
    v_product products%ROWTYPE;
BEGIN
    INSERT INTO products (name, description, price)
    VALUES (p_name, p_description, p_price)
    RETURNING * INTO v_product;

    RETURN QUERY
        SELECT p.id,
               p.name,
               p.description,
               p.price,
               p.created_at,
               p.updated_at
        FROM products p
        WHERE p.id = v_product.id;
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
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create branch and return the branch
CREATE FUNCTION fn_create_branch(p_name VARCHAR(100), p_address VARCHAR(100))
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
DECLARE
    v_branch branches%ROWTYPE;
BEGIN
    INSERT INTO branches (name, address)
    VALUES (p_name, p_address)
    RETURNING * INTO v_branch;

    RETURN QUERY
        SELECT b.id,
               b.name,
               b.address,
               b.created_at,
               b.updated_at
        FROM branches b
        WHERE b.id = v_branch.id;
END;
$$ LANGUAGE plpgsql;

/* Images */

CREATE FUNCTION fn_create_image(p_image BYTEA, p_hash VARCHAR(100))
    RETURNS TABLE
            (
                id         SERIAL,
                image      BYTEA,
                hash       VARCHAR(100),
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
AS
$$
DECLARE
    v_image images%ROWTYPE;
BEGIN
    INSERT INTO images (image, hash)
    VALUES (p_image, p_hash)
    RETURNING * INTO v_image;

    RETURN QUERY
        SELECT i.id,
               i.image,
               i.hash,
               i.created_at,
               i.updated_at
        FROM images i
        WHERE i.id = v_image.id;
END;
$$ LANGUAGE plpgsql;