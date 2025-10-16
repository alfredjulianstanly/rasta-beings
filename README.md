# 🌿 Rasta Beings - E-commerce Platform

<div align="center">

**One Love · One Heart · One Destiny**

A Rastafarian-inspired e-commerce platform built with Rust, embodying peace, unity, and respect for nature.

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Powered by Shuttle](https://img.shields.io/badge/Powered%20by-Shuttle-blue.svg)](https://www.shuttle.rs/)
[![Live Demo](https://img.shields.io/badge/Live-Demo-green.svg)](https://rasta-beings-qoxw.shuttle.app)

</div>

---

## ✨ Features

### 🛍️ Customer Experience
- **Product Catalog** - Browse our spiritually-inspired collection
- **Shopping Cart** - Add items with quantity controls (+ and -)
- **Seamless Checkout** - Complete orders with cultural blessings
- **Responsive Design** - Beautiful on mobile and desktop

### 🎨 Rasta Theme
- **Cultural Identity** - Red-Gold-Green Rastafarian color scheme
- **Symbolic Elements** - Peace signs ☮️, unity fist ✊🏿, lion of Judah 🦁
- **Natural Patterns** - Floating leaf decorations 🌿
- **Inspirational Quotes** - Bob Marley wisdom throughout
- **Celtic Triquetra Logo** - Sacred geometry at the heart

### 🔧 Admin Panel
- **Product Management** - Create, view, and delete products
- **Image Uploads** - Upload product images directly from device
- **Real-time Feedback** - Success notifications and loading states
- **Hidden Access** - Click the logo to access admin features

---

## 🦀 Tech Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[Shuttle.rs](https://www.shuttle.rs/)** - Deployment platform
- **[Tokio](https://tokio.rs/)** - Async runtime
- **Server-Side Rendering** - No JavaScript framework needed
- **In-Memory Storage** - HashMap for Phase 1 POC

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)
- Shuttle CLI

### Installation

```bash
# Clone the repository
git clone https://github.com/alfredjulianstanley/rasta-beings.git
cd rasta-beings

# Install Shuttle CLI
cargo install cargo-shuttle

# Run locally
cargo shuttle run
```

Visit `http://localhost:8000` to see the app! 🌿

### With Hot Reload (Recommended for Development)

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-restart on file changes
cargo watch -x 'shuttle run'
```

---

## 📂 Project Structure

```
rasta-beings/
├── src/
│   ├── assets/
│   │   └── IMG_2513.jpg      # Celtic triquetra logo
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── shop.rs            # Product catalog
│   │   ├── cart.rs            # Shopping cart & checkout
│   │   └── admin.rs           # Product management
│   ├── models/
│   │   ├── mod.rs
│   │   ├── product.rs         # Product model
│   │   └── cart.rs            # Cart & CartItem models
│   ├── state/
│   │   └── mod.rs             # App state (in-memory storage)
│   ├── templates/
│   │   └── mod.rs             # HTML templates
│   └── main.rs                # Entry point
├── Cargo.toml
├── README.md
├── TODO.md                    # Development roadmap
└── CHECKLIST.md               # Phase 1 completion status
```

---

## 🎯 Current Status: Phase 1 MVP ✅

**What's Complete:**
- ✅ Full e-commerce functionality (browse, cart, checkout)
- ✅ Admin panel with product management
- ✅ Image uploads (base64 encoding)
- ✅ Rasta-themed design with cultural elements
- ✅ Responsive layout for mobile and desktop
- ✅ Deployed to production

**See [CHECKLIST.md](CHECKLIST.md) for detailed completion status**

---

## 🔮 Roadmap

### Phase 2: Database & Storage (Next)
- [ ] PostgreSQL integration
- [ ] Cloudflare R2 for image storage
- [ ] Admin authentication
- [ ] Extended product properties (SKU, variants, pricing details)
- [ ] Multiple images per product

### Phase 3: Enhanced Features
- [ ] Product search & filtering
- [ ] Categories and collections
- [ ] User accounts & order history
- [ ] Product reviews
- [ ] Wishlist

### Phase 4: Production Ready
- [ ] Payment integration
- [ ] Email notifications
- [ ] Custom domain
- [ ] SEO optimization

**See [TODO.md](TODO.md) for complete roadmap**

---

## 🛠️ Development

### Adding Products (Admin)

1. Click on the **logo** in the header to access admin panel
2. Fill in product details:
   - Name
   - Description
   - Price 
   - Upload image
3. Click "Add Product"
4. Success notification will appear! ✨

### Deploying to Shuttle

```bash
# Deploy to production
cargo shuttle deploy

# View logs
cargo shuttle logs

# Check status
cargo shuttle status
```

---

## 🌍 Environment

**Development:**
- Built entirely on mobile using Termux

**Production:**
- Deployed on Shuttle.rs
- Live at: [https://rasta-beings-qoxw.shuttle.app](https://rasta-beings-qoxw.shuttle.app)

---

## 🎨 Design Philosophy

**Rastafarian Values:**
- **Peace** ☮️ - Harmonious user experience
- **Unity** ✊🏿 - Community-focused platform
- **Respect for Nature** 🌿 - Natural patterns and organic design

**Color Meaning:**
- 🔴 **Red** - Blood of martyrs, strength
- 💛 **Gold** - Wealth of homeland, sunshine
- 💚 **Green** - Vegetation, hope

**Typography:**
- **Philosopher** - Mystical serif for headers
- **Raleway** - Clean sans-serif for body text

---

## 📝 API Routes

| Method | Route | Description |
|--------|-------|-------------|
| GET | `/` | Product catalog (shop) |
| GET | `/cart` | View shopping cart |
| POST | `/cart/add` | Add product to cart |
| POST | `/cart/update` | Update item quantity |
| POST | `/checkout` | Complete order |
| GET | `/admin` | Admin panel |
| POST | `/admin/products` | Create new product |
| POST | `/admin/products/delete` | Delete product |
| GET | `/logo.jpg` | Logo asset |

---

## 🤝 Contributing

This is currently a personal project.
---

## 📜 License

MIT License

---

## 🙏 Acknowledgments

- **Bob Marley** - For the inspiration and wisdom
- **Rastafarian Culture** - For the values of peace, unity, and love
- **Shuttle.rs** - For making Rust deployment effortless
- **Rust Community** - For the amazing ecosystem

---

## 💬 Contact

- GitHub: [@alfredjulianstanley](https://github.com/alfredjulianstanley)
- Live Demo: [rasta-beings-qoxw.shuttle.app](https://rasta-beings-qoxw.shuttle.app)

---

<div align="center">

**🌿 Let's get together and feel alright 🌿**

*Built with 🦀 and ❤️ *

</div>
