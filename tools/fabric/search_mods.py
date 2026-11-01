#!/usr/bin/env python3
import urllib.request
import json

# Try a simpler search - just search for 26.3
url = 'https://api.modrinth.com/v2/search?query=26.3&facets=[["project_type:mod"],["categories:fabric"]]'
req = urllib.request.Request(url, headers={'User-Agent': 'HorizonFabricMatrix/1.0'})
try:
    with urllib.request.urlopen(req, timeout=30) as response:
        data = json.loads(response.read().decode())
        print('Total hits:', data['total_hits'])
        for hit in data['hits'][:30]:
            print('  ', hit['slug'], '-', hit['title'], '- versions:', hit['versions'][:5], '- environments:', hit.get('server_side', 'unknown'))
except Exception as e:
    print('Error:', e)

# Also try without facets
url2 = 'https://api.modrinth.com/v2/search?query=26.3'
req2 = urllib.request.Request(url2, headers={'User-Agent': 'HorizonFabricMatrix/1.0'})
try:
    with urllib.request.urlopen(req2, timeout=30) as response:
        data = json.loads(response.read().decode())
        print('\n--- Simple query 26.3 ---')
        print('Total hits:', data['total_hits'])
        for hit in data['hits'][:30]:
            print('  ', hit['slug'], '-', hit['title'], '- versions:', hit['versions'][:5], '- server_side:', hit.get('server_side', 'unknown'))
except Exception as e:
    print('Error:', e)