#!/usr/bin/env python3
"""
Build horizon-accel.jar with all dependencies.
Downloads missing dependencies to .local/maven/ for classpath.
"""

import os
import sys
import json
import zipfile
import shutil
import subprocess
import tempfile
import urllib.request
from pathlib import Path

# Configuration
MAVEN_BASE = "https://maven.fabricmc.net"
MAVEN_CENTRAL = "https://repo1.maven.org/maven2"

# Pre-downloaded jars in .local/maven/
DEPENDENCIES = [
    ("net.fabricmc", "fabric-loader-api", "0.20.1", "fabric-loader-0.20.1.jar"),
    ("net.fabricmc.fabric-api", "fabric-api", "0.165.0+26.3", "fabric-api-0.165.0+26.3.jar"),
    ("org.slf4j", "slf4j-api", "2.0.12", "slf4j-api-2.0.12.jar"),
]

REPO_ROOT = Path(__file__).parent.parent.parent
LOCAL_MAVEN = REPO_ROOT / ".local" / "maven"
LOCAL_MODS = REPO_ROOT / ".local" / "fabric-mods"
TARGET_DIR = Path(__file__).parent / "target" / "release"
NATIVE_LIB = TARGET_DIR / "horizon_accel.dll"
OUTPUT_JAR = Path(".local/fabric-mods/horizon-accel-0.1.0.jar")
JAVA_SRC = Path(__file__).parent / "src" / "main" / "java"
RESOURCES = Path(__file__).parent / "resources"
MANIFEST_RESOURCE = Path("resources/accel-manifest.json")

def ensure_dependencies():
    """Verify all required jars exist in .local/maven/"""
    jars = {}
    for group, artifact, version, filename in DEPENDENCIES:
        jar_path = Path(".local/maven") / filename
        if not jar_path.exists():
            raise FileNotFoundError(f"Missing dependency: {jar_path}. Run download_deps.py first.")
        print(f"  Found: {jar_path.name} ({jar_path.stat().st_size} bytes)")
        # Extract artifact name from filename (e.g., fabric-api-0.165.0+26.3.jar -> fabric-api)
        artifact_name = filename.split("-")[1] if "-" in filename else filename.replace(".jar", "")
        jars[artifact] = jar_path
    return jars

def compile_java(jars: dict):
    """Compile the Java mod source with dependencies on classpath."""
    classpath = ";".join([str(jars["fabric-api"]), str(jars["fabric-loader-api"]), str(jars["slf4j-api"])])
    
    classes_dir = Path("build/classes")
    if classes_dir.exists():
        shutil.rmtree(classes_dir)
    classes_dir.mkdir(parents=True, exist_ok=True)
    
    java_files = list(Path("java-src").rglob("*.java"))
    if not java_files:
        raise RuntimeError("No Java source files found in java-src/")
    
    print(f"Compiling {len(java_files)} Java files...")
    cmd = [
        "javac",
        "-cp", classpath,
        "-d", "build/classes",
        "--release", "21",
    ] + [str(f) for f in java_files]
    
    result = subprocess.run(cmd, capture_output=True, text=True, cwd=".")
    if result.returncode != 0:
        print(result.stdout)
        print(result.stderr)
        raise RuntimeError("Java compilation failed")
    print("  Compilation successful")
    return Path("build/classes")

