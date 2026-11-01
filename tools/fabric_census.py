#!/usr/bin/env python3
"""
Symbol census tool for Fabric mods.
Scans mod JARs and extracts all referenced class/method/field names,
plus mixin targets from mixin config files.
Outputs a ranked census of symbols used by mods.
"""

import argparse
import json
import os
import sys
import zipfile
from collections import Counter
from pathlib import Path
from typing import Dict, List, Set, Tuple


# Known Minecraft/Fabric package prefixes we care about
TARGET_PREFIXES = (
    "net.minecraft.",
    "com.mojang.",
    "net.fabricmc.fabric.api.",
    "net.fabricmc.loader.api.",
    "org.spongepowered.mixin.",
)


def extract_mixin_targets(jar_path: Path) -> Dict[str, List[str]]:
    """
    Extract mixin targets from mixin config files in a JAR.
    Returns dict of mixin_config_name -> list of target classes.
    """
    targets = {}
    try:
        with zipfile.ZipFile(jar_path, 'r') as zf:
            for name in zf.namelist():
                # Mixin configs are typically at root or in META-INF
                # Named like: mixins.modid.json or mixin.modid.json
                if name.endswith('.json') and ('mixin' in name.lower() or 'mixins.' in name.lower()):
                    try:
                        data = zf.read(name)
                        config = json.loads(data.decode('utf-8'))
                        # Extract target classes from various mixin config formats
                        config_targets = set()
                        
                        # Format 1: "mixins": ["com.example.MixinClass"]
                        if 'mixins' in config and isinstance(config['mixins'], list):
                            for mixin in config['mixins']:
                                if isinstance(mixin, str):
                                    config_targets.add(mixin)
                        
                        # Format 2: "client": ["com.example.ClientMixin"]
                        for key in ['client', 'server', 'common']:
                            if key in config and isinstance(config[key], list):
                                for mixin in config[key]:
                                    if isinstance(mixin, str):
                                        config_targets.add(mixin)
                        
                        # Format 3: "targets": { "class_name": { ... } }
                        if 'targets' in config and isinstance(config['targets'], dict):
                            for target_class in config['targets'].keys():
                                if isinstance(target_class, str):
                                    config_targets.add(target_class)
                        
                        # Format 4: individual mixin entries with "target" field
                        for key in ['mixins', 'client', 'server', 'common']:
                            if key in config and isinstance(config[key], list):
                                for entry in config[key]:
                                    if isinstance(entry, dict) and 'target' in entry:
                                        config_targets.add(entry['target'])
                        
                        if config_targets:
                            targets[name] = sorted(config_targets)
                    except Exception as e:
                        print(f"Warning: failed to parse mixin config {name} in {jar_path}: {e}", file=sys.stderr)
    except Exception as e:
        print(f"Warning: failed to extract mixin targets from {jar_path}: {e}", file=sys.stderr)
    return targets


def scan_jar(jar_path: Path) -> Tuple[Counter, Counter, Counter, Dict[str, List[str]]]:
    """
    Scan a single JAR for class/method/field references and mixin targets.
    Returns (class_counter, method_counter, field_counter, mixin_targets).
    """
    classes = Counter()
    methods = Counter()
    fields = Counter()

    try:
        with zipfile.ZipFile(jar_path, 'r') as zf:
            for name in zf.namelist():
                if not name.endswith('.class'):
                    continue
                try:
                    data = zf.read(name)
                    # Simple constant pool parsing for class references
                    # This is a simplified scanner - for production use a proper bytecode library
                    class_refs = extract_class_references(data)
                    for ref in class_refs:
                        if ref.startswith(TARGET_PREFIXES):
                            classes[ref] += 1
                except Exception as e:
                    print(f"Warning: failed to parse {name} in {jar_path}: {e}", file=sys.stderr)
    except zipfile.BadZipFile:
        print(f"Warning: {jar_path} is not a valid ZIP file", file=sys.stderr)
    except Exception as e:
        print(f"Warning: failed to read {jar_path}: {e}", file=sys.stderr)

    mixin_targets = extract_mixin_targets(jar_path)
    return classes, methods, fields, mixin_targets


