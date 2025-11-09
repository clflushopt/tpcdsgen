# TPC-DS Test Scripts

This directory contains scripts for testing the Rust TPC-DS implementation against the Java reference implementation.

## Overview

The testing infrastructure validates that the Rust port generates **byte-for-byte identical** output to the Java
implementation (which itself maintains bug-for-bug compatibility with the original C dsdgen).

## Directory Structure

```
tpcdsgen/
├── tests/
│   └── fixtures/               # Generated reference data (gitignored)
│       └── scale-1/           # Scale factor 1 reference data
│           ├── call_center.dat
│           ├── warehouse.dat
│           └── ... (all 24 tables)
└── scripts/
    ├── generate-fixtures.sh   # Generate Java reference data
    ├── compare-table.sh       # Compare one table
    ├── test-all-tables.sh     # Test all ported tables
    ├── clean-fixtures.sh      # Clean up fixtures
    └── README.md              # This file
```

## Scripts

### 1. `generate-fixtures.sh` - Generate Reference Data

Generates TPC-DS tables using the Java implementation. This creates the "golden reference" data that Rust output is compared against.

**Usage:**
```bash
# Generate all 24 tables (recommended first run)
./scripts/generate-fixtures.sh

# Generate specific tables
./scripts/generate-fixtures.sh call_center warehouse

# Quiet mode (minimal output)
./scripts/generate-fixtures.sh --quiet

# Show help
./scripts/generate-fixtures.sh --help
```

**What it does:**
1. Checks if Java implementation is built (builds if needed)
2. Creates `tests/fixtures/scale-1/` directory
3. Generates each table using Java TPC-DS generator
4. Reports progress and statistics

**Output:**
- Generates `.dat` files in `tests/fixtures/scale-1/`
- Each file contains pipe-delimited rows with trailing pipe: `value1|value2|value3|`
- Files are gitignored (regenerate as needed)

**Time:** ~2-5 minutes for all 24 tables at scale 1

---

### 2. `compare-table.sh` - Compare Single Table

Compares Rust-generated output for a single table against the Java reference fixture.

**Usage:**
```bash
# Compare a table
./scripts/compare-table.sh call_center

# Quiet mode
./scripts/compare-table.sh customer_demographics --quiet

# Show help
./scripts/compare-table.sh --help
```

**What it does:**
1. Checks that Java fixture exists
2. Generates table using Rust implementation
3. Performs byte-for-byte comparison with `diff`
4. Reports results

**Exit codes:**
- `0` - Tables match exactly ✓
- `1` - Tables differ or error occurred ✗

**Output example:**
```
[INFO] =========================================
[INFO] Table Comparison: call_center
[INFO] =========================================
[INFO] Java fixture: tests/fixtures/scale-1/call_center.dat
[INFO] Generating call_center with Rust...
[INFO] Using binary: target/debug/generate_call_center
[INFO] Comparing outputs...
[INFO] Java fixture: 6 rows, 4.0K
[INFO] Rust output:  6 rows, 4.0K
[SUCCESS] ✓ call_center: Outputs match exactly (6 rows)
[INFO] =========================================
```

---

### 3. `test-all-tables.sh` - Test All Ported Tables

Runs comparison tests for all tables that have been ported to Rust. This is the main test suite.

**Usage:**
```bash
# Test all ported tables (verbose)
./scripts/test-all-tables.sh

# Quiet mode (show only summary)
./scripts/test-all-tables.sh --quiet

# Show help
./scripts/test-all-tables.sh --help
```

**What it does:**
1. Discovers all ported tables by scanning `src/bin/generate_*.rs`
2. Builds all Rust generators
3. Compares each table against Java fixture
4. Prints comprehensive summary

**Exit codes:**
- `0` - All tables match ✓
- `1` - One or more tables differ ✗

