import urllib.request
import re

url = "https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.20.1/"
with urllib.request.urlopen(url) as f:
    content = f.read().decode()
    for line in content.split('\n'):
        if '.jar' in line and 'href' in line:
            match = __import__('re').search(r'href="([^"]+\.jar)"', line)
            if match:
                print(match.group(1))