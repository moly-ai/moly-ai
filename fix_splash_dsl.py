#!/usr/bin/env python3
"""Fix Splash DSL runtime errors across all files."""

import re
import os
import sys

MOLY_ROOT = os.path.dirname(os.path.abspath(__file__))


def find_rs_files():
    """Find all .rs files in src/ and moly-kit/src/."""
    files = []
    for root, dirs, fnames in os.walk(MOLY_ROOT):
        # Skip target directory, .git, etc.
        dirs[:] = [d for d in dirs if d not in ('target', '.git', 'packaging')]
        for f in fnames:
            if f.endswith('.rs'):
                files.append(os.path.join(root, f))
    return files


def is_in_script_mod(content, pos):
    """Check if position is inside a script_mod! block."""
    # Find the last script_mod! before this position
    before = content[:pos]
    last_script_mod = before.rfind('script_mod!')
    if last_script_mod == -1:
        return False
    # Find the matching closing brace by counting braces
    brace_start = content.find('{', last_script_mod)
    if brace_start == -1 or brace_start > pos:
        return False
    depth = 0
    for i in range(brace_start, len(content)):
        if content[i] == '{':
            depth += 1
        elif content[i] == '}':
            depth -= 1
            if depth == 0:
                return pos <= i
    return False


def fix_merge_operators(content):
    """Fix draw_bg: {...} -> draw_bg +: {...} etc. Only inside script_mod! blocks."""
    # Properties that need merge operator
    merge_props = [
        'draw_bg', 'draw_text', 'draw_icon', 'draw_selection',
        'draw_cursor', 'draw_gutter', 'draw_indent_guide',
        'draw_cursor_bg', 'draw_block',
    ]

    for prop in merge_props:
        # Pattern: prop: DrawType{  or  prop: {
        # Replace with: prop +: {
        # Only replace if NOT already using +:
        pattern = re.compile(
            rf'(\b{prop})\s*:\s*(?:DrawQuad|DrawText|DrawSvg|DrawColor|DrawFlowBlock)?\s*\{{',
            re.MULTILINE
        )
        new_content = []
        last_end = 0
        for m in pattern.finditer(content):
            if is_in_script_mod(content, m.start()):
                new_content.append(content[last_end:m.start()])
                new_content.append(f'{prop} +: {{')
                last_end = m.end()
        if new_content:
            new_content.append(content[last_end:])
            content = ''.join(new_content)

    return content


def fix_text_style(content):
    """Fix text_style: TextStyle{...} or text_style: {...} -> text_style +: {...}."""
    pattern = re.compile(
        r'(\btext_style)\s*:\s*(?:TextStyle\s*)?\{',
        re.MULTILINE
    )
    new_content = []
    last_end = 0
    for m in pattern.finditer(content):
        if is_in_script_mod(content, m.start()):
            new_content.append(content[last_end:m.start()])
            new_content.append('text_style +: {')
            last_end = m.end()
    if new_content:
        new_content.append(content[last_end:])
        content = ''.join(new_content)
    return content


def fix_icon_walk(content):
    """Fix icon_walk: Walk{...} or icon_walk: {...} -> icon_walk +: {...}."""
    pattern = re.compile(
        r'(\bicon_walk)\s*:\s*(?:Walk\s*)?\{',
        re.MULTILINE
    )
    new_content = []
    last_end = 0
    for m in pattern.finditer(content):
        if is_in_script_mod(content, m.start()):
            new_content.append(content[last_end:m.start()])
            new_content.append('icon_walk +: {')
            last_end = m.end()
    if new_content:
        new_content.append(content[last_end:])
        content = ''.join(new_content)
    return content


def fix_source_to_src(content):
    """Fix Image source: -> src: inside script_mod! blocks."""
    pattern = re.compile(r'\bsource\s*:', re.MULTILINE)
    new_content = []
    last_end = 0
    for m in pattern.finditer(content):
        if is_in_script_mod(content, m.start()):
            new_content.append(content[last_end:m.start()])
            new_content.append('src:')
            last_end = m.end()
    if new_content:
        new_content.append(content[last_end:])
        content = ''.join(new_content)
    return content


def fix_animator_syntax(content):
    """Fix animator syntax inside script_mod! blocks."""
    # animator: Animator{ should stay as-is (already correct)
    # The issue is the state definitions inside
    # Pattern: start = { -> start: AnimatorState{
    # This is already done during initial migration, so skip
    return content


def process_file(filepath):
    with open(filepath, 'r') as f:
        original = f.read()

    content = original
    content = fix_merge_operators(content)
    content = fix_text_style(content)
    content = fix_icon_walk(content)
    content = fix_source_to_src(content)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        relpath = os.path.relpath(filepath, MOLY_ROOT)
        print(f"Fixed: {relpath}")
        return True
    return False


def main():
    files = find_rs_files()
    fixed = 0
    for f in files:
        if process_file(f):
            fixed += 1
    print(f"\nFixed {fixed} files total")


if __name__ == '__main__':
    main()
