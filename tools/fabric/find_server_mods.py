#!/usr/bin/env python3
import urllib.request
import json

# Search for server-side required mods
url = 'https://api.modrinth.com/v2/search?query=server&facets=[["project_type:mod"],["categories:fabric"]]'
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

print('\n--- Trying different query ---')
# Try empty query with server_side facet
url2 = 'https://api.modrinth.com/v2/search?query=&facets=[["project_type:mod"],["categories:fabric"]]'
req2 = urllib.request.Request(url2, headers={'User-Agent': 'test'})
try:
    with urllib.request.urlopen(req2, timeout=30) as resp:
        data = json.loads(resp.read().decode())
        print('Total hits:', data['total_hits'])
        count = 0
        for hit in data['hits'][:100]:
            versions_263 = [v for v in hit['versions'] if '26.3' in v]
            if versions_263 and hit.get('server_side') == 'required':
                print('  ', hit['slug'], '-', hit['title'], '- 26.3 versions:', versions_263[:5], '- server_side:', hit.get('server_side', 'unknown'))
                count += 1
        print(f'Found {count} mods with server_side=required and 26.3')
except Exception as e:
    print('Error:', e)