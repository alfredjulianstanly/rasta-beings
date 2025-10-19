#!/bin/bash

echo "🔧 Fixing all PostgreSQL integration issues..."

# Update Cargo.toml with correct shuttle version
cat > Cargo.toml << 'EOF'
[package]
name = "rasta-beings"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs"] }
serde = { version = "1.0", features = ["derive"] }
shuttle-runtime = "0.50.0"
shuttle-axum = "0.50.0"
shuttle-shared-db = { version = "0.50.0", features = ["postgres", "sqlx"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
axum-extra = { version = "0.9", features = ["multipart"] }
base64 = "0.22"
EOF

echo "✅ Updated Cargo.toml"
echo "⏳ This may take a while - compiling..."

cargo check 2>&1 | head -20

echo ""
echo "📊 Status check complete"
