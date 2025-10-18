-- Products table with extended properties
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    icon TEXT NOT NULL,
    
    -- Extended properties
    sku VARCHAR(100) UNIQUE,
    category VARCHAR(100),
    tags TEXT[],
    
    -- Variants
    sizes TEXT[],
    colors TEXT[],
    
    -- Pricing details
    mrp DECIMAL(10, 2),
    discount_percent DECIMAL(5, 2) DEFAULT 0,
    cost_price DECIMAL(10, 2),
    profit_margin DECIMAL(5, 2),
    tax_percent DECIMAL(5, 2) DEFAULT 0,
    
    -- Inventory
    stock_quantity INTEGER DEFAULT 0,
    
    -- Supplier info
    supplier_name VARCHAR(255),
    supplier_contact VARCHAR(255),
    purchase_date DATE,
    invoice_number VARCHAR(100),
    
    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Carts table
CREATE TABLE IF NOT EXISTS carts (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Cart items table
CREATE TABLE IF NOT EXISTS cart_items (
    id SERIAL PRIMARY KEY,
    cart_id INTEGER NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(cart_id, product_id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_carts_session_id ON carts(session_id);
CREATE INDEX IF NOT EXISTS idx_cart_items_cart_id ON cart_items(cart_id);
CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);
