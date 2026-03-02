-- Rollback seed data

-- Delete transaction details first (depends on transactions)
DELETE FROM transactions_detail;

-- Delete transactions (depends on baskets)
DELETE FROM transactions;

-- Delete baskets (depends on users and basket_category)
DELETE FROM baskets;

-- Delete users
DELETE FROM users;

-- Delete transaction types (children first, then parents)
DELETE FROM transactions_type WHERE parent_id IS NOT NULL;
DELETE FROM transactions_type WHERE parent_id IS NULL;

-- Delete basket categories
DELETE FROM basket_category;

-- Reset all sequences
SELECT setval('transactions_detail_id_seq', 1, false);
SELECT setval('transactions_id_seq', 1, false);
SELECT setval('baskets_id_seq', 1, false);
SELECT setval('users_id_seq', 1, false);
SELECT setval('transactions_type_id_seq', 1, false);
SELECT setval('basket_category_id_seq', 1, false);
