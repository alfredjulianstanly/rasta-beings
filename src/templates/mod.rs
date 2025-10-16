use crate::models::Product;

pub fn base_layout(title: &str, content: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Rasta Beings</title>
    <link href="https://fonts.googleapis.com/css2?family=Philosopher:wght@400;700&family=Raleway:wght@300;400;500;600&display=swap" rel="stylesheet">
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        :root {{
            --primary: #0a0a0a;
            --secondary-gold: #d4af37;
            --rasta-red: #e74c3c;
            --rasta-green: #27ae60;
            --rasta-gold: #f39c12;
            --accent: #8b7355;
            --light: #f5f5f5;
        }}

        body {{
            font-family: 'Raleway', sans-serif;
            background: #0a0a0a;
            background-image: 
                radial-gradient(circle at 20% 50%, rgba(39, 174, 96, 0.03) 0%, transparent 50%),
                radial-gradient(circle at 80% 80%, rgba(243, 156, 18, 0.03) 0%, transparent 50%),
                radial-gradient(circle at 40% 20%, rgba(231, 76, 60, 0.02) 0%, transparent 50%);
            color: var(--light);
            min-height: 100vh;
        }}

        h1, h2, h3 {{
            font-family: 'Philosopher', serif;
            font-weight: 700;
        }}

        .header {{
            background: rgba(15, 15, 15, 0.95);
            backdrop-filter: blur(10px);
            padding: 15px 30px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 4px solid;
            border-image: linear-gradient(90deg, 
                var(--rasta-red) 0%, var(--rasta-red) 33%, 
                var(--rasta-gold) 33%, var(--rasta-gold) 66%, 
                var(--rasta-green) 66%, var(--rasta-green) 100%) 1;
            margin-bottom: 30px;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
            position: relative;
        }}
        
        .header::before {{
            content: '🌿';
            position: absolute;
            left: 10px;
            top: 50%;
            transform: translateY(-50%);
            font-size: 1.5rem;
            opacity: 0.3;
        }}
        
        .header::after {{
            content: '🌿';
            position: absolute;
            right: 10px;
            top: 50%;
            transform: translateY(-50%) scaleX(-1);
            font-size: 1.5rem;
            opacity: 0.3;
        }}
        
        .logo-section {{
            display: flex;
            align-items: center;
            gap: 15px;
            z-index: 1;
        }}
        
        .logo-link {{
            text-decoration: none;
            display: flex;
            align-items: center;
            transition: transform 0.3s ease;
            position: relative;
        }}
        
        .logo-link:hover {{
            transform: scale(1.08) rotate(5deg);
        }}
        
        .logo-link::after {{
            content: '✨';
            position: absolute;
            top: -5px;
            right: -5px;
            font-size: 0.8rem;
            opacity: 0;
            transition: opacity 0.3s ease;
        }}
        
        .logo-link:hover::after {{
            opacity: 1;
        }}
        
        .logo {{
            width: 50px;
            height: 50px;
            object-fit: cover;
            border-radius: 50%;
            border: 3px solid var(--rasta-gold);
            filter: drop-shadow(0 0 15px rgba(243, 156, 18, 0.6));
            transition: all 0.3s ease;
        }}

        .brand-name {{
            font-size: 1.8rem;
            color: var(--rasta-gold);
            letter-spacing: 3px;
            font-weight: 700;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
            position: relative;
        }}
        
        .brand-name::after {{
            content: '☮';
            position: absolute;
            right: -25px;
            top: -8px;
            font-size: 0.6rem;
            color: var(--rasta-green);
            opacity: 0.7;
        }}

        .nav-links {{
            display: flex;
            gap: 20px;
            align-items: center;
            z-index: 1;
        }}

        .nav-link {{
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 5px;
            color: var(--light);
            text-decoration: none;
            font-weight: 500;
            padding: 8px 15px;
            border-radius: 12px;
            transition: all 0.3s ease;
            font-size: 0.85rem;
            position: relative;
            overflow: hidden;
        }}
        
        .nav-link::before {{
            content: '';
            position: absolute;
            top: 0;
            left: -100%;
            width: 100%;
            height: 100%;
            background: linear-gradient(90deg, 
                transparent, 
                rgba(243, 156, 18, 0.2), 
                transparent
            );
            transition: left 0.5s ease;
        }}
        
        .nav-link:hover::before {{
            left: 100%;
        }}

        .nav-link:hover {{
            background: rgba(243, 156, 18, 0.1);
            transform: translateY(-2px);
        }}
        
        .nav-icon {{
            font-size: 1.5rem;
        }}

        .container {{
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }}
        
        .product-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
            gap: 25px;
            margin-top: 30px;
        }}
        
        .product-image {{
            width: 100%;
            height: 250px;
            object-fit: cover;
            border-radius: 0;
        }}
        
        .rasta-accent {{
            height: 4px;
            background: linear-gradient(90deg, 
                var(--rasta-red) 0%, var(--rasta-red) 33%, 
                var(--rasta-gold) 33%, var(--rasta-gold) 66%, 
                var(--rasta-green) 66%, var(--rasta-green) 100%);
            position: relative;
        }}
        
        .rasta-accent::before,
        .rasta-accent::after {{
            content: '🌿';
            position: absolute;
            top: 50%;
            transform: translateY(-50%);
            font-size: 1rem;
            opacity: 0.6;
        }}
        
        .rasta-accent::before {{
            left: -25px;
        }}
        
        .rasta-accent::after {{
            right: -25px;
            transform: translateY(-50%) scaleX(-1);
        }}
        
        .unity-symbol {{
            display: inline-block;
            margin: 0 5px;
            color: var(--rasta-green);
        }}
        
        .leaf-pattern {{
            position: absolute;
            font-size: 3rem;
            opacity: 0.05;
            pointer-events: none;
            z-index: 0;
        }}
        
        @keyframes float {{
            0%, 100% {{ transform: translateY(0px) rotate(0deg); }}
            50% {{ transform: translateY(-10px) rotate(5deg); }}
        }}
        
        .floating {{
            animation: float 6s ease-in-out infinite;
        }}
    </style>
