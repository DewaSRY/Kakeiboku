-- Seed default basket categories
INSERT INTO basket_category (name, description) VALUES
    ('Default', 'Default basket category'),
    ('Savings', 'General savings basket'),
    ('Emergency Fund', 'Emergency fund for unexpected expenses'),
    ('Investment', 'Investment allocation basket'),
    ('Daily Expenses', 'Daily spending basket'),
    ('Bills', 'Monthly bills and utilities'),
    ('Entertainment', 'Entertainment and leisure spending'),
    ('Education', 'Education and learning expenses'),
    ('Travel', 'Travel and vacation fund'),
    ('Health', 'Health and medical expenses');

-- Seed default transaction types (parent categories)
INSERT INTO transactions_type (id, parent_id, name, description) VALUES
    (1, NULL, 'Deposit', 'External deposit to main basket'),
    (2, NULL, 'Transfer', 'Internal transfer between baskets'),
    (3, NULL, 'Income', 'Income transactions'),
    (4, NULL, 'Expense', 'Expense transactions'),
    (5, NULL, 'Savings', 'Savings related transactions')
ON CONFLICT DO NOTHING;

-- Seed transaction type children (Income)
INSERT INTO transactions_type (parent_id, name, description) VALUES
    (3, 'Salary', 'Monthly salary income'),
    (3, 'Bonus', 'Work bonus'),
    (3, 'Freelance', 'Freelance income'),
    (3, 'Investment Return', 'Returns from investments'),
    (3, 'Gift', 'Money received as gift'),
    (3, 'Other Income', 'Other income sources')
ON CONFLICT DO NOTHING;

-- Seed transaction type children (Expense)
INSERT INTO transactions_type (parent_id, name, description) VALUES
    (4, 'Food', 'Food and groceries'),
    (4, 'Transportation', 'Transportation costs'),
    (4, 'Utilities', 'Electricity, water, internet bills'),
    (4, 'Rent', 'House or apartment rent'),
    (4, 'Education', 'Education and courses'),
    (4, 'Healthcare', 'Medical and health expenses'),
    (4, 'Entertainment', 'Movies, games, hobbies'),
    (4, 'Shopping', 'General shopping'),
    (4, 'Loan Payment', 'Loan or debt payments'),
    (4, 'Insurance', 'Insurance premiums'),
    (4, 'Subscription', 'Monthly subscriptions'),
    (4, 'Other Expense', 'Other expenses')
ON CONFLICT DO NOTHING;

-- Seed transaction type children (Savings)
INSERT INTO transactions_type (parent_id, name, description) VALUES
    (5, 'Emergency Fund', 'Emergency fund allocation'),
    (5, 'Retirement', 'Retirement savings'),
    (5, 'Investment', 'Investment allocation'),
    (5, 'Vacation Fund', 'Vacation savings'),
    (5, 'Goal Savings', 'Specific goal savings')
ON CONFLICT DO NOTHING;

-- Reset sequence to avoid conflicts with future inserts
SELECT setval('transactions_type_id_seq', (SELECT MAX(id) FROM transactions_type));

-- ============================================
-- SEED USERS
-- Password for all users: "password123"
-- Argon2id hash generated for "password123"
-- ============================================
INSERT INTO users (id, first_name, last_name, email, password, role) VALUES
    (1, 'Admin', 'User', 'admin@kakeiboku.com', '$argon2id$v=19$m=19456,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$0Z8+ePVRFobM7VKK3sUqQqY1y9LjP4d8nQmZ6G5NjAE', 'admin'),
    (2, 'John', 'Doe', 'john@example.com', '$argon2id$v=19$m=19456,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$0Z8+ePVRFobM7VKK3sUqQqY1y9LjP4d8nQmZ6G5NjAE', 'user'),
    (3, 'Jane', 'Smith', 'jane@example.com', '$argon2id$v=19$m=19456,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$0Z8+ePVRFobM7VKK3sUqQqY1y9LjP4d8nQmZ6G5NjAE', 'user'),
    (4, 'Demo', 'Account', 'demo@kakeiboku.com', '$argon2id$v=19$m=19456,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$0Z8+ePVRFobM7VKK3sUqQqY1y9LjP4d8nQmZ6G5NjAE', 'user')
ON CONFLICT (email) DO NOTHING;

SELECT setval('users_id_seq', (SELECT MAX(id) FROM users));

-- ============================================
-- SEED BASKETS
-- Each user gets a main basket + branch baskets
-- ============================================

-- User 1 (Admin) baskets
INSERT INTO baskets (id, user_id, name, description, basket_category_id, created_by_id, type, status) VALUES
    (1, 1, 'Main Wallet', 'Primary wallet for all transactions', 1, 1, 'main', 'active'),
    (2, 1, 'Emergency Fund', 'Emergency savings', 3, 1, 'branch', 'active'),
    (3, 1, 'Investment Portfolio', 'Long-term investments', 4, 1, 'branch', 'active')
ON CONFLICT (user_id, name, type) DO NOTHING;

-- User 2 (John) baskets
INSERT INTO baskets (id, user_id, name, description, basket_category_id, created_by_id, type, status) VALUES
    (4, 2, 'Main Wallet', 'Johns primary wallet', 1, 2, 'main', 'active'),
    (5, 2, 'Savings Account', 'Monthly savings', 2, 2, 'branch', 'active'),
    (6, 2, 'Travel Fund', 'Vacation savings', 9, 2, 'branch', 'active'),
    (7, 2, 'Entertainment', 'Fun money', 7, 2, 'branch', 'active')
ON CONFLICT (user_id, name, type) DO NOTHING;

