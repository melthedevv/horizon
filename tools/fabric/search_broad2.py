#!/usr/bin/env python3
import urllib.request
import json

queries = [
    'fabric',
    'optimization',
    'performance',
    'anti',
    'chat',
    'inventory',
    'world',
    'entity',
    'utility',
]

all_mods = {}
for query in queries:
    url = f'https://api.modrinth.com/v2/search?query={query}'
    req = urllib.request.Request(url, headers={'User-Agent': 'test'})
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = json.loads(resp.read().decode())
            for hit in data['hits'][:30]:
                # Check if it's a Fabric mod
                if 'fabric' in hit.get('categories', []):
                    versions_263 = [v for v in hit['versions'] if '26.3' in v]
                    if versions_263:
                        key = hit['slug']
                        if key not in all_mods:
                            all_mods[key] = {
                                'title': hit['title'],
                                'versions': versions_263,
                                'server_side': hit.get('server_side', 'unknown'),
                                'client_side': hit.get('client_side', 'unknown'),
                            }
    except Exception as e:
        print('Error for query', query, ':', e)

print(f'Total unique mods with 26.3: {len(all_mods)}')
for slug, info in sorted(all_mods.items()):
    print(f'  {slug} - {info["title"]} - 26.3: {info["versions"][:3]} - server: {info["server_side"]} - client: {info["client_side"]}')