#!/usr/bin/env python3
"""
Download popular Fabric mods for 26.3 from Modrinth.
"""

import argparse
import json
import os
import sys
import urllib.request
from pathlib import Path
from typing import Dict, List, Optional


MODRINTH_API = "https://api.modrinth.com/v2"


def search_fabric_mods(version: str = "26.3", limit: int = 30) -> List[Dict]:
    """Search Modrinth for Fabric mods for the given version."""
    url = f"{MODRINTH_API}/search"
    params = {
        "facets": json.dumps([["categories:fabric"], [f"versions:{version}"]]),
        "limit": limit,
        "index": "relevance",
    }
    query_string = "&".join(f"{k}={urllib.parse.quote(str(v))}" for k, v in params.items())
    full_url = f"{url}?{query_string}"
    
    req = urllib.request.Request(full_url, headers={"User-Agent": "HorizonFabricCensus/1.0"})
    with urllib.request.urlopen(req) as response:
        data = json.load(response)
    return data.get("hits", [])


def get_project_versions(project_id: str, version: str) -> List[Dict]:
    """Get versions for a project matching the given game version."""
    url = f"{MODRINTH_API}/project/{project_id}/version"
    params = {
        "game_versions": json.dumps([version]),
        "loaders": json.dumps(["fabric"]),
    }
    query_string = "&".join(f"{k}={urllib.parse.quote(str(v))}" for k, v in params.items())
    full_url = f"{url}?{query_string}"
    
    req = urllib.request.Request(full_url, headers={"User-Agent": "HorizonFabricCensus/1.0"})
    with urllib.request.urlopen(req) as response:
        return json.load(response)


def download_file(url: str, dest: Path) -> bool:
    """Download a file with progress."""
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "HorizonFabricCensus/1.0"})
        with urllib.request.urlopen(req) as response:
            total = int(response.headers.get("Content-Length", 0))
            downloaded = 0
            with open(dest, "wb") as f:
                while True:
                    chunk = response.read(8192)
                    if not chunk:
                        break
                    f.write(chunk)
                    downloaded += len(chunk)
                    if total > 0:
                        pct = downloaded * 100 // total
                        print(f"\r  Downloading {dest.name}: {pct}%", end="", flush=True)
        print()
        return True
    except Exception as e:
        print(f"\n  Failed to download {url}: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(description="Download Fabric mods for census")
    parser.add_argument("--version", default="26.3", help="Minecraft version")
    parser.add_argument("--limit", type=int, default=30, help="Number of mods to download")
    parser.add_argument("--output", "-o", type=Path, default=Path("mods/fabric-downloads"),
                        help="Output directory")
    args = parser.parse_args()
    
    args.output.mkdir(parents=True, exist_ok=True)
    
    print(f"Searching for Fabric mods for {args.version}...")
    hits = search_fabric_mods(args.version, args.limit)
    print(f"Found {len(hits)} projects")
    
    downloaded = 0
    for hit in hits:
        if downloaded >= args.limit:
            break
        project_id = hit["project_id"]
        name = hit["slug"]
        print(f"\nProcessing {name}...")
        
        try:
            versions = get_project_versions(project_id, args.version)
            if not versions:
                print(f"  No versions for {args.version}")
                continue
            
            # Pick the latest version
            version_info = versions[0]
            files = version_info.get("files", [])
            if not files:
                print(f"  No files in version")
                continue
            
            # Find the primary JAR
            jar_file = None
            for f in files:
                if f["filename"].endswith(".jar") and not f["filename"].endswith("-sources.jar"):
                    jar_file = f
                    break
            
            if not jar_file:
                print(f"  No JAR file found")
                continue
            
            dest = args.output / jar_file["filename"]
            if dest.exists():
                print(f"  Already downloaded: {dest.name}")
                downloaded += 1
                continue
            
            print(f"  Downloading {jar_file['filename']}...")
            if download_file(jar_file["url"], dest):
                downloaded += 1
                print(f"  Success!")
            else:
                print(f"  Failed!")
                
        except Exception as e:
            print(f"  Error processing {name}: {e}")
    
    print(f"\nDownloaded {downloaded} mods to {args.output}")


if __name__ == "__main__":
    import urllib.parse
    main()