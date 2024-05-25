\c pharmacy

/* Products */
CREATE TRIGGER products_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.products
    FOR EACH ROW
EXECUTE FUNCTION logs.fn_products_trigger();

/* Branches */
CREATE TRIGGER branches_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.branches
    FOR EACH ROW
EXECUTE FUNCTION logs.fn_branches_trigger();