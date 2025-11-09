#!/usr/bin/env bash
#
# Test all ported Rust tables against Java reference fixtures
#
# Usage:
#   ./scripts/test-all-tables.sh [--quiet]
#
# Exit codes:
#   0 - All tables match
#   1 - One or more tables differ

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

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

# Print usage
usage() {
    cat << EOF
Test all ported Rust tables against Java reference fixtures

Usage:
    $(basename "$0") [--quiet]

Options:
    --quiet         Quiet mode (show only summary)

Examples:
    $(basename "$0")           # Test all ported tables (verbose)
    $(basename "$0") --quiet   # Test all ported tables (quiet)

Exit codes:
    0 - All tables match exactly
    1 - One or more tables differ

EOF
    exit 0
}

# Find all ported Rust table generators
find_ported_tables() {
    local tables=()

    for bin_file in "$PROJECT_ROOT"/src/bin/generate_*.rs; do
        if [[ -f "$bin_file" ]]; then
            local basename
            basename=$(basename "$bin_file" .rs)
            # Extract table name: generate_call_center -> call_center
            local table_name="${basename#generate_}"

            # Skip custom variants
            if [[ "$table_name" != *"_custom" ]]; then
                tables+=("$table_name")
            fi
        fi
    done

    echo "${tables[@]}"
}

# Build all Rust table generators
build_all_generators() {
    log_info "Building all Rust table generators..."

    if cargo build --bins --quiet 2>&1; then
        log_success "All generators built successfully"
        return 0
    else
        log_error "Failed to build Rust generators"
        return 1
    fi
}

# Test a single table
test_table() {
    local table=$1
    local compare_script="$SCRIPT_DIR/compare-table.sh"

    if [[ $QUIET -eq 1 ]]; then
        "$compare_script" "$table" --quiet
    else
        "$compare_script" "$table"
    fi
}

# Main function
main() {
    local passed_tables=()
    local failed_tables=()
    local start_time
    local end_time

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
                log_error "Unknown option: $1"
                usage
                ;;
        esac
    done

    log_info "========================================="
    log_info "TPC-DS Table Test Suite"
    log_info "========================================="

    # Find ported tables
    local ported_tables
    ported_tables=$(find_ported_tables)
    local tables_array=($ported_tables)
    local total_count=${#tables_array[@]}

    log_info "Found $total_count ported tables:"
    for table in "${tables_array[@]}"; do
        log_info "  - $table"
    done
    log_info "========================================="

    # Build all generators
    cd "$PROJECT_ROOT"
    if ! build_all_generators; then
        exit 1
    fi
    log_info "========================================="

    # Test each table
    start_time=$(date +%s)

    for table in "${tables_array[@]}"; do
        log_info ""
        log_info "Testing: $table"
        log_info "-----------------------------------------"

        if test_table "$table"; then
            passed_tables+=("$table")
        else
            failed_tables+=("$table")
        fi

        log_info "-----------------------------------------"
    done

    end_time=$(date +%s)
    local duration=$((end_time - start_time))

    # Print summary
    echo ""
    log_info "========================================="
    log_info "Test Summary"
    log_info "========================================="
    log_info "Total tables tested: $total_count"
    log_success "Passed: ${#passed_tables[@]}"

    if [[ ${#failed_tables[@]} -gt 0 ]]; then
        log_error "Failed: ${#failed_tables[@]}"
        log_error ""
        log_error "Failed tables:"
        for table in "${failed_tables[@]}"; do
            log_error "  ✗ $table"
        done
    fi

    if [[ ${#passed_tables[@]} -gt 0 ]]; then
        echo ""
        log_success "Passed tables:"
        for table in "${passed_tables[@]}"; do
            log_success "  ✓ $table"
        done
    fi

    log_info ""
    log_info "Total time: ${duration}s"
    log_info "========================================="

    # Exit with error if any tables failed
    if [[ ${#failed_tables[@]} -gt 0 ]]; then
        exit 1
    fi

    exit 0
}

main "$@"
