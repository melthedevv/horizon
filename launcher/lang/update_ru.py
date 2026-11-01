import json
import sys

# Read the file
with open('E:\\horison\\horizon\\launcher\\lang\\ru.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

# Add new translations
data["Horizon-Fabric"] = "Horizon-Fabric"
data["Horizon-Fabric 26.3"] = "Horizon-Fabric 26.3"
data["Horizon's Rust engine with Fabric mods. Experimental."] = "Rust-движок Horizon с модами Fabric. Экспериментально."
data["Pinned to Minecraft 26.3 with Fabric Loader. No version selection needed."] = "Закреплено на Minecraft 26.3 с Fabric Loader. Выбор версии не требуется."
# Note: duplicate keys will be overwritten, but JSON doesn't support duplicate keys anyway

# Write back
with open('E:\\horison\\horizon\\launcher\\lang\\ru.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, ensure_ascii=False, indent=2)

print("Updated ru.json")