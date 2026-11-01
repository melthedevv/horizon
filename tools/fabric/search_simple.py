#!/usr/bin/env python3
import urllib.request
import json

# Simple search without facets
url = 'https://api.modrinth.com/v2/search?query=server+fabric+26.3'
req = urllib.request.Request(url, headers={'User-Agent': 'test'})
try:
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.loads(resp.read().decode())
        print('Total hits:', data['total_hits'])
        for hit in data['hits'][:50]:
            versions_263 = [v for v in hit['versions'] if '26.3' in v]
            if versions_263:
                print('  ', hit['slug'], '-', hit['title'], '- 26.3 versions:', versions_263[:5], '- server_side:', hit.get('server_side', 'unknown'))
except Exception as e:
    print('Error:', e)