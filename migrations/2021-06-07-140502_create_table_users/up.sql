CREATE TABLE users (
    uuid BINARY(16) PRIMARY KEY,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    account_status ENUM('CREATED', 'CONFIRMED', 'DISABLED') NOT NULL,
    timezone VARCHAR(40), -- see max length (40): https://stackoverflow.com/a/12546342/5155484
    first_log_in_at TIMESTAMP(6) NULL,
    last_log_in_at TIMESTAMP(6) NULL,
    language VARCHAR(10) NULL
);

ALTER TABLE `users` ADD UNIQUE `users_idx_unique_username` (`username`);
