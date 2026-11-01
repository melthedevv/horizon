#!/usr/bin/env python3
import urllib.request
import json

# Known project IDs with their slugs
PROJECTS = {
    "fabric-api": "P7dR8mSH",
    "lithium": "gvQqBUqZ",
    "sodium": "AANobbMI",
    "starlight": "H8CaAYZC",
    "modernfix": "nmDcB62a",
    "entityculling": "NNAgCjsB",
    "iris": "YL57xq9U",
    "modmenu": "mOgUt4GM",
    "cloth-config": "9s6osm5g",
    "journeymap": "lfHFW1mp",
    "no-chat-reports": "qQyHxfxd",
}

for slug, pid in PROJECTS.items():
    url = f'https://api.modrinth.com/v2/project/{pid}/version'
    req = urllib.request.Request(url, headers={'User-Agent': 'test'})
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            versions = json.loads(resp.read().decode())
            found = False
            for v in versions:
                game_versions = v.get('game_versions', [])
                environments = v.get('environments', [])
                if '26.3' in game_versions and ('server' in environments or '*' in environments):
                    print(slug + ': YES 26.3 server - version ' + v['version_number'] + ' (id=' + v['id'] + ')')
                    found = True
                    break
            if not found:
                # Check if any 26.3 version exists at all
                for v in versions:
                    if '26.3' in v.get('game_versions', []):
                        print(slug + ': 26.3 exists but not server - ' + v['version_number'] + ' envs=' + str(v.get('environments', [])))
                        found = True
                        break
                if not found:
                    print(slug + ': NO 26.3 version')
    except Exception as e:
        print(slug + ': ERROR - ' + str(e))