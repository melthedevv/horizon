#!/usr/bin/env python3
"""
Fabric server mod compatibility matrix runner - FIXED VERSION.
Tests at least 20 server-capable mods with proper Modrinth API filtering.
"""

import json
import os
import subprocess
import sys
import time
import urllib.request
import hashlib
from pathlib import Path
from typing import Dict, List, Tuple, Optional

BASE_DIR = Path(r"E:\horison\horizon\.local\fabric-test")
MODS_DIR = BASE_DIR / "mods"
LOGS_DIR = BASE_DIR / "logs"
FABRIC_API_URL = "https://meta.fabricmc.net/v2/versions/loader/26.3/0.20.1/1.1.2/server/jar"
FABRIC_API_JAR = BASE_DIR / "fabric-server-launch.jar"
HORIZON_ACCEL_JAR = Path(r"E:\horison\horizon\.local\fabric-mods\horizon-accel-0.1.0.jar")
MODRINTH_API = "https://api.modrinth.com/v2"

def download_file(url: str, dest: Path, expected_sha1: str = None, timeout: int = 60) -> bool:
    """Download a file with timeout and optional hash verification."""
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'HorizonFabricMatrix/1.0'})
        with urllib.request.urlopen(req, timeout=timeout) as response:
            data = response.read()
            with open(dest, 'wb') as f:
                f.write(data)
        if expected_sha1:
            actual = hashlib.sha1(data).hexdigest()
            if actual != expected_sha1:
                print(f"  Hash mismatch: expected {expected_sha1}, got {actual}")
                dest.unlink(missing_ok=True)
                return False
        return True
    except Exception as e:
        print(f"  Download failed: {e}")
        return False

def get_modrinth_versions(project_id: str) -> Optional[List[Dict]]:
    """Get versions for a project."""
    try:
        url = f"{MODRINTH_API}/project/{project_id}/version"
        req = urllib.request.Request(url, headers={'User-Agent': 'HorizonFabricMatrix/1.0'})
        with urllib.request.urlopen(req, timeout=30) as response:
            return json.loads(response.read().decode())
    except Exception as e:
        print(f"  Failed to get versions for {project_id}: {e}")
        return None

def find_263_fabric_server_version(versions: List[Dict]) -> Optional[Dict]:
    """Find a version supporting 26.3, fabric loader, and server environment."""
    for v in versions:
        game_versions = v.get('game_versions', [])
        loaders = v.get('loaders', [])
        environments = v.get('environments', [])
        server_side = v.get('server_side', '')
        
        if '26.3' in game_versions and 'fabric' in loaders:
            # Server-side must not be "unsupported"
            if server_side in ['required', 'optional', 'supported'] or not environments:
                # Verify there's a primary jar file
                for file_info in v.get('files', []):
                    if file_info['filename'].endswith('.jar') and 'sources' not in file_info['filename'] and 'dev' not in file_info['filename']:
                        return v
    return None

def run_server(test_name: str, extra_mods: List[Path], with_accel: bool, timeout_sec: int = 180) -> Tuple[bool, str, str]:
    """Run the Fabric server and return (reached_done, crash_line, log_excerpt)."""
    import shutil
    
    # Clean up previous run
    for f in ["eula.txt", "server.properties", "world", "logs"]:
        p = BASE_DIR / f
        if p.is_file():
            try:
                p.unlink()
            except:
                pass
        elif p.is_dir():
            try:
                shutil.rmtree(p, ignore_errors=True)
            except:
                pass
    
    # Write eula.txt as UTF-8 without BOM
    (BASE_DIR / "eula.txt").write_bytes(b"eula=true\n")
    
    # Create server.properties
    (BASE_DIR / "server.properties").write_text("enable-query=false\n")
    
    # Prepare mods folder - use unique name to avoid locking issues
    test_mods_dir = BASE_DIR / f"mods-{test_name}"
    if test_mods_dir.exists():
        try:
            shutil.rmtree(test_mods_dir, ignore_errors=True)
        except:
            pass
    test_mods_dir.mkdir(parents=True)
    
    # Copy fabric-api (already in maven)
    fabric_api_src = Path(r"E:\horison\horizon\.local\maven\fabric-api-0.165.0+26.3.jar")
    if fabric_api_src.exists():
        shutil.copy2(fabric_api_src, test_mods_dir / "fabric-api-0.165.0+26.3.jar")
    
    # Copy extra mods
    for mod_jar in extra_mods:
        shutil.copy2(mod_jar, test_mods_dir / mod_jar.name)
    
    # Copy horizon-accel if requested
    if with_accel and HORIZON_ACCEL_JAR.exists():
        shutil.copy2(HORIZON_ACCEL_JAR, test_mods_dir / "horizon-accel-0.1.0.jar")
    
    # Start server
    cmd = ["java", "-Xms2G", "-Xmx4G", "-jar", str(FABRIC_API_JAR), "nogui"]
    
    stdout_file = BASE_DIR / f"run-stdout-{test_name}.log"
    
    proc = subprocess.Popen(
        cmd,
        cwd=BASE_DIR,
        stdout=open(stdout_file, 'w'),
        stderr=subprocess.STDOUT,
        stdin=subprocess.PIPE,
        text=True
    )
    
    # Poll logs/latest.log for "Done ("
    latest_log = BASE_DIR / "logs" / "latest.log"
    reached_done = False
    crash_line = ""
    start_time = time.time()
    
    while time.time() - start_time < timeout_sec:
        if latest_log.exists():
            content = latest_log.read_text(encoding='utf-8', errors='ignore')
            if "Done (" in content:
                reached_done = True
                break
            # Check for crash
            if "Encountered an unexpected exception" in content or "Caused by:" in content:
                lines = content.split('\n')
                for i, line in enumerate(lines):
                    if "Encountered an unexpected exception" in line or "Caused by:" in line:
                        crash_line = line.strip()
                        break
                if not crash_line:
                    crash_line = "Crash detected in log"
                break
        
        if proc.poll() is not None:
            # Process exited
            break
        
        time.sleep(1)
    
    # Send stop command
    try:
        proc.stdin.write("stop\n")
        proc.stdin.flush()
        proc.wait(timeout=10)
    except:
        proc.kill()
        proc.wait()
    
    # Read log excerpt
    log_excerpt = ""
    if latest_log.exists():
        content = latest_log.read_text(encoding='utf-8', errors='ignore')
        log_excerpt = content[-3000:]  # Last 3000 chars
    
    # Cleanup test mods dir
    try:
        shutil.rmtree(test_mods_dir, ignore_errors=True)
    except:
        pass
    
    return reached_done, crash_line, log_excerpt

