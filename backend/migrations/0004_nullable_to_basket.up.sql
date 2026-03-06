-- Make to_basket_id nullable to support spend transactions
-- (spend = money leaves a branch basket to external, so to_basket_id = NULL)
ALTER TABLE transactions ALTER COLUMN to_basket_id DROP NOT NULL;
