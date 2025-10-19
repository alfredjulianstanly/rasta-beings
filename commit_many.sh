#!/bin/bash

# Remove vim swap file
rm -f src/templates/.mod.rs.swp

# Commit 1: Project initialization
git add .gitignore Cargo.toml Cargo.lock
git commit -m "chore: initialize Rust project with Shuttle.rs

- Add Cargo.toml with dependencies (axum, shuttle, tokio, serde, base64)
- Configure shuttle-runtime and shuttle-axum
- Add .gitignore for Rust projects
- Set up workspace structure"

# Commit 2: Core data models
git add src/models/
git commit -m "feat: implement core data models

- Add Product model (id, name, description, price, icon)
- Add Cart and CartItem models
- Implement cart operations (add_item, remove_item)
- Export models via mod.rs"

# Commit 3: Application state
git add src/state/
git commit -m "feat: add in-memory state management

- Implement AppState with Arc<Mutex<HashMap>> for thread-safety
- Store products and carts in memory
- Add auto-incrementing product IDs
- Create default products for demo"

# Commit 4: Templates
git add src/templates/
git commit -m "feat: implement HTML templates with Rasta theme

- Add base_layout with Philosopher and Raleway fonts
- Implement Rasta color scheme (red-gold-green)
- Add product_card template with hover effects
- Include cultural symbols (🌿 ☮️ ✊🏿 🦁)
- Add floating leaf animations
- Implement tricolor accent stripe"

# Commit 5: Shop functionality
git add src/handlers/shop.rs
git commit -m "feat: add product catalog (shop) page

- Implement shop_handler with product grid layout
- Add 'One Love, One Heart, One Destiny' messaging
- Include Bob Marley quote
- Handle empty product state
- Add responsive grid with auto-fill"

# Commit 6: Shopping cart & checkout
git add src/handlers/cart.rs
git commit -m "feat: implement shopping cart and checkout flow

- Add cart view with product thumbnails
- Implement quantity controls (+ and -)
- Add update_quantity_handler for cart modifications
- Create checkout flow with order completion page
- Display order total in INR (₹)
- Add 'Continue Shopping' button
- Include cultural blessing on checkout success
- Clear cart after successful order"

# Commit 7: Admin panel
git add src/handlers/admin.rs
git commit -m "feat: add admin panel with product management

- Create admin interface (accessible via logo click)
- Implement product creation form with multipart upload
- Add base64 image encoding for photos
- Include success toast notification
- Add loading spinner during upload
- Display current products with thumbnails
- Implement product deletion
- Add file size preview
- Include detailed error logging"

# Commit 8: Main application & routing
git add src/main.rs src/handlers/mod.rs
git commit -m "feat: set up Axum server with routing

- Configure Axum router with all endpoints
- Add routes: /, /cart, /admin, /checkout
- Implement logo endpoint with embedded asset
- Export all handlers via mod.rs
- Set up Shuttle.rs integration"

# Commit 9: Logo asset
git add src/assets/IMG_2513.jpg
git commit -m "feat: add Celtic triquetra logo

- Include sacred geometry logo asset
- Embed as compile-time resource
- Serve via /logo.jpg endpoint
- Display as circular image with golden border"

# Show commit history
echo ""
echo "✅ All commits completed successfully!"
echo ""
echo "📊 Commit history:"
git log --oneline --graph -10

echo ""
echo "🎯 Next steps:"
echo "1. Create documentation (README.md, TODO.md, CHECKLIST.md)"
echo "2. Final commit for documentation"
echo "3. Tag as v1.0.0-mvp"
echo "4. Push to GitHub"