def extract_class_references(class_data: bytes) -> List[str]:
    """
    Extract class references from Java class file constant pool.
    This is a minimal implementation - a full parser would use ASM or similar.
    """
    refs = []
    
    # Java class file format:
    # u4 magic, u2 minor_version, u2 major_version, u2 constant_pool_count
    # Then constant_pool entries (1-indexed)
    
    if len(class_data) < 10:
        return refs
    
    # Check magic number (0xCAFEBABE)
    if class_data[0:4] != b'\xca\xfe\xba\xbe':
        return refs
    
    # Read constant pool count
    cp_count = int.from_bytes(class_data[8:10], 'big')
    offset = 10
    
    constant_pool = [None]  # 1-indexed
    
    for i in range(1, cp_count):
        if offset >= len(class_data):
            break
        tag = class_data[offset]
        offset += 1
        
        if tag == 7:  # CONSTANT_Class
            if offset + 2 > len(class_data):
                break
            name_index = int.from_bytes(class_data[offset:offset+2], 'big')
            offset += 2
            constant_pool.append(('class', name_index))
        elif tag == 9:  # CONSTANT_Fieldref
            if offset + 4 > len(class_data):
                break
            class_index = int.from_bytes(class_data[offset:offset+2], 'big')
            name_and_type_index = int.from_bytes(class_data[offset+2:offset+4], 'big')
            offset += 4
            constant_pool.append(('fieldref', class_index, name_and_type_index))
        elif tag == 10:  # CONSTANT_Methodref
            if offset + 4 > len(class_data):
                break
            class_index = int.from_bytes(class_data[offset:offset+2], 'big')
            name_and_type_index = int.from_bytes(class_data[offset+2:offset+4], 'big')
            offset += 4
            constant_pool.append(('methodref', class_index, name_and_type_index))
        elif tag == 11:  # CONSTANT_InterfaceMethodref
            if offset + 4 > len(class_data):
                break
            class_index = int.from_bytes(class_data[offset:offset+2], 'big')
            name_and_type_index = int.from_bytes(class_data[offset+2:offset+4], 'big')
            offset += 4
            constant_pool.append(('imethodref', class_index, name_and_type_index))
        elif tag == 8:  # CONSTANT_String
            if offset + 2 > len(class_data):
                break
            string_index = int.from_bytes(class_data[offset:offset+2], 'big')
            offset += 2
            constant_pool.append(('string', string_index))
        elif tag == 12:  # CONSTANT_NameAndType
            if offset + 4 > len(class_data):
                break
            name_index = int.from_bytes(class_data[offset:offset+2], 'big')
            descriptor_index = int.from_bytes(class_data[offset+2:offset+4], 'big')
            offset += 4
            constant_pool.append(('nameandtype', name_index, descriptor_index))
        elif tag == 1:  # CONSTANT_Utf8
            if offset + 2 > len(class_data):
                break
            length = int.from_bytes(class_data[offset:offset+2], 'big')
            offset += 2
            if offset + length > len(class_data):
                break
            try:
                value = class_data[offset:offset+length].decode('utf-8')
            except UnicodeDecodeError:
                value = ''
            offset += length
            constant_pool.append(('utf8', value))
        elif tag in (3, 4):  # CONSTANT_Integer, CONSTANT_Float
            offset += 4
            constant_pool.append((tag, None))
        elif tag in (5, 6):  # CONSTANT_Long, CONSTANT_Double
            offset += 8
            constant_pool.append((tag, None))
            constant_pool.append(None)  # Long/double take two slots
        elif tag == 15:  # CONSTANT_MethodHandle
            offset += 3
            constant_pool.append((tag, None))
        elif tag == 16:  # CONSTANT_MethodType
            offset += 2
            constant_pool.append((tag, None))
        elif tag == 17:  # CONSTANT_Dynamic
            offset += 4
            constant_pool.append((tag, None))
        elif tag == 18:  # CONSTANT_InvokeDynamic
            offset += 4
            constant_pool.append((tag, None))
        elif tag == 19:  # CONSTANT_Module
            offset += 2
            constant_pool.append((tag, None))
        elif tag == 20:  # CONSTANT_Package
            offset += 2
            constant_pool.append((tag, None))
        else:
            # Unknown tag, skip
            pass
    
    # Resolve references
    for entry in constant_pool[1:]:
        if entry is None:
            continue
        if entry[0] == 'class':
            name_index = entry[1]
            if 0 < name_index < len(constant_pool):
                target = constant_pool[name_index]
                if target and target[0] == 'utf8':
                    class_name = target[1].replace('/', '.')
                    if class_name.startswith(TARGET_PREFIXES):
                        refs.append(class_name)
        elif entry[0] in ('fieldref', 'methodref', 'imethodref'):
            class_index = entry[1]
            name_and_type_index = entry[2]
            if 0 < class_index < len(constant_pool):
                class_entry = constant_pool[class_index]
                if class_entry and class_entry[0] == 'class':
                    name_index = class_entry[1]
                    if 0 < name_index < len(constant_pool):
                        target = constant_pool[name_index]
                        if target and target[0] == 'utf8':
                            class_name = target[1].replace('/', '.')
                            if class_name.startswith(TARGET_PREFIXES):
                                refs.append(class_name)
    
    return list(set(refs))  # Deduplicate per class file


