
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('admin', 'user')) DEFAULT 'user',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE baskets(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    basket_category_id BIGINT NOT NULL, 
    type varchar(20) not null check (type in ('main', 'branch')),
    status varchar(20) not null check (status in('active', 'frozen', 'archived')),
    created_by_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE basket_category (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    created_by_id BIGINT NOT NULL,
    from_basket_id BIGINT,
    to_basket_id BIGINT NOT NULL,
    amount DECIMAL(15, 2) NOT NULL CHECK (amount >= 0),
    transaction_type_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE transactions_type (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  
);

CREATE TABLE transactions_detail (
    id BIGSERIAL PRIMARY KEY,
    transaction_id BIGINT NOT NULL,
    title VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE baskets
    ADD CONSTRAINT fk_basket_basket_category FOREIGN KEY (basket_category_id) REFERENCES basket_category(id),
    ADD CONSTRAINT fk_basket_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    ADD CONSTRAINT unique_basket_per_user UNIQUE (user_id, name, type);

ALTER TABLE transactions
    ADD CONSTRAINT fk_basket_transaction_basket FOREIGN KEY (from_basket_id) REFERENCES baskets(id) ON DELETE CASCADE,
    ADD CONSTRAINT fk_basket_transaction_transaction FOREIGN KEY (to_basket_id) REFERENCES baskets(id) ON DELETE CASCADE;

ALTER TABLE transactions_detail
    ADD CONSTRAINT fk_transaction_detail_transaction FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE;
    

CREATE INDEX idx_users_email ON users(email);

CREATE INDEX idx_basket_user_id ON baskets(user_id);
CREATE INDEX idx_basket_created_by ON baskets(created_by_id);
CREATE INDEX idx_basket_category_id ON baskets(basket_category_id);
CREATE INDEX idx_basket_status ON baskets(status);

CREATE INDEX idx_transactions_basket_history ON transactions(from_basket_id, created_at DESC);
CREATE INDEX idx_transactions_to_basket_history ON transactions(to_basket_id, created_at DESC);

CREATE UNIQUE INDEX idx_unique_main_basket_per_user ON baskets(user_id) WHERE type = 'main';


CREATE INDEX idx_transactions_from_basket ON transactions(from_basket_id);
CREATE INDEX idx_transactions_to_basket ON transactions(to_basket_id);
CREATE INDEX idx_transactions_type ON transactions(transaction_type_id);