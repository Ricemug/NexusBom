# BOM Engine Branding Assets

## Files

### Logo (400x400px)
- `logo.svg` - Vector format (editable)
- `logo.png` - PNG format (for Ko-fi profile image)

**用途：**
- Ko-fi profile image
- GitHub avatar
- Social media profile
- App icons

### Cover (1200x400px)
- `cover.svg` - Vector format (editable)
- `cover.png` - PNG format (for Ko-fi cover)

**用途：**
- Ko-fi cover image
- GitHub social preview
- LinkedIn/Twitter banners
- Documentation headers

## Color Scheme

### Primary Colors
- **Deep Navy**: `#1a1a2e` (background)
- **Blue**: `#4A90E2` (primary accent)
- **Dark Blue**: `#2C5AA0` (secondary)
- **Orange**: `#FF8C42` (highlights)
- **Dark Orange**: `#FF6B35` (CTAs)

### Secondary Colors
- **Light Grey**: `#8899AA` (text secondary)
- **Dark Grey**: `#16213e` (boxes/cards)
- **White**: `#ffffff` (text primary)

## Design Elements

### Logo Design
- BOM tree structure visualization
- Gradient circles for hierarchy
- Professional typography
- Clean, modern look

### Cover Design
- Left: Title and features
- Right: BOM visualization
- Performance metrics highlighted
- Tech stack badges

## Usage Guidelines

### Do's ✅
- Use on dark backgrounds
- Maintain aspect ratios
- Keep sufficient whitespace
- Use for promotional materials

### Don'ts ❌
- Don't distort or stretch
- Don't change color scheme drastically
- Don't add filters or effects
- Don't use on busy backgrounds

## Converting SVG to PNG

If PNG files weren't auto-generated, use one of these methods:

### Online Tools (Easiest)
1. https://convertio.co/svg-png/
2. https://cloudconvert.com/svg-to-png
3. https://svgtopng.com/

### Command Line
```bash
# Using ImageMagick
convert logo.svg -resize 400x400 logo.png
convert cover.svg -resize 1200x400 cover.png

# Using Inkscape
inkscape logo.svg --export-type=png --export-width=400
inkscape cover.svg --export-type=png --export-width=1200

# Using rsvg-convert
rsvg-convert -w 400 -h 400 logo.svg -o logo.png
rsvg-convert -w 1200 -h 400 cover.svg -o cover.png
```

## Customization

To edit these designs:
1. Open SVG files in any vector editor (Inkscape, Adobe Illustrator, Figma)
2. Modify colors, text, or layout
3. Export as PNG in required sizes

## File Sizes

Recommended file sizes for web use:
- **Logo PNG**: < 100 KB
- **Cover PNG**: < 500 KB

If files are too large, compress at https://tinypng.com/

---

Created for BOM Calculation Engine
Design by: Ivan (xiaoivan1@proton.me)
