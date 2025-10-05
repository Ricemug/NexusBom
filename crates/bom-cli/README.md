# BOM CLI - Command Line Interface

Multi-language BOM calculation tool with i18n support.

## 🌍 Supported Languages

- English (en)
- 繁體中文 (zh-TW)
- 简体中文 (zh-CN)
- Deutsch (de)

## 📦 Installation

### Download Binary (Recommended for End Users)

Download the pre-built binary for your platform from [GitHub Releases](https://github.com/Ricemug/NexusBom/releases):

- **Linux**: `bom-linux-amd64`
- **macOS (Intel)**: `bom-macos-amd64`
- **macOS (Apple Silicon)**: `bom-macos-arm64`
- **Windows**: `bom-windows-amd64.exe`

Make it executable (Linux/macOS):
```bash
chmod +x bom-linux-amd64
./bom-linux-amd64 --help
```

### Build from Source

```bash
cargo build --release -p bom-cli
# Binary will be in: target/release/bom
```

## 🚀 Usage

### Basic Commands

```bash
# Material Explosion
bom -i example_bom.json explode BIKE-001 --quantity 10

# Cost Calculation
bom -i example_bom.json cost BIKE-001

# Where-Used Analysis
bom -i example_bom.json where-used TUBE-001
```

### Language Selection

```bash
# Use Traditional Chinese
bom -i example_bom.json --lang zh-TW explode BIKE-001

# Use Simplified Chinese
bom -i example_bom.json --lang zh-CN cost BIKE-001

# Use German
bom -i example_bom.json --lang de where-used TUBE-001

# Auto-detect system language
bom -i example_bom.json explode BIKE-001
```

### Output Formats

```bash
# Table format (default, colored output)
bom -i example_bom.json explode BIKE-001 --format table

# JSON format
bom -i example_bom.json cost BIKE-001 --format json

# CSV format
bom -i example_bom.json where-used TUBE-001 --format csv
```

### Save Output to File

```bash
# Save to file
bom -i example_bom.json -o result.json explode BIKE-001 --format json

# Verbose mode
bom -i example_bom.json -v explode BIKE-001
```

## 📄 Input File Formats

### JSON Format

```json
{
  "components": [
    {
      "id": "BIKE-001",
      "description": "Bicycle",
      "component_type": "FinishedProduct",
      "standard_cost": "1000.00",
      "uom": "EA",
      "procurement_type": "Make",
      "organization": "PLANT-01"
    }
  ],
  "bom_items": [
    {
      "parent_id": "BIKE-001",
      "child_id": "FRAME-001",
      "quantity": "1",
      "scrap_factor": "0",
      "sequence": 10
    }
  ]
}
```

See [example_bom.json](./example_bom.json) for a complete example.

### CSV Format

Simple CSV format with 4 columns: `parent,child,quantity,cost`

```csv
parent,child,quantity,cost
BIKE-001,FRAME-001,1,300.00
BIKE-001,WHEEL-001,2,150.00
FRAME-001,TUBE-001,4,50.00
```

See [example_bom.csv](./example_bom.csv) for a complete example.

## 🎯 Commands

### explode

Explode BOM structure to calculate material requirements.

```bash
bom -i data.json explode <COMPONENT_ID> [OPTIONS]

Options:
  -q, --quantity <QTY>    Quantity to manufacture (default: 1)
  -f, --format <FORMAT>   Output format: table, json, csv (default: table)
```

**Example:**
```bash
bom -i example_bom.json explode BIKE-001 --quantity 100
```

**Output:**
```
Material Explosion Result for BIKE-001 (Qty: 100)

Level | Component  | Quantity
──────────────────────────────────────────────────────────────────────────────
0     | BIKE-001   | 100
1     | FRAME-001  | 100
1     | WHEEL-001  | 200
2     | TUBE-001   | 600

Total items: 4
```

### cost

Calculate total cost for a BOM.

```bash
bom -i data.json cost <COMPONENT_ID> [OPTIONS]

Options:
  -f, --format <FORMAT>   Output format: table, json, csv (default: table)
```

**Example:**
```bash
bom -i example_bom.json cost BIKE-001
```

**Output:**
```
Cost Breakdown for BIKE-001

Total Cost: $1000
Material Cost: $900
Labor Cost: $50
Overhead Cost: $50
```

### where-used

Find where a component is used.

```bash
bom -i data.json where-used <COMPONENT_ID> [OPTIONS]

Options:
  -f, --format <FORMAT>   Output format: table, json, csv (default: table)
```

**Example:**
```bash
bom -i example_bom.json where-used TUBE-001
```

**Output:**
```
Where-Used Analysis for TUBE-001

Used in 2 assemblies

Parent     | Usage Qty
──────────────────────────────────────────────────────────────────────────────
FRAME-001  | 4 | 1
WHEEL-001  | 1 | 1
```

## 🛠️ Global Options

```
Options:
  -i, --input <FILE>      Input file (JSON or CSV format) [required]
  -o, --output <FILE>     Output file (optional, prints to stdout if not specified)
  -l, --lang <LANG>       Language (en, zh-TW, zh-CN, de) [default: auto]
  -v, --verbose           Verbose output
  -h, --help              Print help
  -V, --version           Print version
```

## 📚 Examples

### Complete Workflow

```bash
# 1. Prepare your BOM data (JSON or CSV)
cat > my_bom.csv << EOF
parent,child,quantity,cost
LAPTOP-001,CPU-001,1,500
LAPTOP-001,RAM-001,2,100
LAPTOP-001,SSD-001,1,200
EOF

# 2. Material explosion for 50 laptops
bom -i my_bom.csv explode LAPTOP-001 --quantity 50 --format table

# 3. Calculate cost
bom -i my_bom.csv cost LAPTOP-001 --format json -o cost_result.json

# 4. Find where CPU is used
bom -i my_bom.csv where-used CPU-001
```

### Multi-language Examples

**繁體中文：**
```bash
bom -i example_bom.json --lang zh-TW explode BIKE-001 --quantity 10
```

**简体中文：**
```bash
bom -i example_bom.json --lang zh-CN cost BIKE-001
```

**Deutsch:**
```bash
bom -i example_bom.json --lang de where-used TUBE-001
```

## 🔧 Troubleshooting

### File not found

Make sure the input file path is correct:
```bash
# Use absolute path
bom -i /path/to/your/bom.json explode BIKE-001

# Or relative path from current directory
bom -i ./data/bom.json explode BIKE-001
```

### Component not found

Check that the component ID exists in your input file:
```bash
# Enable verbose mode to see loaded components
bom -i example_bom.json -v explode BIKE-001
```

### Invalid file format

Ensure your file is valid JSON or CSV:
```bash
# Validate JSON
cat example_bom.json | jq .

# Check CSV format
head example_bom.csv
```

## 📞 Support

- **Issues**: https://github.com/Ricemug/NexusBom/issues
- **Email**: xiaoivan1@proton.me
- **Documentation**: https://github.com/Ricemug/NexusBom

## 📜 License

Dual licensed under either:
- AGPL-3.0 License
- Commercial License

See [COMMERCIAL-LICENSE.md](../../COMMERCIAL-LICENSE.md) for commercial licensing options.