**Output example:**
```
[INFO] =========================================
[INFO] TPC-DS Table Test Suite
[INFO] =========================================
[INFO] Found 9 ported tables:
[INFO]   - call_center
[INFO]   - customer_demographics
[INFO]   - date_dim
[INFO]   - household_demographics
[INFO]   - income_band
[INFO]   - reason
[INFO]   - ship_mode
[INFO]   - time_dim
[INFO]   - warehouse
[INFO] =========================================
[INFO] Building all Rust table generators...
[SUCCESS] All generators built successfully
[INFO] =========================================

[INFO] Testing: call_center
...
[SUCCESS] ✓ call_center: Outputs match exactly (6 rows)
...

[INFO] =========================================
[INFO] Test Summary
[INFO] =========================================
[INFO] Total tables tested: 9
[SUCCESS] Passed: 9

[SUCCESS] Passed tables:
[SUCCESS]   ✓ call_center
[SUCCESS]   ✓ customer_demographics
[SUCCESS]   ✓ date_dim
[SUCCESS]   ✓ household_demographics
[SUCCESS]   ✓ income_band
[SUCCESS]   ✓ reason
[SUCCESS]   ✓ ship_mode
[SUCCESS]   ✓ time_dim
[SUCCESS]   ✓ warehouse

[INFO] Total time: 5s
[INFO] =========================================
```

---

### 4. `clean-fixtures.sh` - Clean Up Fixtures

Removes all generated fixtures to free up disk space or force regeneration.

**Usage:**
```bash
# Clean with confirmation prompt
./scripts/clean-fixtures.sh

# Clean without confirmation
./scripts/clean-fixtures.sh --yes

# Show help
./scripts/clean-fixtures.sh --help
```

**What it does:**
1. Counts fixture files and reports total size
2. Asks for confirmation (unless `--yes` provided)
3. Deletes entire `tests/fixtures/` directory

---

## Typical Workflow

### Initial Setup
```bash
# 1. Generate all reference fixtures (one-time, or when Java changes)
./scripts/generate-fixtures.sh

# This creates tests/fixtures/scale-1/*.dat files
```

### During Development
```bash
# 2. After implementing a new table, compare it
./scripts/compare-table.sh new_table_name

# 3. Or test all ported tables at once
./scripts/test-all-tables.sh
```

### Cleanup
```bash
# 4. Remove fixtures if needed (can regenerate anytime)
./scripts/clean-fixtures.sh --yes
```

---

## Requirements

- **Java:** Maven-built TPC-DS JAR at `../tpcds/target/tpcds-*-jar-with-dependencies.jar`
- **Rust:** Cargo-built binaries at `target/debug/generate_*` or `target/release/generate_*`
- **Disk space:** ~500MB-1GB for scale 1 fixtures

---

## Troubleshooting

**Problem:** `Java JAR not found`
```bash
cd ../tpcds
mvn clean package
```

**Problem:** `Rust binary not found for table: X`
```bash
cargo build --bin generate_X
```

**Problem:** `Fixture not found`
```bash
./scripts/generate-fixtures.sh X
```

**Problem:** Tables don't match
1. Check if both implementations use same seed (should be deterministic)
2. Verify Rust port logic against Java source
3. Use `diff` output to find first difference
4. Debug specific row/column that differs

---

## Integration with CI/CD

These scripts are designed to be CI-friendly:

```yaml
# Example GitHub Actions workflow
- name: Generate fixtures
  run: ./scripts/generate-fixtures.sh --quiet

- name: Test all tables
  run: ./scripts/test-all-tables.sh --quiet
```

Exit codes make it easy to fail CI on mismatches.

---

## TODOs

- [ ] Support multiple scale factors (scale-10, scale-100)
- [ ] Parallel table generation
- [ ] Performance benchmarking (Rust vs Java speed)
- [ ] Rust integration tests (`tests/integration/*.rs`)
- [ ] MD5 hash validation (faster than full diff for large tables)

---

## Notes

- **Fixtures are gitignored** - They're generated artifacts, not source code
- **Deterministic output** - Same seed always produces same data
- **Byte-for-byte equality** - Not just row count, complete binary match
- **Bug compatibility** - Maintains same quirks as Java/C versions (e.g., leap year bug)
