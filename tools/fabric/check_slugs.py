#!/usr/bin/env python3
import urllib.request
import json

slugs = ['fabric-api', 'lithium', 'sodium', 'ferritecore', 'starlight', 'modernfix', 'entityculling', 'iris', 'modmenu', 'cloth-config', 'roughly-enough-items', 'journeymap', 'no-chat-reports']

for slug in slugs:
    url = f'https://api.modrinth.com/v2/project/{slug}'
    req = urllib.request.Request(url, headers={'User-Agent': 'test'})
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            data = json.loads(resp.read().decode())
            print(slug + ': id=' + data['id'] + ', title=' + data['title'])
    except Exception as e:
        print(slug + ': ERROR - ' + str(e))