#!/usr/bin/env python3
"""
Build horizon-accel.jar - one command builds everything.
"""

import os
import sys
import json
import zipfile
import shutil
import subprocess
import tempfile
from pathlib import Path

def main():
    print("=== Building horizon-accel.jar ===")

    # 1. Verify native library exists
    native = Path("target/release/horizon_accel.dll")
    if not native.exists():
        print("Building native library...")
        subprocess.run(["cargo", "build", "-p", "horizon-accel", "--features", "chunk_io,worldgen,lighting,collision,meshing,pathfinding", "--release"], check=True)

    # 2. Verify dependencies exist in .local/maven/
    for name, path in [("fabric-api", ".local/maven/fabric-api-0.165.0+26.3.jar"),
                        ("fabric-loader-api", ".local/maven/fabric-loader-0.20.1.jar"),
                        ("slf4j-api", ".local/maven/slf4j-api-2.0.12.jar")]:
        if not Path(path).exists():
            raise FileNotFoundError(f"Missing dependency: {path}")
        print(f"  Found: {name}")

    # 2. Compile Java
    print("\n2. Compiling Java mod...")
    classpath = ";".join([
        str(Path(".local/maven/fabric-api-0.165.0+26.3.jar")),
        str(Path(".local/maven/fabric-loader-0.20.1.jar")),
        str(Path(".local/maven/slf4j-api-2.0.12.jar")),
        str(Path(".local/maven/sponge-mixin-0.17.5+mixin.0.8.7.jar")),
    ])

    classes_dir = Path("build/classes")
    if classes_dir.exists():
        shutil.rmtree(classes_dir)
    Path("build/classes").mkdir(parents=True, exist_ok=True)

    # Only compile the main mod classes, not the Mixin classes or ChunkIoModule (which need Minecraft classes)
    java_files = list(Path("crates/horizon-accel/src/main/java/dev/horizonlab/accel").rglob("*.java"))
    java_files = [f for f in java_files if "mixin" not in str(f) and f.name != "ChunkIoModule.java"]
    print(f"Compiling {len(java_files)} Java files (excluding Mixin classes and ChunkIoModule)...")
    cmd = [
        "javac", "-cp", classpath,
        "-d", "build/classes",
        "--release", "21",
    ] + [str(f) for f in java_files]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(result.stdout)
        print(result.stderr)
        raise RuntimeError("Java compilation failed")
    print("  Compilation successful")

    # 3. Build JAR
    print("\n3. Packaging JAR...")
    with tempfile.TemporaryDirectory() as tmpdir:
        tmpdir = Path(tmpdir)
        classes_dir = tmpdir / "classes"
        classes_dir.mkdir()
        # Copy classes directly to preserve package structure: dev/horizonlab/accel/*.class
        # build/classes has dev/horizonlab/accel, so copy build/classes/dev -> classes_dir
        shutil.copytree("build/classes/dev", classes_dir / "dev", dirs_exist_ok=True)

        # fabric.mod.json
        manifest = {
            "schemaVersion": 1,
            "id": "horizon-accel",
            "version": "0.1.0",
            "name": "Horizon Accelerator",
            "description": "Horizon Rust accelerator modules for Fabric",
            "authors": ["Horizon"],
            "license": "MIT",
            "environment": "*",
            "entrypoints": {"main": ["dev.horizonlab.accel.HorizonAccelMod"]},
            "depends": {"fabricloader": ">=0.19.5", "minecraft": "26.3", "fabric-api": "*"},
            "custom": {"horizon-accel": {"protocol_version": 1, "native_library": "horizon_accel", "manifest_resource": "accel-manifest.json"}}
        }
        with open(tmpdir / "fabric.mod.json", "w") as f:
            json.dump(manifest, f, indent=2)
        shutil.copy2(tmpdir / "fabric.mod.json", tmpdir / "classes" / "fabric.mod.json")
        shutil.copy2("crates/horizon-accel/resources/accel-manifest.json", tmpdir / "classes" / "accel-manifest.json")
        shutil.copy2("crates/horizon-accel/resources/horizon-accel.mixins.json", tmpdir / "classes" / "horizon-accel.mixins.json")

        # Native library
        native_dir = tmpdir / "classes" / "META-INF" / "native" / "windows" / "x86_64"
        native_dir.mkdir(parents=True, exist_ok=True)
        shutil.copy2("target/release/horizon_accel.dll", native_dir / "horizon_accel.dll")

        # Create JAR
        with zipfile.ZipFile(tmpdir / "horizon-accel-0.1.0.jar", "w", zipfile.ZIP_DEFLATED) as zf:
            for root, dirs, files in os.walk(tmpdir / "classes"):
                for file in os.listdir(root):
                    file_path = os.path.join(root, file)
                    arcname = os.path.relpath(file_path, tmpdir / "classes")
                    zf.write(file_path, arcname)

        shutil.copy2(tmpdir / "horizon-accel-0.1.0.jar", "horizon-accel-0.1.0.jar")
        shutil.copy2(tmpdir / "horizon-accel-0.1.0.jar", ".local/fabric-mods/horizon-accel-0.1.0.jar")
        print(f"Created: horizon-accel-0.1.0.jar ({Path('horizon-accel-0.1.0.jar').stat().st_size} bytes)")

if __name__ == "__main__":
    main()