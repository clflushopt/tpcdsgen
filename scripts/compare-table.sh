#!/usr/bin/env bash
#
# Compare Rust-generated table output with Java reference fixture
#
# Usage:
#   ./scripts/compare-table.sh TABLE_NAME [--quiet]
#
# Exit codes:
#   0 - Tables match exactly
#   1 - Tables differ or error occurred

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Configuration
SCALE_FACTOR=1
FIXTURE_DIR="$PROJECT_ROOT/tests/fixtures/scale-$SCALE_FACTOR"
QUIET=0

# Logging functions
log_info() {
    if [[ $QUIET -eq 0 ]]; then
        echo -e "${BLUE}[INFO]${NC} $*"
    fi
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_diff() {
    echo -e "${YELLOW}[DIFF]${NC} $*"
}

# Print usage
usage() {
    cat << EOF
Compare Rust-generated table output with Java reference fixture

Usage:
    $(basename "$0") TABLE_NAME [--quiet]

Arguments:
    TABLE_NAME      Name of the table to compare (e.g., call_center)

Options:
    --quiet         Quiet mode (minimal output)

Examples:
    $(basename "$0") call_center
    $(basename "$0") customer_demographics --quiet

Exit codes:
    0 - Tables match exactly
    1 - Tables differ or error occurred

EOF
    exit 0
}

# Find Rust binary for table
find_rust_binary() {
    local table=$1
    local binary="$PROJECT_ROOT/target/debug/generate_$table"

    if [[ -f "$binary" ]]; then
        echo "$binary"
        return 0
    fi

    # Try release build
    binary="$PROJECT_ROOT/target/release/generate_$table"
    if [[ -f "$binary" ]]; then
        echo "$binary"
        return 0
    fi

    return 1
}

# Generate table with Rust
generate_rust_table() {
    local table=$1
    local output_file=$2
    local binary

    if ! binary=$(find_rust_binary "$table"); then
        log_error "Rust binary not found for table: $table"
        log_error "Build it with: cargo build --bin generate_$table"
        return 1
    fi

    log_info "Generating $table with Rust..."
    log_info "Using binary: $binary"

    # Create temp directory for generation
    local temp_dir
    temp_dir=$(mktemp -d)

    # Run Rust generator in temp directory
    cd "$temp_dir"
    if ! "$binary" >/dev/null 2>&1; then
        log_error "Failed to generate $table with Rust"
        cd - >/dev/null
        rm -rf "$temp_dir"
        return 1
    fi
    cd - >/dev/null

    # Move generated file
    if [[ -f "$temp_dir/${table}.dat" ]]; then
        mv "$temp_dir/${table}.dat" "$output_file"
        rm -rf "$temp_dir"
        return 0
    else
        log_error "Expected Rust output file not found: $temp_dir/${table}.dat"
        rm -rf "$temp_dir"
        return 1
    fi
}

# Compare two files
compare_files() {
    local java_file=$1
    local rust_file=$2
    local table=$3

    log_info "Comparing outputs..."

    # Get file sizes
    local java_size
    local rust_size
    local java_rows
    local rust_rows

    java_size=$(du -h "$java_file" | cut -f1)
    rust_size=$(du -h "$rust_file" | cut -f1)
    java_rows=$(wc -l < "$java_file" | tr -d ' ')
    rust_rows=$(wc -l < "$rust_file" | tr -d ' ')

    log_info "Java fixture: $java_rows rows, $java_size"
    log_info "Rust output:  $rust_rows rows, $rust_size"

    # Quick check: row count must match
    if [[ "$java_rows" != "$rust_rows" ]]; then
        log_error "Row count mismatch!"
        log_error "  Java: $java_rows rows"
        log_error "  Rust: $rust_rows rows"
        return 1
    fi

    # Byte-for-byte comparison
    if diff -q "$java_file" "$rust_file" >/dev/null 2>&1; then
        log_success "✓ $table: Outputs match exactly ($java_rows rows)"
        return 0
    else
        log_error "✗ $table: Outputs differ"
        log_diff "Showing first 10 differences:"
        diff -u "$java_file" "$rust_file" | head -30 || true
        return 1
    fi
}

# Main function
main() {
    local table=""

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --quiet)
                QUIET=1
                shift
                ;;
            --help)
                usage
                ;;
            *)
                if [[ -z "$table" ]]; then
                    table=$1
                else
                    log_error "Too many arguments"
                    usage
                fi
                shift
                ;;
        esac
    done

    # Validate table argument
    if [[ -z "$table" ]]; then
        log_error "Table name required"
        usage
    fi

    log_info "========================================="
    log_info "Table Comparison: $table"
    log_info "========================================="

    # Check if fixture exists
    local fixture_file="$FIXTURE_DIR/${table}.dat"
    if [[ ! -f "$fixture_file" ]]; then
        log_error "Fixture not found: $fixture_file"
        log_error "Generate fixtures first: ./scripts/generate-fixtures.sh $table"
        exit 1
    fi

    log_info "Java fixture: $fixture_file"

    # Generate Rust output
    local rust_output
    rust_output=$(mktemp)

    if ! generate_rust_table "$table" "$rust_output"; then
        rm -f "$rust_output"
        exit 1
    fi

    log_info "Rust output: $rust_output (temporary)"

    # Compare files
    local result=0
    if ! compare_files "$fixture_file" "$rust_output" "$table"; then
        result=1
    fi

    # Cleanup
    rm -f "$rust_output"

    log_info "========================================="

    exit $result
}

main "$@"
