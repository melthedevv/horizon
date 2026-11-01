#!/usr/bin/env python3
import urllib.request
import json

# Get project IDs for the server-compatible mods
mods = [
    "adaptive-performance-tweaks",
    "anti-xray",
    "antiblocksrechiseled",
    "c2me-fabric",
    "dismount-entity",
    "entity-view-distance",
    "friends-and-foes",
    "grimac",
    "inventory-essentials",
    "inventory-management",
    "inventory-sorting",
    "inventory-totem",
    "mcwifipnp",
    "minertrack",
    "neruina",
    "no-chat-reports",
    "repurposed-structures-fabric",
    "fabric-api",
    "fabric-language-kotlin",
    "friends-and-foes",
    "grimac",
    "minertrack",
    "repurposed-structures-fabric",
]

for slug in mods:
    url = f'https://api.modrinth.com/v2/project/{slug}'
    req = urllib.request.Request(url, headers={'User-Agent': 'test'})
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            data = json.loads(resp.read().decode())
            print(f"{slug}: id={data['id']}, title={data['title']}")
    except Exception as e:
        print(f"{slug}: ERROR - {e}")