def build_jar(classes_dir: Path):
    """Package the JAR with classes, native lib, resources, and manifest."""
    output_jar = Path("horizon-accel-0.1.0.jar")
    
    with tempfile.TemporaryDirectory() as tmpdir:
        tmpdir = Path(tmpdir)
        classes_dir_dest = Path(tmpdir) / "classes"
        classes_dir_dest.mkdir()
        
        # Copy compiled classes
        shutil.copytree(classes_dir, classes_dir_dest / "dev", dirs_exist_ok=True)
        
        # Native library
        native_dir = Path(tmpdir) / "classes" / "META-INF" / "native" / "windows" / "x86_64"
        native_dir.mkdir(parents=True, exist_ok=True)
        shutil.copy2("target/release/horizon_accel.dll", 
                       Path(tmpdir) / "classes" / "META-INF" / "native" / "windows" / "x86_64" / "horizon_accel.dll")
        
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
            "custom": {"horizon-accel": {
                "protocol_version": 1,
                "native_library": "horizon_accel",
                "manifest_resource": "accel-manifest.json"
            }}
        }
        with open(classes_dir / "fabric.mod.json", "w") as f:
            json.dump(manifest, f, indent=2)
        shutil.copy2("fabric.mod.json", Path(tmpdir) / "classes" / "fabric.mod.json")
        
        # Copy manifest resource
        shutil.copy2("resources/accel-manifest.json", Path(tmpdir) / "classes" / "accel-manifest.json")
        
        # Create JAR
        output_jar = Path("horizon-accel-0.1.0.jar")
        with zipfile.ZipFile(output_jar, "w", zipfile.ZIP_DEFLATED) as zf:
            for root, dirs, files in os.walk(tmpdir):
                for file in os.listdir(root):
                    file_path = os.path.join(root, file)
                    arcname = os.path.relpath(file_path, tmpdir) if os.path.isdir(root) else os.path.relpath(os.path.join(root, file), tmpdir)
                    # Only include files under a temp subdir
                    for root, dirs, files in os.walk("."):
                        for file in os.listdir(root):
                            if os.path.isfile(os.path.join(root, file)):
                                pass
                    pass
            # Use temp dir as base
            cwd = os.getcwd()
            os.chdir(tmpdir)
            try:
                with zipfile.ZipFile("horizon-accel-0.1.0.jar", "w", zipfile.ZIP_DEFLATED) as zf:
                    for root, dirs, files in os.walk("classes"):
                        for file in files:
                            file_path = os.path.join(root, file)
                            arcname = os.path.relpath(os.path.join(root, file), "classes")
                            zf.write(file_path, arcname)
            os.chdir(os.path.dirname(os.getcwd()))
            shutil.copy2(os.path.join(tmpdir, "horizon-accel-0.1.0.jar"), "horizon-accel-0.1.0.jar")
            break

def main():
    print("=== Building horizon-accel.jar ===")
    
    # 1. Verify native library
    if not Path("target/release/horizon_accel.dll").exists():
        print("Building native library...")
        subprocess.run(["cargo", "build", "-p", "horizon-accel", "--features", "chunk_io,worldgen,lighting,collision,meshing,pathfinding", "--release"], check=True)
    
    # 2. Download dependencies (or verify cached)
    print("\n1. Verifying dependencies...")
    jars = {}
    for group, artifact, version, filename in [
        ("net.fabricmc", "fabric-loader-api", "0.20.1", "fabric-loader-0.20.1.jar"),
        ("net.fabricmc.fabric-api", "fabric-api", "0.165.0+26.3", "fabric-api-0.165.0+26.3.jar"),
        ("org.slf4j", "slf4j-api", "2.0.12", "slf4j-api-2.0.12.jar"),
    ]:
        jar_path = Path(f".local/maven/{os.path.basename(f'fabric-api-0.165.0+26.3.jar') if 'fabric-api' in artifact else filename}")
        # Find the correct filename
        for fn in ["fabric-api-0.165.0+26.3.jar", "fabric-loader-0.20.1.jar", "slf4j-api-2.0.12.jar"]:
            p = Path(f".local/maven/{fn}")
            if p.exists():
                artifact_name = fn.replace(".jar", "").split("-")[1] if "-" in fn else fn.replace(".jar", "")
                jars[artifact] = Path(f".local/maven/{fn}")
                break
    
    # 2. Compile Java
    print("\n2. Compiling Java mod...")
    classes_dir = compile_java(jars)
    
    # 3. Build JAR
    print("\n3. Packaging JAR...")
    output = build_jar(Path("build/classes"))
    
    print(f"\n=== Done! JAR at {Path('.local/fabric-mods/horizon-accel-0.1.0.jar')} ===")

if __name__ == "__main__":
    main()