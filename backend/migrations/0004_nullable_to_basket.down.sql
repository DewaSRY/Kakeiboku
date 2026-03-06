-- Revert: make to_basket_id NOT NULL again
-- First delete any rows where to_basket_id IS NULL (spend transactions)
DELETE FROM transactions WHERE to_basket_id IS NULL;
ALTER TABLE transactions ALTER COLUMN to_basket_id SET NOT NULL;
