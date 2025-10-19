# 🌿 Rasta Beings - E-commerce Platform

A modern, full-stack e-commerce platform built with Rust, featuring cloud storage and real-time inventory management.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Live Demo](https://img.shields.io/badge/demo-live-green.svg)](https://rasta-beings-qoxw.shuttle.app)

🔗 **Live Demo:** [https://rasta-beings-qoxw.shuttle.app](https://rasta-beings-qoxw.shuttle.app)

---

## ✨ Features

### Current (Phase 2.2)
- 🛍️ **Product Catalog** - Browse products with images served from global CDN
- 🛒 **Shopping Cart** - Persistent cart with quantity management
- 💰 **Checkout System** - Complete order processing
- 🔧 **Admin Panel** - Add/delete products with image upload
- ☁️ **Cloud Storage** - Cloudflare R2 integration for images
- 💾 **Database** - PostgreSQL with migrations
- 🌍 **Global CDN** - Fast image delivery worldwide

### Upcoming (Planned)
- 🔐 Admin authentication
- 📦 Extended product fields (SKU, categories, stock)
- 👤 User accounts
- 📊 Order history
- 💳 Payment processing

---

## 🛠️ Tech Stack

### Backend
- **Rust** - Systems programming language
- **Axum** - Web framework
- **SQLx** - SQL toolkit with compile-time verification
- **PostgreSQL** - Relational database

### Storage & Deployment
- **Cloudflare R2** - S3-compatible object storage (99% cheaper than AWS!)
- **Shuttle.rs** - Rust-native deployment platform
- **AWS SDK** - S3-compatible client for R2

### Frontend
- Server-side HTML rendering
- Responsive design with inline CSS
- Rasta-themed UI (Red, Gold, Green)

---

## 📊 Architecture
┌─────────────────────────────────────────────┐
│  User Browser                               │
│  (rasta-beings.shuttle.app)                 │
└────────┬────────────────────────────────────┘
│
↓
┌─────────────────────────────────────────────┐
│  Shuttle Server (Rust/Axum)                 │
│  - Request handling                         │
│  - Business logic                           │
│  - HTML generation                          │
└────────┬──────────────┬─────────────────────┘
│              │
↓              ↓
┌────────────────┐  ┌──────────────────────┐
│  PostgreSQL    │  │  Cloudflare R2       │
│  (Shuttle)     │  │  (Cloud Storage)     │
│                │  │                      │
│  • Products    │  │  • Product images    │
│  • Cart        │  │  • CDN delivery      │
│  • Orders      │  │  • Free egress!      │
└────────────────┘  └──────────────────────┘
---

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- PostgreSQL (or use Shuttle's managed DB)
- Cloudflare account (for R2)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/alfredjulianstanley/rasta-beings.git
   cd rasta-beings
Set up Cloudflare R2
Create R2 bucket: rasta-beings-images
Generate API tokens with Read & Write permissions
Enable public access to get public URL
Configure secrets
Create Secrets.toml:
R2_ACCESS_KEY_ID = "your_access_key"
R2_SECRET_ACCESS_KEY = "your_secret_key"
R2_ENDPOINT = "https://xxxxx.r2.cloudflarestorage.com"
R2_BUCKET_NAME = "rasta-beings-images"
R2_PUBLIC_URL = "https://pub-xxxxx.r2.dev"
Important: Add to .gitignore:
echo "Secrets.toml" >> .gitignore
Install Shuttle CLI
cargo install cargo-shuttle
Deploy
cargo shuttle deploy
📁 Project Structure
rasta-beings/
├── src/
│   ├── main.rs              # App entry point & routing
│   ├── r2.rs                # Cloudflare R2 client
│   ├── handlers/
│   │   ├── mod.rs           # Handler exports
│   │   ├── shop.rs          # Product catalog
│   │   ├── cart.rs          # Shopping cart
│   │   └── admin.rs         # Admin panel
│   ├── models/
│   │   ├── mod.rs           # Model exports
│   │   ├── product.rs       # Product model
│   │   └── cart.rs          # Cart models
│   ├── state/
│   │   └── mod.rs           # App state (DB pool)
│   └── templates/
│       └── mod.rs           # HTML templates
├── migrations/
│   ├── 0001_initial_schema.sql
│   ├── 0002_fix_price_types.sql
│   └── 0003_reset_schema.sql
├── Cargo.toml               # Dependencies
├── Secrets.toml            # R2 credentials (gitignored!)
└── README.md
💾 Database Schema
Products Table
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    icon TEXT NOT NULL,  -- R2 URL
    
    -- Extended fields (for future use)
    sku VARCHAR(100) UNIQUE,
    category VARCHAR(100),
    stock_quantity INTEGER DEFAULT 0,
    -- ... more fields
);
Carts & Cart Items
CREATE TABLE carts (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cart_items (
    id SERIAL PRIMARY KEY,
    cart_id INTEGER REFERENCES carts(id),
    product_id INTEGER REFERENCES products(id),
    quantity INTEGER DEFAULT 1,
    UNIQUE(cart_id, product_id)
);
📈 Performance & Costs
Before R2 (Base64 in Database)
Database size: 16MB for 8 products
Loading speed: Slow (embedded in HTML)
Scalability: Limited (~500 products max)
Cost: Would need paid Shuttle tier
After R2 (Cloud Storage)
Database size: 400 bytes for 8 products (99.9% reduction!)
Loading speed: Fast (Cloudflare CDN)
Scalability: Unlimited products
Cost: $0/month (R2 free tier: 10GB storage, unlimited downloads!)
🔧 Development
Local Testing
Note: Local testing requires Docker (for PostgreSQL). On Termux/mobile, deploy directly to Shuttle.
# Build only (no Docker needed)
cargo check

# Deploy to Shuttle (compiles on their servers)
cargo shuttle deploy
Database Migrations
# Migrations run automatically on deployment
# Located in: migrations/*.sql
Environment Variables
All secrets are loaded from Secrets.toml:
R2_ACCESS_KEY_ID - R2 access key
R2_SECRET_ACCESS_KEY - R2 secret key
R2_ENDPOINT - R2 API endpoint
R2_BUCKET_NAME - Bucket name
R2_PUBLIC_URL - Public CDN URL
📖 API Routes
Public Routes
GET / - Product catalog (shop)
GET /cart - View shopping cart
POST /cart/add - Add item to cart
POST /cart/update - Update quantity
POST /checkout - Complete order
Admin Routes
GET /admin - Admin panel
POST /admin/products - Add product (with image upload)
POST /admin/products/delete - Delete product
🎨 Design Philosophy
Rasta Theme
Colors: Red, Gold, Green (Rastafarian symbolism)
Message: "One Love, One Heart, One Destiny"
Vibe: Peace, Unity, Respect for Nature
UI/UX
Server-side rendering (no JavaScript frameworks)
Responsive design
Smooth transitions and hover effects
Accessible color contrasts
🌟 Key Achievements
Built Entirely on Mobile! 📱
This project was developed using:
Termux on Android
No desktop required!
Full Rust development workflow on phone
Production-Grade Features
✅ Database migrations
✅ Cloud storage integration
✅ CDN delivery
✅ Proper error handling
✅ Session management
✅ Image optimization
Modern Stack
Latest Rust async/await
Type-safe SQL queries
S3-compatible cloud storage
Zero-downtime deployments
📝 Roadmap
Phase 2.3 - Authentication (Next)
[ ] Admin login with password
[ ] Session-based auth
[ ] Protected admin routes
Phase 2.4 - Extended Features
[ ] SKU and category fields
[ ] Stock management
[ ] Multiple images per product
[ ] MRP and discount pricing
Phase 3 - User Features
[ ] User registration/login
[ ] Order history
[ ] Wishlist
[ ] Email notifications
Phase 4 - Payments
[ ] Razorpay integration
[ ] Order tracking
[ ] Invoice generation
🤝 Contributing
Contributions are welcome! This is a learning project, so feel free to:
Report bugs
Suggest features
Submit pull requests
Ask questions
📄 License
This project is licensed under the MIT License - see the LICENSE file for details.
🙏 Acknowledgments
Shuttle.rs - Amazing Rust deployment platform
Cloudflare - Free R2 storage with unlimited downloads
Rust Community - Excellent documentation and support
Built with ❤️ on Termux 📱
📞 Contact
GitHub: @alfredjulianstanly
Project Link: https://github.com/alfredjulianstanly/rasta-beings
Live Demo: https://rasta-beings-qoxw.shuttle.app
�

🌿 One Love • One Heart • One Destiny 🌿
Made with 🦀 Rust | Deployed on 📱 Mobile | Powered by ☁️ Cloud
�
