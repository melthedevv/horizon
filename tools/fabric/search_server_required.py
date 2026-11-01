#!/usr/bin/env python3
import urllib.request
import json

# Search for mods with server_side: required for 26.3
url = 'https://api.modrinth.com/v2/search?query=&facets=[["project_type:mod"],["categories:fabric"],["server_side:required"]]'
req = urllib.request.Request(url, headers={'User-Agent': 'test'})
try:
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.loads(resp.read().decode())
        print('Total hits:', data['total_hits'])
        for hit in data['hits'][:30]:
            versions_263 = [v for v in hit['versions'] if '26.3' in v]
            if versions_263:
                print('  ', hit['slug'], '-', hit['title'], '- 26.3 versions:', versions_263[:5])
except Exception as e:
    print('Error:', e)