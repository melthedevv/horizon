# Fabric API Module Status

Tracks which Fabric API modules initialize and work in Horizon.

| Module | Maven Artifact | Status | Notes |
|--------|----------------|--------|-------|
| fabric-api-base | `net.fabricmc.fabric-api:fabric-api-base` | | Core bootstrap |
| fabric-api-lookup | `net.fabricmc.fabric-api:fabric-api-lookup-v1` | | Registry lookups |
| fabric-registry | `net.fabricmc.fabric-api:fabric-registry-v1` | | Registry extensions |
| fabric-events | `net.fabricmc.fabric-api:fabric-events-v1` | | Event callbacks |
| fabric-networking | `net.fabricmc.fabric-api:fabric-networking-v1` | | Custom packets |
| fabric-commands | `net.fabricmc.fabric-api:fabric-commands-v2` | | Brigadier commands |
| fabric-dimensions | `net.fabricmc.fabric-api:fabric-dimensions-v1` | | Custom dimensions |
| fabric-biomes | `net.fabricmc.fabric-api:fabric-biomes-v1` | | Biome API |
| fabric-particles | `net.fabricmc.fabric-api:fabric-particles-v1` | | Particle types |
| fabric-loot | `net.fabricmc.fabric-api:fabric-loot-v1` | | Loot tables |
| fabric-tags | `net.fabricmc.fabric-api:fabric-tags-v1` | | Tag API |
| fabric-block-entity | `net.fabricmc.fabric-api:fabric-block-entity-v1` | | Block entity helpers |
| fabric-item-groups | `net.fabricmc.fabric-api:fabric-item-groups-v1` | | Creative tabs |
| fabric-fluids | `net.fabricmc.fabric-api:fabric-fluids-v1` | | Fluid API |
| fabric-transfer | `net.fabricmc.fabric-api:fabric-transfer-api-v1` | | Item/fluid/energy transfer |
| fabric-attachments | `net.fabricmc.fabric-api:fabric-attachments-v1` | | Entity/block attachments |
| fabric-resources | `net.fabricmc.fabric-api:fabric-resources-v1` | | Resource reload |
| fabric-gametest | `net.fabricmc.fabric-api:fabric-gametest-v1` | | GameTest framework |
| fabric-keybindings | `net.fabricmc.fabric-api:fabric-keybindings-v1` | | Keybinding API |
| fabric-rendering | `net.fabricmc.fabric-api:fabric-rendering-v1` | | Render helpers |
| fabric-renderer | `net.fabricmc.fabric-api:fabric-renderer-v1` | | Block/item model rendering |
| fabric-crash-report | `net.fabricmc.fabric-api:fabric-crash-report-v1` | | Crash report extensions |

## Status Legend
- ✅ **Working** - Module initializes, core functionality tested
- ⚠️ **Partial** - Initializes but some features missing
- ❌ **Fails** - Initialization error or crash
- ⏳ **Not started** - Not yet attempted

*Updated after each Phase 1/2 milestone*