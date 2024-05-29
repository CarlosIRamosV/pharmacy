\c pharmacy

/* Products */
CREATE TRIGGER products_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.product
    FOR EACH ROW
EXECUTE FUNCTION log.fn_product_trigger();

/* Branches */
CREATE TRIGGER branches_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.branch
    FOR EACH ROW
EXECUTE FUNCTION log.fn_branch_trigger();

/* Product Branches */
CREATE TRIGGER product_branches_trigger
    AFTER INSERT OR DELETE
    ON public.product_branch
    FOR EACH ROW
EXECUTE FUNCTION log.fn_product_branch_trigger();

/* Profiles */
CREATE TRIGGER profiles_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.profile
    FOR EACH ROW
EXECUTE FUNCTION log.fn_profile_trigger();

/* Users */
CREATE TRIGGER users_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public."user"
    FOR EACH ROW
EXECUTE FUNCTION log.fn_user_trigger();

/* Roles */
CREATE TRIGGER roles_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.role
    FOR EACH ROW
EXECUTE FUNCTION log.fn_role_trigger();

/* User Roles */
CREATE TRIGGER user_roles_trigger
    AFTER INSERT OR DELETE
    ON public.user_role
    FOR EACH ROW
EXECUTE FUNCTION log.fn_user_role_trigger();

/* Orders */
CREATE TRIGGER order_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.order
    FOR EACH ROW
EXECUTE FUNCTION log.fn_order_trigger();

CREATE TRIGGER order_total_trigger
    BEFORE UPDATE
    ON public.order
    FOR EACH ROW
EXECUTE FUNCTION fn_order_total_trigger();

/* Order Products */
CREATE TRIGGER order_product_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.order_product
    FOR EACH ROW
EXECUTE FUNCTION log.fn_order_product_trigger();

/* Order Products */
CREATE TRIGGER order_product_total_trigger
    BEFORE INSERT OR UPDATE
    ON public.order_product
    FOR EACH ROW
EXECUTE FUNCTION fn_order_product_total_trigger();