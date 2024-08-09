#!/bin/bash

output_file="empire_of_wind_codebase.txt"

# Function to create a tree-like structure including files
create_tree() {
    local dir="$1"
    local prefix="$2"

    # Use find to get all entries, sort them so directories come first
    local entries=$(find "$dir" -mindepth 1 -maxdepth 1 | sort -d)

    local num_entries=$(echo "$entries" | wc -l)
    local count=0

    while IFS= read -r entry; do
        count=$((count + 1))
        local name=$(basename "$entry")

        if [ $count -eq $num_entries ]; then
            new_prefix="$prefix└── "
            next_prefix="$prefix    "
        else
            new_prefix="$prefix├── "
            next_prefix="$prefix│   "
        fi

        echo "$new_prefix$name"

        if [ -d "$entry" ]; then
            create_tree "$entry" "$next_prefix"
        fi
    done <<< "$entries"
}

# Write Cargo.toml contents
echo "=== Cargo.toml ===" > "$output_file"
cat Cargo.toml >> "$output_file"
echo -e "\n" >> "$output_file"

# Create and write src/ directory tree
echo "=== Source Directory Structure ===" >> "$output_file"
create_tree "src" "" >> "$output_file"
echo -e "\n" >> "$output_file"

# Write contents of files in src/ directory
find src -type f | while read -r file; do
    echo "=== $file ===" >> "$output_file"
    cat "$file" >> "$output_file"
    echo -e "\n" >> "$output_file"
done

# Create and write assets/ directory tree
echo "=== Assets Directory Structure ===" >> "$output_file"
create_tree "assets" "" >> "$output_file"
echo -e "\n" >> "$output_file"

# List contents of assets directory
echo "=== Assets File List ===" >> "$output_file"
find assets -type f >> "$output_file"

echo "Project contents have been written to $output_file for LLM knowledge base"