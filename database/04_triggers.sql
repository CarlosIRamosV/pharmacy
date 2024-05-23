\c pharmacy

/*
CREATE TRIGGER products_trigger
    AFTER UPDATE OR DELETE
    ON public.products
    FOR EACH ROW
EXECUTE FUNCTION logs.fn_products_trigger();

CREATE TRIGGER branches_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.branches
    FOR EACH ROW
EXECUTE FUNCTION logs.fn_branches_trigger();

 */

CREATE TRIGGER branches_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.branches
    FOR EACH ROW
EXECUTE FUNCTION logs.fn_branches_trigger();

CREATE TRIGGER stocks_trigger
    AFTER INSERT OR UPDATE OR DELETE
    ON public.stocks
    FOR EACH ROW
EXECUTE FUNCTION logs.fn_stocks_trigger();


