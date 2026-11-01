#!/usr/bin/env python3
import urllib.request
import json

# Let's check fabric-api in detail
url = 'https://api.modrinth.com/v2/project/P7dR8mSH/version'
req = urllib.request.Request(url, headers={'User-Agent': 'test'})
with urllib.request.urlopen(req, timeout=10) as resp:
    versions = json.loads(resp.read().decode())
    for v in versions:
        if '26.3' in v.get('game_versions', []):
            print('fabric-api version:', v['version_number'])
            print('  game_versions:', v.get('game_versions', []))
            print('  environments:', v.get('environments', []))
            print('  server_side:', v.get('server_side', 'unknown'))
            print('  client_side:', v.get('client_side', 'unknown'))
            for f in v.get('files', []):
                print('  file:', f['filename'])
            print()