-- User 3 (Jane) baskets
INSERT INTO baskets (id, user_id, name, description, basket_category_id, created_by_id, type, status) VALUES
    (8, 3, 'Main Wallet', 'Janes primary wallet', 1, 3, 'main', 'active'),
    (9, 3, 'Health Fund', 'Medical expenses', 10, 3, 'branch', 'active'),
    (10, 3, 'Education', 'Learning budget', 8, 3, 'branch', 'active')
ON CONFLICT (user_id, name, type) DO NOTHING;

-- User 4 (Demo) baskets
INSERT INTO baskets (id, user_id, name, description, basket_category_id, created_by_id, type, status) VALUES
    (11, 4, 'Main Wallet', 'Demo main wallet', 1, 4, 'main', 'active'),
    (12, 4, 'Savings', 'Demo savings basket', 2, 4, 'branch', 'active'),
    (13, 4, 'Bills', 'Monthly bills allocation', 6, 4, 'branch', 'active'),
    (14, 4, 'Daily Expenses', 'Daily spending money', 5, 4, 'branch', 'active')
ON CONFLICT (user_id, name, type) DO NOTHING;

SELECT setval('baskets_id_seq', (SELECT MAX(id) FROM baskets));

-- ============================================
-- SEED TRANSACTIONS
-- Various transaction types for demo data
-- ============================================

-- Get transaction type IDs for reference:
-- 1 = Deposit, 2 = Transfer, 3 = Income, 4 = Expense, 5 = Savings
-- Income children start from ~6, Expense from ~12, Savings from ~24

-- Demo User (id=4) transactions - comprehensive demo data

-- Initial deposit to main wallet (external deposit, from_basket_id is NULL)
INSERT INTO transactions (id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at) VALUES
    (1, 4, NULL, 11, 5000.00, 1, NOW() - INTERVAL '30 days'),
    (2, 4, NULL, 11, 3000.00, 1, NOW() - INTERVAL '15 days'),
    (3, 4, NULL, 11, 2500.00, 1, NOW() - INTERVAL '1 day')
ON CONFLICT DO NOTHING;

-- Transfer from main to branch baskets
INSERT INTO transactions (id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at) VALUES
    (4, 4, 11, 12, 2000.00, 2, NOW() - INTERVAL '29 days'),
    (5, 4, 11, 13, 1500.00, 2, NOW() - INTERVAL '28 days'),
    (6, 4, 11, 14, 1000.00, 2, NOW() - INTERVAL '27 days'),
    (7, 4, 11, 12, 500.00, 2, NOW() - INTERVAL '14 days'),
    (8, 4, 11, 14, 500.00, 2, NOW() - INTERVAL '7 days')
ON CONFLICT DO NOTHING;

-- John (id=2) transactions
INSERT INTO transactions (id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at) VALUES
    (9, 2, NULL, 4, 4000.00, 1, NOW() - INTERVAL '25 days'),
    (10, 2, 4, 5, 1500.00, 2, NOW() - INTERVAL '24 days'),
    (11, 2, 4, 6, 800.00, 2, NOW() - INTERVAL '20 days'),
    (12, 2, 4, 7, 300.00, 2, NOW() - INTERVAL '15 days'),
    (13, 2, NULL, 4, 2000.00, 1, NOW() - INTERVAL '5 days')
ON CONFLICT DO NOTHING;

-- Jane (id=3) transactions
INSERT INTO transactions (id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at) VALUES
    (14, 3, NULL, 8, 3500.00, 1, NOW() - INTERVAL '20 days'),
    (15, 3, 8, 9, 500.00, 2, NOW() - INTERVAL '18 days'),
    (16, 3, 8, 10, 700.00, 2, NOW() - INTERVAL '12 days'),
    (17, 3, NULL, 8, 1800.00, 1, NOW() - INTERVAL '3 days')
ON CONFLICT DO NOTHING;

-- Admin (id=1) transactions
INSERT INTO transactions (id, created_by_id, from_basket_id, to_basket_id, amount, transaction_type_id, created_at) VALUES
    (18, 1, NULL, 1, 10000.00, 1, NOW() - INTERVAL '60 days'),
    (19, 1, 1, 2, 3000.00, 2, NOW() - INTERVAL '55 days'),
    (20, 1, 1, 3, 5000.00, 2, NOW() - INTERVAL '50 days')
ON CONFLICT DO NOTHING;

SELECT setval('transactions_id_seq', (SELECT MAX(id) FROM transactions));

-- ============================================
-- SEED TRANSACTION DETAILS
-- Add details to some transactions
-- ============================================

INSERT INTO transactions_detail (id, transaction_id, title, description) VALUES
    (1, 1, 'Initial Deposit', 'First deposit to start tracking finances'),
    (2, 2, 'Salary Deposit', 'Monthly salary received'),
    (3, 3, 'Bonus Payment', 'Quarterly performance bonus'),
    (4, 4, 'Savings Allocation', 'Monthly savings transfer'),
    (5, 5, 'Bills Budget', 'Allocated for monthly bills'),
    (6, 6, 'Weekly Budget', 'Weekly spending allowance'),
    (7, 9, 'Salary', 'Johns monthly salary'),
    (8, 10, 'Savings Transfer', 'Monthly savings goal'),
    (9, 11, 'Vacation Fund', 'Saving for summer vacation'),
    (10, 14, 'Salary Deposit', 'Janes monthly income'),
    (11, 15, 'Health Reserve', 'Monthly health fund allocation'),
    (12, 16, 'Course Budget', 'Online learning subscription'),
    (13, 18, 'Startup Capital', 'Initial account funding'),
    (14, 19, 'Emergency Reserve', 'Emergency fund setup'),
    (15, 20, 'Investment Allocation', 'Initial investment deposit')
ON CONFLICT DO NOTHING;

SELECT setval('transactions_detail_id_seq', (SELECT MAX(id) FROM transactions_detail));
