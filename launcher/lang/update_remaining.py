import json
import sys

# Ukrainian
with open('E:\\horison\\horizon\\launcher\\lang\\uk.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

data["Horizon-Fabric"] = "Horizon-Fabric"
data["Horizon-Fabric 26.3"] = "Horizon-Fabric 26.3"
data["Horizon's Rust engine with Fabric mods. Experimental."] = "Rust-двигун Horizon з модами Fabric. Експериментально."
data["Pinned to Minecraft 26.3 with Fabric Loader. No version selection needed."] = "Закріплено на Minecraft 26.3 з Fabric Loader. Вибір версії не потрібен."

with open('E:\\horison\\horizon\\launcher\\lang\\uk.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, ensure_ascii=False, indent=2)

# Chinese
with open('E:\\horison\\horizon\\launcher\\lang\\zh.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

data["Horizon-Fabric"] = "Horizon-Fabric"
data["Horizon-Fabric 26.3"] = "Horizon-Fabric 26.3"
data["Horizon's Rust engine with Fabric mods. Experimental."] = "Horizon的Rust引擎搭配Fabric模组。实验性。"
data["Pinned to Minecraft 26.3 with Fabric Loader. No version selection needed."] = "固定至Minecraft 26.3与Fabric Loader。无需选择版本。"

with open('E:\\horison\\horizon\\launcher\\lang\\zh.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, ensure_ascii=False, indent=2)

print("Updated uk.json and zh.json")