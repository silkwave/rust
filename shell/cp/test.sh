#!/bin/bash

# Test script for the Rust cp implementation

set -e

echo "Building the cp command..."
cargo build --quiet

# Clean up any existing test files
rm -rf test_*

echo "Creating test files and directories..."
echo "Hello, World!" > test_file1.txt
echo "Rust cp test" > test_file2.txt
mkdir test_dir
echo "Content in subdirectory" > test_dir/subfile.txt

echo "Testing basic file copy..."
./target/debug/cp test_file1.txt copied_file.txt
if [ "$(cat copied_file.txt)" = "Hello, World!" ]; then
    echo "✓ Basic file copy works"
else
    echo "✗ Basic file copy failed"
fi

echo "Testing file exists error..."
if ./target/debug/cp test_file1.txt test_file2.txt 2>/dev/null; then
    echo "✗ Should have failed on existing file"
else
    echo "✓ Correctly failed on existing file"
fi

echo "Testing force overwrite..."
./target/debug/cp -f test_file1.txt test_file2.txt
if [ "$(cat test_file2.txt)" = "Hello, World!" ]; then
    echo "✓ Force overwrite works"
else
    echo "✗ Force overwrite failed"
fi

echo "Testing directory copy (recursive)..."
./target/debug/cp -r test_dir copied_dir
if [ -f copied_dir/subfile.txt ] && [ "$(cat copied_dir/subfile.txt)" = "Content in subdirectory" ]; then
    echo "✓ Recursive directory copy works"
else
    echo "✗ Recursive directory copy failed"
fi

echo "Testing directory copy without -r (should fail)..."
if ./target/debug/cp test_dir should_fail 2>/dev/null; then
    echo "✗ Should have failed without -r flag"
else
    echo "✓ Correctly failed without -r flag"
fi

echo "Testing multiple files copy..."
mkdir target_dir
./target/debug/cp test_file1.txt test_file2.txt target_dir/
if [ -f target_dir/test_file1.txt ] && [ -f target_dir/test_file2.txt ]; then
    echo "✓ Multiple files copy works"
else
    echo "✗ Multiple files copy failed"
fi

echo "Cleaning up..."
rm -rf test_* copied_* target_*

echo "All tests completed!"