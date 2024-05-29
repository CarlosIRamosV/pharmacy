\c pharmacy

-- Add a product
CREATE OR REPLACE PROCEDURE add_product(
    IN p_name VARCHAR(255),
    IN p_description TEXT,
    IN p_price DECIMAL(10, 2)
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO product (name, description, price)
    VALUES (p_name, p_description, p_price);
END;
$$;

-- Add a branch
CREATE OR REPLACE PROCEDURE add_branch(
    IN p_name VARCHAR(255),
    IN p_address VARCHAR(255)
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO branch (name, address)
    VALUES (p_name, p_address);
END;
$$;

-- Add a product to a branch
CREATE OR REPLACE PROCEDURE add_product_to_branch(
    IN p_product_id INT,
    IN p_branch_id INT
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO product_branch (product_id, branch_id)
    VALUES (p_product_id, p_branch_id);
END;
$$;

-- Add a user
CREATE OR REPLACE PROCEDURE add_user(
    IN p_username VARCHAR(255),
    IN p_password VARCHAR(255),
    IN p_profile_id INT
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO "user" (username, password, profile_id)
    VALUES (p_username, p_password, p_profile_id);
END;
$$;

-- Add a role
CREATE OR REPLACE PROCEDURE add_role(
    IN p_name VARCHAR(255)
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO role (name)
    VALUES (p_name);
END;
$$;

-- Add a role to a user
CREATE OR REPLACE PROCEDURE add_role_to_user(
    IN p_user_id INT,
    IN p_role_id INT
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO user_role (user_id, role_id)
    VALUES (p_user_id, p_role_id);
END;
$$;

-- Add a profile
CREATE OR REPLACE PROCEDURE add_profile(
    IN p_first_name VARCHAR(255),
    IN p_last_name VARCHAR(255),
    IN p_email VARCHAR(255)
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO profile (first_name, last_name, email)
    VALUES (p_first_name, p_last_name, p_email);
END;
$$;

-- Add an order
CREATE OR REPLACE PROCEDURE add_order(
    IN p_user_id INT,
    IN p_branch_id INT
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO "order" (user_id, branch_id)
    VALUES (p_user_id, p_branch_id);
END;
$$;

-- Add a product on th order
CREATE OR REPLACE PROCEDURE add_product_to_order(
    IN p_order_id INT,
    IN p_product_id INT,
    IN p_quantity INT
) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO order_product (order_id, product_id, quantity, total)
    VALUES (p_order_id, p_product_id, p_quantity, DEFAULT);

    UPDATE "order" SET total = DEFAULT WHERE id = p_order_id;
END;
$$;

-- Remove a product from the order
CREATE OR REPLACE PROCEDURE remove_product_from_order(
    IN p_order_id INT,
    IN p_product_id INT
) LANGUAGE plpgsql AS $$
BEGIN
    DELETE FROM order_product
    WHERE order_id = p_order_id AND product_id = p_product_id;

    UPDATE "order" SET total = DEFAULT WHERE id = p_order_id;
END;
$$;