def scan_mods_directory(mods_dir: Path, output_dir: Path) -> None:
    """Scan all JARs in a mods directory and write census output."""
    if not mods_dir.exists():
        print(f"Mods directory {mods_dir} does not exist", file=sys.stderr)
        sys.exit(1)
    
    output_dir.mkdir(parents=True, exist_ok=True)
    
    jar_files = list(mods_dir.glob('*.jar'))
    if not jar_files:
        print(f"No JAR files found in {mods_dir}", file=sys.stderr)
        sys.exit(1)
    
    print(f"Scanning {len(jar_files)} mods...")
    
    total_classes = Counter()
    mod_class_counts = {}  # mod_name -> Counter of classes
    all_mixin_targets = {}  # mod_name -> {config_name -> [targets]}
    
    for jar_path in jar_files:
        mod_name = jar_path.stem
        print(f"  Scanning {mod_name}...")
        classes, _, _, mixin_targets = scan_jar(jar_path)
        total_classes.update(classes)
        mod_class_counts[mod_name] = classes
        if mixin_targets:
            all_mixin_targets[mod_name] = mixin_targets
    
    # Write census output
    census = {
        'total_mods': len(jar_files),
        'total_unique_symbols': len(total_classes),
        'symbols': {
            symbol: {
                'total_count': count,
                'mods_using': sum(1 for mod_classes in mod_class_counts.values() if symbol in mod_classes)
            }
            for symbol, count in total_classes.most_common()
        }
    }
    
    output_file = output_dir / 'census.json'
    with open(output_file, 'w') as f:
        json.dump(census, f, indent=2)
    print(f"Census written to {output_file}")
    
    # Also write per-mod breakdown
    per_mod = {}
    for mod_name, classes in mod_class_counts.items():
        per_mod[mod_name] = {k: v for k, v in classes.most_common()}
    
    per_mod_file = output_dir / 'census_per_mod.json'
    with open(per_mod_file, 'w') as f:
        json.dump(per_mod, f, indent=2)
    print(f"Per-mod census written to {per_mod_file}")
    
    # Write mixin targets
    if all_mixin_targets:
        mixin_file = output_dir / 'mixin_targets.json'
        with open(mixin_file, 'w') as f:
            json.dump(all_mixin_targets, f, indent=2)
        print(f"Mixin targets written to {mixin_file}")
        
        # Aggregate mixin targets across all mods
        all_targets = Counter()
        for mod_targets in all_mixin_targets.values():
            for config_targets in mod_targets.values():
                all_targets.update(config_targets)
        
        print("\nTop 50 mixin targets:")
        for i, (target, count) in enumerate(all_targets.most_common(50)):
            print(f"  {i+1:3d}. {target} (mods: {count})")
    
    # Print top 50 symbols
    print("\nTop 50 symbols:")
    for i, (symbol, count) in enumerate(total_classes.most_common(50)):
        mods_using = sum(1 for mod_classes in mod_class_counts.values() if symbol in mod_classes)
        print(f"  {i+1:3d}. {symbol} (refs: {count}, mods: {mods_using})")


def main():
    parser = argparse.ArgumentParser(description='Fabric mod symbol census tool')
    parser.add_argument('mods_dir', type=Path, help='Directory containing mod JARs')
    parser.add_argument('--output', '-o', type=Path, default=Path('docs/fabric/census'),
                        help='Output directory for census data')
    args = parser.parse_args()
    
    scan_mods_directory(args.mods_dir, args.output)


if __name__ == '__main__':
    main()