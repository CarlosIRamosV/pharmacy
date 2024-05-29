\c pharmacy

/* product */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_product_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.product (action, product_id, name, description, price)
        VALUES ('INSERT', NEW.id, NEW.name, NEW.description, NEW.price);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.product (action, product_id, name, description, price)
        VALUES ('UPDATE', NEW.id, NEW.name, NEW.description, NEW.price);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.product (action, product_id, name, description, price)
        VALUES ('DELETE', OLD.id, OLD.name, OLD.description, OLD.price);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* branch */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_branch_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.branch (action, branch_id, name, address)
        VALUES ('INSERT', NEW.id, NEW.name, NEW.address);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.branch (action, branch_id, name, address)
        VALUES ('UPDATE', NEW.id, NEW.name, NEW.address);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.branch (action, branch_id, name, address)
        VALUES ('DELETE', OLD.id, OLD.name, OLD.address);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* product_branch */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_product_branch_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.product_branch (action, product_id, branch_id)
        VALUES ('INSERT', NEW.product_id, NEW.branch_id);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.product_branch (action, product_id, branch_id)
        VALUES ('DELETE', OLD.product_id, OLD.branch_id);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* profile */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_profile_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.profile (action, profile_id, first_name, last_name, email)
        VALUES ('INSERT', NEW.id, NEW.first_name, NEW.last_name, NEW.email);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.profile (action, profile_id, first_name, last_name, email)
        VALUES ('UPDATE', NEW.id, NEW.first_name, NEW.last_name, NEW.email);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.profile (action, profile_id, first_name, last_name, email)
        VALUES ('DELETE', OLD.id, OLD.first_name, OLD.last_name, OLD.email);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* user */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_user_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.user (action, user_id, profile_id, username, password)
        VALUES ('INSERT', NEW.id, NEW.profile_id, NEW.username, NEW.password);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.user (action, user_id, profile_id, username, password)
        VALUES ('UPDATE', NEW.id, NEW.profile_id, NEW.username, NEW.password);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.user (action, user_id, profile_id, username, password)
        VALUES ('DELETE', OLD.id, OLD.profile_id, OLD.username, OLD.password);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* role */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_role_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.role (action, role_id, name)
        VALUES ('INSERT', NEW.id, NEW.name);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.role (action, role_id, name)
        VALUES ('UPDATE', NEW.id, NEW.name);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.role (action, role_id, name)
        VALUES ('DELETE', OLD.id, OLD.name);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* user_role */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_user_role_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.user_role (action, user_id, role_id)
        VALUES ('INSERT', NEW.user_id, NEW.role_id);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.user_role (action, user_id, role_id)
        VALUES ('DELETE', OLD.user_id, OLD.role_id);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* order */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_order_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.order (action, order_id, user_id, branch_id, total)
        VALUES ('INSERT', NEW.id, NEW.user_id, NEW.branch_id, NEW.total);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.order (action, order_id, user_id, branch_id, total)
        VALUES ('UPDATE', NEW.id, NEW.user_id, NEW.branch_id, NEW.total);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.order (action, order_id, user_id, branch_id, total)
        VALUES ('DELETE', OLD.id, OLD.user_id, OLD.branch_id, OLD.total);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Calculate the total of the order
CREATE OR REPLACE FUNCTION fn_order_total_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.total = (SELECT SUM(total) FROM order_product WHERE order_id = NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* order_product */
-- Trigger function
CREATE OR REPLACE FUNCTION log.fn_order_product_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        INSERT INTO log.order_product (action, order_id, product_id, quantity, total)
        VALUES ('INSERT', NEW.order_id, NEW.product_id, NEW.quantity, NEW.total);
    ELSIF (TG_OP = 'UPDATE') THEN
        INSERT INTO log.order_product (action, order_id, product_id, quantity, total)
        VALUES ('UPDATE', NEW.order_id, NEW.product_id, NEW.quantity, NEW.total);
    ELSIF (TG_OP = 'DELETE') THEN
        INSERT INTO log.order_product (action, order_id, product_id, quantity, total)
        VALUES ('DELETE', OLD.order_id, OLD.product_id, OLD.quantity, OLD.total);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Calculate the total of the order product
CREATE OR REPLACE FUNCTION fn_order_product_total_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.total = NEW.quantity * (SELECT price FROM product WHERE id = NEW.product_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Get the total of the order in the branch
CREATE OR REPLACE FUNCTION fn_get_order_total(p_branch_id INT)
RETURNS DECIMAL AS $$
DECLARE
    v_total DECIMAL;
BEGIN
    SELECT SUM(o.total)
    INTO v_total
    FROM "order" o
    WHERE o.branch_id = p_branch_id;

    RETURN COALESCE(v_total, 0);
END;
$$ LANGUAGE plpgsql;

-- Get Total sales of the product in the branch
CREATE OR REPLACE FUNCTION fn_get_product_total_sales(p_product_id INT, p_branch_id INT)
RETURNS DECIMAL AS $$
DECLARE
    v_total DECIMAL;
BEGIN
    SELECT SUM(op.total)
    INTO v_total
    FROM order_product op
             JOIN "order" o ON op.order_id = o.id
    WHERE op.product_id = p_product_id
      AND o.branch_id = p_branch_id;

    RETURN COALESCE(v_total, 0);
END;
$$ LANGUAGE plpgsql;
