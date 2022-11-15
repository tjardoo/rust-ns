CREATE TABLE products
(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    product_number VARCHAR(50) NOT NULL,
    category_code VARCHAR(50) NOT NULL,
    short_category_name VARCHAR(50) NOT NULL,
    long_category_name VARCHAR(50) NOT NULL,
    operator_code VARCHAR(50) NOT NULL,
    operator_name VARCHAR(50) NOT NULL,
    product_type VARCHAR(50) NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);