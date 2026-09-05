#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import argparse
from pathlib import Path

def add_path_to_file(file_path, root_dir, dry_run=False):
    """
    Add relative file path information to the beginning of a Rust file
    
    Args:
        file_path: Path to the file
        root_dir: Root directory for calculating relative path
        dry_run: If True, only preview changes without writing
    """
    file_path = Path(file_path)
    root_dir = Path(root_dir)
    
    if not file_path.exists():
        print(f"Warning: File does not exist - {file_path}")
        return False
    
    if not file_path.is_file():
        print(f"Skipping: Not a file - {file_path}")
        return False
    
    # Only process .rs files
    if file_path.suffix != '.rs':
        print(f"Skipping: Not a .rs file - {file_path}")
        return False
    
    # Calculate relative path
    try:
        relative_path = file_path.relative_to(root_dir)
    except ValueError:
        # If file is not under root_dir, use absolute path
        relative_path = file_path
    
    header = f"// File: {relative_path}\n"
    header += f"// Directory: {relative_path.parent}\n"
    header += f"// Filename: {relative_path.name}\n"
    header += "//" + "=" * 70 + "\n\n"
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        if content.startswith("// File:"):
            print(f"Skipping: Already contains path info - {relative_path}")
            return False
        
        if dry_run:
            print(f"[Preview] Will add path to: {relative_path}")
            print(f"[Preview] Header:\n{header}")
            return True
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(header + content)
        
        print(f"Processed: {relative_path}")
        return True
        
    except UnicodeDecodeError:
        print(f"Skipping: Cannot read with UTF-8 - {relative_path}")
        return False
    except Exception as e:
        print(f"Error: Failed to process {relative_path} - {e}")
        return False

def traverse_directory(root_dir, dry_run=False, exclude_dirs=None):
    """
    Traverse directory and process all .rs files
    
    Args:
        root_dir: Root directory path
        dry_run: If True, only preview changes
        exclude_dirs: List of directory names to exclude
    """
    root_dir = Path(root_dir)
    
    if not root_dir.exists():
        print(f"Error: Directory does not exist - {root_dir}")
        return
    
    if not root_dir.is_dir():
        print(f"Error: Not a directory - {root_dir}")
        return
    
    if exclude_dirs is None:
        exclude_dirs = ['__pycache__', '.git', '.svn', 'node_modules', '.idea', '.vscode', 'venv', 'env', 'target']
    
    all_files = []
    
    print(f"Traversing: {root_dir.absolute()}")
    print(f"Excluding: {', '.join(exclude_dirs)}")
    print(f"Processing: .rs files only")
    print("-" * 70)
    
    for root, dirs, files in os.walk(root_dir):
        dirs[:] = [d for d in dirs if d not in exclude_dirs]
        
        for file in files:
            file_path = Path(root) / file
            
            # Only process .rs files
            if file_path.suffix == '.rs':
                all_files.append(file_path)
    
    all_files.sort()
    
    print(f"Found {len(all_files)} .rs files")
    print("-" * 70)
    
    processed_count = 0
    for file_path in all_files:
        if add_path_to_file(file_path, root_dir, dry_run):
            processed_count += 1
    
    print("-" * 70)
    if not dry_run:
        print(f"Done! Successfully processed {processed_count} .rs files")
    else:
        print(f"Preview mode complete! Will process {processed_count} .rs files")

def main():
    parser = argparse.ArgumentParser(
        description='Traverse directories and add relative file path to each .rs file'
    )
    
    parser.add_argument(
        'directory',
        nargs='?',
        default='.',
        help='Directory to traverse (default: current directory)'
    )
    
    parser.add_argument(
        '-n', '--dry-run',
        action='store_true',
        help='Preview mode, do not actually modify files'
    )
    
    parser.add_argument(
        '-x', '--exclude',
        help='Directories to exclude (comma-separated, e.g., test,backup)'
    )
    
    args = parser.parse_args()
    
    exclude_dirs = None
    if args.exclude:
        exclude_dirs = [d.strip() for d in args.exclude.split(',')]
    
    traverse_directory(args.directory, args.dry_run, exclude_dirs)

if __name__ == '__main__':
    main()