</head>
<body>
    <!-- Floating leaf decorations -->
    <div class="leaf-pattern floating" style="top: 10%; left: 5%; animation-delay: 0s;">🌿</div>
    <div class="leaf-pattern floating" style="top: 30%; right: 10%; animation-delay: 2s;">🍃</div>
    <div class="leaf-pattern floating" style="bottom: 20%; left: 15%; animation-delay: 4s;">🌿</div>
    <div class="leaf-pattern floating" style="top: 60%; right: 5%; animation-delay: 3s;">🍃</div>
    
    <header class="header">
        <div class="logo-section">
            <a href="/admin" class="logo-link" title="Admin Panel">
                <img src="/logo.jpg" alt="Rasta Beings Logo" class="logo">
            </a>
            <h1 class="brand-name">RASTA BEINGS</h1>
        </div>
        <nav class="nav-links">
            <a href="/" class="nav-link">
                <span class="nav-icon">🏪</span>
                <span>Shop</span>
            </a>
            <a href="/cart" class="nav-link">
                <span class="nav-icon">🛒</span>
                <span>Cart</span>
            </a>
        </nav>
    </header>
    <div class="container">
        {content}
    </div>
</body>
</html>"##,
        title = title,
        content = content
    )
}

pub fn product_card(product: &Product) -> String {
    let image_html = if product.icon.starts_with("data:image") {
        format!(r##"<img src="{}" class="product-image" alt="{}">"##, product.icon, product.name)
    } else {
        format!(r##"<div style="width: 100%; height: 250px; display: flex; align-items: center; justify-content: center; font-size: 4rem; background: linear-gradient(135deg, #2a2a2a 0%, #1a1a1a 100%);">{}</div>"##, product.icon)
    };
    
    format!(
        r##"<div style="background: rgba(255, 255, 255, 0.03); border-radius: 12px; overflow: hidden; border: 1px solid rgba(212, 175, 55, 0.2); transition: all 0.3s ease; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3); position: relative;" onmouseover="this.style.transform='translateY(-8px)'; this.style.boxShadow='0 8px 30px rgba(243, 156, 18, 0.4)'; this.style.borderColor='rgba(243, 156, 18, 0.5)';" onmouseout="this.style.transform=''; this.style.boxShadow='0 4px 20px rgba(0, 0, 0, 0.3)'; this.style.borderColor='rgba(212, 175, 55, 0.2)';">
            <div class="rasta-accent"></div>
            {}
            <div style="padding: 20px; position: relative;">
                <div style="position: absolute; top: 10px; right: 10px; font-size: 1.5rem; opacity: 0.2;">🌿</div>
                <h3 style="color: var(--rasta-gold); margin-bottom: 10px; font-size: 1.3rem; font-family: 'Philosopher', serif;">{}</h3>
                <p style="color: var(--light); margin-bottom: 15px; line-height: 1.6; font-size: 0.9rem; opacity: 0.9;">{}</p>
                <div style="font-size: 1.6rem; color: var(--rasta-gold); font-weight: 700; margin-bottom: 15px;">₹{:.2}</div>
                <form method="post" action="/cart/add">
                    <input type="hidden" name="product_id" value="{}">
                    <button type="submit" style="width: 100%; padding: 14px; background: linear-gradient(135deg, var(--rasta-gold) 0%, var(--secondary-gold) 100%); color: var(--primary); border: none; border-radius: 8px; font-weight: 700; cursor: pointer; text-transform: uppercase; letter-spacing: 1.5px; transition: all 0.3s ease; font-size: 0.9rem; position: relative; overflow: hidden;" onmouseover="this.style.transform='scale(1.02)'; this.style.boxShadow='0 6px 20px rgba(243, 156, 18, 0.5)';" onmouseout="this.style.transform=''; this.style.boxShadow='';">
                        <span style="position: relative; z-index: 1;">Add to Cart</span>
                    </button>
                </form>
            </div>
        </div>"##,
        image_html,
        product.name,
        product.description,
        product.price,
        product.id
    )
}