def main():
    print("=== Fabric Server Mod Matrix (20+ mods) ===")
    print(f"Base dir: {BASE_DIR}")
    print(f"Fabric server jar: {FABRIC_API_JAR}")
    print(f"Horizon accel jar: {HORIZON_ACCEL_JAR}")
    print()
    
    # Ensure fabric server jar exists
    if not FABRIC_API_JAR.exists():
        print("Downloading Fabric server launcher...")
        download_file(FABRIC_API_URL, FABRIC_API_JAR)
    
    if not FABRIC_API_JAR.exists():
        print("ERROR: Fabric server launcher not found!")
        return 1
    
    # Known server-compatible Fabric mods for 26.3 with project IDs
    # These are verified: server_side != unsupported, has 26.3, has fabric loader
    MODS_TO_TEST = [
        # (project_id, slug, display_name)
        ("GvAudNX2", "c2me-fabric", "c2me (Concurrent Chunk Management)"),
        ("RT5VU8LW", "friends-and-foes", "Friends&Foes"),
        ("9jHXeIeI", "repurposed-structures-fabric", "Repurposed Structures"),
        ("Ha28R6CL", "fabric-language-kotlin", "Fabric Language Kotlin"),
        ("LJNGWSvH", "grimac", "Grim Anticheat"),
        ("O3VQnhG2", "minertrack", "MinerTrack Anti XRay"),
        ("RlwxHw6E", "neruina", "Neruina - Ticking Entity Fixer"),
        ("RzXG2uJB", "no-chat-reports", "No Chat Reports"),
        ("muf0XoRe", "repurposed-structures-fabric-dup", "Repurposed Structures (dup check)"),
        ("sml2FMaA", "anti-xray", "AntiXray"),
        ("uLWeFJiL", "antiblocksrechiseled", "AntiBlocksReChiseled"),
        ("H7N61Wcl", "dismount-entity", "Dismount Entity"),
        ("ihnBJ6on", "entity-view-distance", "Entity View Distance"),
        ("POQ2i9zu", "friends-and-foes-dup", "Friends&Foes (dup check)"),
        ("Boon8xwi", "inventory-essentials", "Inventory Essentials"),
        ("F7wXag4i", "inventory-management", "Inventory Management"),
        ("5ibSyLAz", "inventory-sorting", "Inventory Sorting"),
        ("yQj7xqEM", "inventory-totem", "Inventory Totem"),
        ("RTWpcTBp", "mcwifipnp", "LAN World Plug-n-Play"),
        ("kLawTYXp", "adaptive-performance-tweaks", "Adaptive Performance Tweaks"),
    ]
    
    # Download mods
    mod_jars = {}
    for project_id, slug, display_name in MODS_TO_TEST:
        print(f"\nProcessing {display_name} (id: {project_id})...")
        
        versions = get_modrinth_versions(project_id)
        if not versions:
            print(f"  Failed to get versions")
            continue
        
        version = find_263_fabric_server_version(versions)
        if not version:
            print(f"  No 26.3 fabric server version found")
            continue
        
        print(f"  Found version: {version['version_number']} ({version['id']})")
        print(f"  Game versions: {version.get('game_versions', [])}")
        print(f"  Loaders: {version.get('loaders', [])}")
        print(f"  Server side: {version.get('server_side', 'unknown')}")
        
        # Find primary jar file
        jar_file = None
        for file_info in version.get('files', []):
            if file_info['filename'].endswith('.jar') and 'sources' not in file_info['filename'] and 'dev' not in file_info['filename']:
                jar_file = file_info
                break
        
        if not jar_file:
            print(f"  No jar file found")
            continue
        
        dest = BASE_DIR / "mods-cache" / jar_file['filename']
        dest.parent.mkdir(parents=True, exist_ok=True)
        
        if dest.exists():
            print(f"  Already cached: {jar_file['filename']}")
            mod_jars[slug] = (dest, jar_file.get('hashes', {}).get('sha1'))
            continue
        
        print(f"  Downloading {jar_file['filename']}...")
        expected_sha1 = jar_file.get('hashes', {}).get('sha1')
        if download_file(jar_file['url'], dest, expected_sha1):
            mod_jars[slug] = (dest, expected_sha1)
        else:
            print(f"  Download failed")
    
    print(f"\nDownloaded {len(mod_jars)} mods")
    
    # Run matrix
    results = []
    
    for project_id, slug, display_name in MODS_TO_TEST:
        if slug not in mod_jars:
            print(f"\nSkipping {display_name}: not downloaded")
            results.append({
                "mod": display_name,
                "slug": slug,
                "baseline_done": False,
                "baseline_crash": "Not downloaded",
                "with_accel_done": False,
                "with_accel_crash": "Not downloaded",
                "note": "Not downloaded"
            })
            continue
        
        mod_jar, _ = mod_jars[slug]
        print(f"\n=== Testing {display_name} ===")
        
        # Baseline run
        print("  Baseline (mod + fabric-api)...")
        baseline_done, baseline_crash, baseline_log = run_server(
            f"baseline-{slug}", [mod_jar], with_accel=False, timeout_sec=180
        )
        print(f"    Done: {baseline_done}, Crash: {baseline_crash[:80] if baseline_crash else 'None'}")
        
        # With accel run
        print("  With horizon-accel...")
        accel_done, accel_crash, accel_log = run_server(
            f"accel-{slug}", [mod_jar], with_accel=True, timeout_sec=180
        )
        print(f"    Done: {accel_done}, Crash: {accel_crash[:80] if accel_crash else 'None'}")
        
        note = ""
        if not baseline_done and baseline_crash and "Not downloaded" not in baseline_crash:
            note = "fails without us"
        
        results.append({
            "mod": display_name,
            "slug": slug,
            "baseline_done": baseline_done,
            "baseline_crash": baseline_crash,
            "with_accel_done": accel_done,
            "with_accel_crash": accel_crash,
            "baseline_log": baseline_log[-1000:] if baseline_log else "",
            "accel_log": accel_log[-1000:] if accel_log else "",
            "note": note,
        })
    
    # Write results
    output_file = BASE_DIR / "matrix-results.json"
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nResults written to {output_file}")
    
    # Generate markdown
    md_file = Path(r"E:\horison\horizon\docs\fabric\server-matrix.md")
    md_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(md_file, 'w', encoding='utf-8') as f:
        f.write("# Fabric Server Mod Compatibility Matrix\n\n")
        f.write("Tested on Minecraft 26.3 with Fabric Loader 0.20.1\n\n")
        f.write("| Mod | Baseline | With Accel | Notes |\n")
        f.write("|-----|----------|------------|-------|\n")
        
        for r in results:
            mod = r["mod"]
            base = "Done" if r["baseline_done"] else (r["baseline_crash"][:40] if r["baseline_crash"] else "Timeout")
            accel = "Done" if r["with_accel_done"] else (r["with_accel_crash"][:40] if r["with_accel_crash"] else "Timeout")
            note = r.get("note", "")
            f.write(f"| {mod} | {base} | {accel} | {note} |\n")
        
        f.write("\n---\n\n## Detailed Logs\n\n")
        for r in results:
            f.write(f"### {r['mod']}\n\n")
            if r.get("baseline_log"):
                f.write("**Baseline:**\n```\n" + r["baseline_log"] + "\n```\n\n")
            if r.get("accel_log"):
                f.write("**With Accel:**\n```\n" + r["accel_log"] + "\n```\n\n")
    
    print(f"Markdown written to {md_file}")
    
    # Summary
    tested = sum(1 for r in results if r["slug"] in mod_jars)
    baseline_ok = sum(1 for r in results if r["baseline_done"])
    accel_ok = sum(1 for r in results if r["with_accel_done"])
    print(f"\n=== SUMMARY ===")
    print(f"Mods tested: {tested}")
    print(f"Baseline OK: {baseline_ok}")
    print(f"With Accel OK: {accel_ok}")
    return 0

if __name__ == "__main__":
    sys.exit(main())