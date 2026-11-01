# Fabric Server Mod Compatibility Matrix

Tested on Minecraft 26.3 with Fabric Loader 0.20.1

| Mod | Baseline | With Accel | Notes |
|-----|----------|------------|-------|
| c2me (Concurrent Chunk Management) | Not downloaded | Not downloaded | Not downloaded |
| Friends&Foes | Not downloaded | Not downloaded | Not downloaded |
| Repurposed Structures | Not downloaded | Not downloaded | Not downloaded |
| Fabric Language Kotlin | Done | Done |  |
| Grim Anticheat | Done | Done |  |
| MinerTrack Anti XRay | Done | Done |  |
| Neruina - Ticking Entity Fixer | Not downloaded | Not downloaded | Not downloaded |
| No Chat Reports | Not downloaded | Not downloaded | Not downloaded |
| Repurposed Structures (dup check) | Done | Done |  |
| AntiXray | Done | Done |  |
| AntiBlocksReChiseled | Done | Done |  |
| Dismount Entity | Done | Done |  |
| Entity View Distance | Done | Done |  |
| Friends&Foes (dup check) | Done | Done |  |
| Inventory Essentials | Done | Done |  |
| Inventory Management | Done | Done |  |
| Inventory Sorting | Done | Done |  |
| Inventory Totem | Done | Done |  |
| LAN World Plug-n-Play | Done | Done |  |
| Adaptive Performance Tweaks | Done | Done |  |

---

## Detailed Logs

### c2me (Concurrent Chunk Management)

### Friends&Foes

### Repurposed Structures

### Fabric Language Kotlin

**Baseline:**
```
rsistent chunks...
[23:56:47] [Server thread/INFO]: Preparing spawn area: 100%
[23:56:47] [Server thread/INFO]: Time elapsed: 2180 ms
[23:56:47] [Server thread/INFO]: Done (2.446s)! For help, type "help"
[23:56:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:56:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:56:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:56:47] [Server thread/INFO]: System chat: Stopping the server
[23:56:47] [Server thread/INFO]: Stopping server
[23:56:47] [Server thread/INFO]: Saving players
[23:56:47] [Server thread/INFO]: Saving worlds
[23:56:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:56:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:56:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

**With Accel:**
```
rsistent chunks...
[23:57:01] [Server thread/INFO]: Preparing spawn area: 100%
[23:57:01] [Server thread/INFO]: Time elapsed: 2288 ms
[23:57:01] [Server thread/INFO]: Done (2.569s)! For help, type "help"
[23:57:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:57:01] [Server thread/INFO]: System chat: Stopping the server
[23:57:01] [Server thread/INFO]: Stopping server
[23:57:01] [Server thread/INFO]: Saving players
[23:57:01] [Server thread/INFO]: Saving worlds
[23:57:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### Grim Anticheat

**Baseline:**
```
rsistent chunks...
[23:57:14] [Server thread/INFO]: Preparing spawn area: 100%
[23:57:14] [Server thread/INFO]: Time elapsed: 1869 ms
[23:57:14] [Server thread/INFO]: Done (2.156s)! For help, type "help"
[23:57:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:57:15] [Server thread/INFO]: System chat: Stopping the server
[23:57:15] [Server thread/INFO]: Stopping server
[23:57:15] [Server thread/INFO]: Saving players
[23:57:15] [Server thread/INFO]: Saving worlds
[23:57:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[23:57:28] [Server thread/INFO]: Preparing spawn area: 100%
[23:57:28] [Server thread/INFO]: Time elapsed: 1888 ms
[23:57:28] [Server thread/INFO]: Done (2.646s)! For help, type "help"
[23:57:28] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:28] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:57:28] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:29] [Server thread/INFO]: System chat: Stopping the server
[23:57:29] [Server thread/INFO]: Stopping server
[23:57:29] [Server thread/INFO]: Saving players
[23:57:29] [Server thread/INFO]: Saving worlds
[23:57:29] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:29] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:57:29] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

### MinerTrack Anti XRay

**Baseline:**
```
rsistent chunks...
[23:57:41] [Server thread/INFO]: Preparing spawn area: 100%
[23:57:41] [Server thread/INFO]: Time elapsed: 2001 ms
[23:57:41] [Server thread/INFO]: Done (2.320s)! For help, type "help"
[23:57:41] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:41] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:41] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:57:42] [Server thread/INFO]: System chat: Stopping the server
[23:57:42] [Server thread/INFO]: Stopping server
[23:57:42] [Server thread/INFO]: Saving players
[23:57:42] [Server thread/INFO]: Saving worlds
[23:57:42] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:42] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:42] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[23:57:54] [Server thread/INFO]: Preparing spawn area: 100%
[23:57:54] [Server thread/INFO]: Time elapsed: 2165 ms
[23:57:54] [Server thread/INFO]: Done (2.440s)! For help, type "help"
[23:57:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:57:55] [Server thread/INFO]: System chat: Stopping the server
[23:57:55] [Server thread/INFO]: Stopping server
[23:57:55] [Server thread/INFO]: Saving players
[23:57:55] [Server thread/INFO]: Saving worlds
[23:57:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:57:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:57:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### Neruina - Ticking Entity Fixer

### No Chat Reports

### Repurposed Structures (dup check)

**Baseline:**
```
rsistent chunks...
[23:58:08] [Server thread/INFO]: Preparing spawn area: 100%
[23:58:08] [Server thread/INFO]: Time elapsed: 1979 ms
[23:58:08] [Server thread/INFO]: Done (2.262s)! For help, type "help"
[23:58:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:58:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:58:08] [Server thread/INFO]: System chat: Stopping the server
[23:58:08] [Server thread/INFO]: Stopping server
[23:58:08] [Server thread/INFO]: Saving players
[23:58:08] [Server thread/INFO]: Saving worlds
[23:58:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:58:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[23:58:21] [Server thread/INFO]: Preparing spawn area: 100%
[23:58:21] [Server thread/INFO]: Time elapsed: 1685 ms
[23:58:21] [Server thread/INFO]: Done (2.143s)! For help, type "help"
[23:58:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:58:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:58:21] [Server thread/INFO]: System chat: Stopping the server
[23:58:21] [Server thread/INFO]: Stopping server
[23:58:21] [Server thread/INFO]: Saving players
[23:58:21] [Server thread/INFO]: Saving worlds
[23:58:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:58:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

### AntiXray

**Baseline:**
```
rsistent chunks...
[23:58:34] [Server thread/INFO]: Preparing spawn area: 100%
[23:58:34] [Server thread/INFO]: Time elapsed: 2223 ms
[23:58:34] [Server thread/INFO]: Done (2.469s)! For help, type "help"
[23:58:34] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:34] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:58:34] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:58:35] [Server thread/INFO]: System chat: Stopping the server
[23:58:35] [Server thread/INFO]: Stopping server
[23:58:35] [Server thread/INFO]: Saving players
[23:58:35] [Server thread/INFO]: Saving worlds
[23:58:35] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:35] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:58:35] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

**With Accel:**
```
rsistent chunks...
[23:58:48] [Server thread/INFO]: Preparing spawn area: 100%
[23:58:48] [Server thread/INFO]: Time elapsed: 2395 ms
[23:58:48] [Server thread/INFO]: Done (2.653s)! For help, type "help"
[23:58:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:58:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:58:49] [Server thread/INFO]: System chat: Stopping the server
[23:58:49] [Server thread/INFO]: Stopping server
[23:58:49] [Server thread/INFO]: Saving players
[23:58:49] [Server thread/INFO]: Saving worlds
[23:58:49] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:58:49] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:58:49] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### AntiBlocksReChiseled

**Baseline:**
```
rsistent chunks...
[23:59:01] [Server thread/INFO]: Preparing spawn area: 100%
[23:59:01] [Server thread/INFO]: Time elapsed: 1864 ms
[23:59:01] [Server thread/INFO]: Done (2.144s)! For help, type "help"
[23:59:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:01] [Server thread/INFO]: System chat: Stopping the server
[23:59:01] [Server thread/INFO]: Stopping server
[23:59:01] [Server thread/INFO]: Saving players
[23:59:01] [Server thread/INFO]: Saving worlds
[23:59:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:02] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:02] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[23:59:14] [Server thread/INFO]: Preparing spawn area: 100%
[23:59:14] [Server thread/INFO]: Time elapsed: 1917 ms
[23:59:14] [Server thread/INFO]: Done (2.176s)! For help, type "help"
[23:59:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:14] [Server thread/INFO]: System chat: Stopping the server
[23:59:14] [Server thread/INFO]: Stopping server
[23:59:14] [Server thread/INFO]: Saving players
[23:59:14] [Server thread/INFO]: Saving worlds
[23:59:14] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

### Dismount Entity

**Baseline:**
```
rsistent chunks...
[23:59:27] [Server thread/INFO]: Preparing spawn area: 100%
[23:59:27] [Server thread/INFO]: Time elapsed: 2223 ms
[23:59:27] [Server thread/INFO]: Done (2.500s)! For help, type "help"
[23:59:27] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:27] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:27] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:28] [Server thread/INFO]: System chat: Stopping the server
[23:59:28] [Server thread/INFO]: Stopping server
[23:59:28] [Server thread/INFO]: Saving players
[23:59:28] [Server thread/INFO]: Saving worlds
[23:59:28] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:28] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:28] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[23:59:41] [Server thread/INFO]: Preparing spawn area: 100%
[23:59:41] [Server thread/INFO]: Time elapsed: 2421 ms
[23:59:41] [Server thread/INFO]: Done (2.696s)! For help, type "help"
[23:59:41] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:41] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:41] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:42] [Server thread/INFO]: System chat: Stopping the server
[23:59:42] [Server thread/INFO]: Stopping server
[23:59:42] [Server thread/INFO]: Saving players
[23:59:42] [Server thread/INFO]: Saving worlds
[23:59:42] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:42] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:42] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### Entity View Distance

**Baseline:**
```
rsistent chunks...
[23:59:54] [Server thread/INFO]: Preparing spawn area: 100%
[23:59:54] [Server thread/INFO]: Time elapsed: 1937 ms
[23:59:54] [Server thread/INFO]: Done (2.207s)! For help, type "help"
[23:59:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[23:59:54] [Server thread/INFO]: System chat: Stopping the server
[23:59:55] [Server thread/INFO]: Stopping server
[23:59:55] [Server thread/INFO]: Saving players
[23:59:55] [Server thread/INFO]: Saving worlds
[23:59:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[23:59:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[23:59:55] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

**With Accel:**
```
rsistent chunks...
[00:00:08] [Server thread/INFO]: Preparing spawn area: 100%
[00:00:08] [Server thread/INFO]: Time elapsed: 2464 ms
[00:00:08] [Server thread/INFO]: Done (2.738s)! For help, type "help"
[00:00:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:00:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:00:08] [Server thread/INFO]: System chat: Stopping the server
[00:00:08] [Server thread/INFO]: Stopping server
[00:00:08] [Server thread/INFO]: Saving players
[00:00:08] [Server thread/INFO]: Saving worlds
[00:00:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:00:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

### Friends&Foes (dup check)

**Baseline:**
```
rsistent chunks...
[00:00:20] [Server thread/INFO]: Preparing spawn area: 100%
[00:00:20] [Server thread/INFO]: Time elapsed: 1815 ms
[00:00:20] [Server thread/INFO]: Done (2.082s)! For help, type "help"
[00:00:20] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:20] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:00:20] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:00:21] [Server thread/INFO]: System chat: Stopping the server
[00:00:21] [Server thread/INFO]: Stopping server
[00:00:21] [Server thread/INFO]: Saving players
[00:00:21] [Server thread/INFO]: Saving worlds
[00:00:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:00:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[00:00:33] [Server thread/INFO]: Preparing spawn area: 100%
[00:00:33] [Server thread/INFO]: Time elapsed: 1970 ms
[00:00:33] [Server thread/INFO]: Done (2.237s)! For help, type "help"
[00:00:33] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:33] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:00:33] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:00:34] [Server thread/INFO]: System chat: Stopping the server
[00:00:34] [Server thread/INFO]: Stopping server
[00:00:34] [Server thread/INFO]: Saving players
[00:00:34] [Server thread/INFO]: Saving worlds
[00:00:34] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:34] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:00:34] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

### Inventory Essentials

**Baseline:**
```
rsistent chunks...
[00:00:47] [Server thread/INFO]: Preparing spawn area: 100%
[00:00:47] [Server thread/INFO]: Time elapsed: 2114 ms
[00:00:47] [Server thread/INFO]: Done (2.389s)! For help, type "help"
[00:00:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:00:47] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:00:47] [Server thread/INFO]: System chat: Stopping the server
[00:00:47] [Server thread/INFO]: Stopping server
[00:00:47] [Server thread/INFO]: Saving players
[00:00:47] [Server thread/INFO]: Saving worlds
[00:00:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:00:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:00:48] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[00:01:01] [Server thread/INFO]: Preparing spawn area: 100%
[00:01:01] [Server thread/INFO]: Time elapsed: 2514 ms
[00:01:01] [Server thread/INFO]: Done (2.808s)! For help, type "help"
[00:01:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:01] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:01:01] [Server thread/INFO]: System chat: Stopping the server
[00:01:01] [Server thread/INFO]: Stopping server
[00:01:01] [Server thread/INFO]: Saving players
[00:01:01] [Server thread/INFO]: Saving worlds
[00:01:02] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:02] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:02] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### Inventory Management

**Baseline:**
```
rsistent chunks...
[00:01:15] [Server thread/INFO]: Preparing spawn area: 100%
[00:01:15] [Server thread/INFO]: Time elapsed: 2290 ms
[00:01:15] [Server thread/INFO]: Done (2.647s)! For help, type "help"
[00:01:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:01:15] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:15] [Server thread/INFO]: System chat: Stopping the server
[00:01:16] [Server thread/INFO]: Stopping server
[00:01:16] [Server thread/INFO]: Saving players
[00:01:16] [Server thread/INFO]: Saving worlds
[00:01:16] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:16] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:01:16] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

**With Accel:**
```
rsistent chunks...
[00:01:29] [Server thread/INFO]: Preparing spawn area: 100%
[00:01:29] [Server thread/INFO]: Time elapsed: 1774 ms
[00:01:29] [Server thread/INFO]: Done (2.360s)! For help, type "help"
[00:01:29] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:29] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:29] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:01:30] [Server thread/INFO]: System chat: Stopping the server
[00:01:30] [Server thread/INFO]: Stopping server
[00:01:30] [Server thread/INFO]: Saving players
[00:01:30] [Server thread/INFO]: Saving worlds
[00:01:30] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:30] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:30] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### Inventory Sorting

**Baseline:**
```
rsistent chunks...
[00:01:44] [Server thread/INFO]: Preparing spawn area: 100%
[00:01:44] [Server thread/INFO]: Time elapsed: 2905 ms
[00:01:44] [Server thread/INFO]: Done (3.197s)! For help, type "help"
[00:01:44] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:44] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:44] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:01:45] [Server thread/INFO]: System chat: Stopping the server
[00:01:45] [Server thread/INFO]: Stopping server
[00:01:45] [Server thread/INFO]: Saving players
[00:01:45] [Server thread/INFO]: Saving worlds
[00:01:45] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:45] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:45] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[00:01:57] [Server thread/INFO]: Preparing spawn area: 100%
[00:01:57] [Server thread/INFO]: Time elapsed: 1952 ms
[00:01:57] [Server thread/INFO]: Done (2.239s)! For help, type "help"
[00:01:57] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:57] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:57] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:01:58] [Server thread/INFO]: System chat: Stopping the server
[00:01:58] [Server thread/INFO]: Stopping server
[00:01:58] [Server thread/INFO]: Saving players
[00:01:58] [Server thread/INFO]: Saving worlds
[00:01:58] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:01:58] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:01:58] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### Inventory Totem

**Baseline:**
```
rsistent chunks...
[00:02:11] [Server thread/INFO]: Preparing spawn area: 100%
[00:02:11] [Server thread/INFO]: Time elapsed: 2202 ms
[00:02:11] [Server thread/INFO]: Done (2.470s)! For help, type "help"
[00:02:11] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:11] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:11] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:02:12] [Server thread/INFO]: System chat: Stopping the server
[00:02:12] [Server thread/INFO]: Stopping server
[00:02:12] [Server thread/INFO]: Saving players
[00:02:12] [Server thread/INFO]: Saving worlds
[00:02:12] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:12] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:12] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

**With Accel:**
```
rsistent chunks...
[00:02:24] [Server thread/INFO]: Preparing spawn area: 100%
[00:02:24] [Server thread/INFO]: Time elapsed: 2074 ms
[00:02:24] [Server thread/INFO]: Done (2.375s)! For help, type "help"
[00:02:25] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:25] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:02:25] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:25] [Server thread/INFO]: System chat: Stopping the server
[00:02:25] [Server thread/INFO]: Stopping server
[00:02:25] [Server thread/INFO]: Saving players
[00:02:25] [Server thread/INFO]: Saving worlds
[00:02:25] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:25] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:02:25] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

### LAN World Plug-n-Play

**Baseline:**
```
rsistent chunks...
[00:02:39] [Server thread/INFO]: Preparing spawn area: 100%
[00:02:39] [Server thread/INFO]: Time elapsed: 2698 ms
[00:02:39] [Server thread/INFO]: Done (3.032s)! For help, type "help"
[00:02:39] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:39] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:39] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:02:39] [Server thread/INFO]: System chat: Stopping the server
[00:02:39] [Server thread/INFO]: Stopping server
[00:02:39] [Server thread/INFO]: Saving players
[00:02:39] [Server thread/INFO]: Saving worlds
[00:02:39] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:40] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:40] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

**With Accel:**
```
rsistent chunks...
[00:02:53] [Server thread/INFO]: Preparing spawn area: 100%
[00:02:53] [Server thread/INFO]: Time elapsed: 2442 ms
[00:02:53] [Server thread/INFO]: Done (2.768s)! For help, type "help"
[00:02:53] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:53] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:53] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:02:54] [Server thread/INFO]: System chat: Stopping the server
[00:02:54] [Server thread/INFO]: Stopping server
[00:02:54] [Server thread/INFO]: Saving players
[00:02:54] [Server thread/INFO]: Saving worlds
[00:02:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:02:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:02:54] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether

```

### Adaptive Performance Tweaks

**Baseline:**
```
rsistent chunks...
[00:03:07] [Server thread/INFO]: Preparing spawn area: 100%
[00:03:07] [Server thread/INFO]: Time elapsed: 2295 ms
[00:03:07] [Server thread/INFO]: Done (2.576s)! For help, type "help"
[00:03:07] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:03:07] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:03:07] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:03:08] [Server thread/INFO]: System chat: Stopping the server
[00:03:08] [Server thread/INFO]: Stopping server
[00:03:08] [Server thread/INFO]: Saving players
[00:03:08] [Server thread/INFO]: Saving worlds
[00:03:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:03:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:03:08] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

**With Accel:**
```
rsistent chunks...
[00:03:21] [Server thread/INFO]: Preparing spawn area: 100%
[00:03:21] [Server thread/INFO]: Time elapsed: 1887 ms
[00:03:21] [Server thread/INFO]: Done (2.155s)! For help, type "help"
[00:03:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:03:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:03:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[00:03:21] [Server thread/INFO]: System chat: Stopping the server
[00:03:21] [Server thread/INFO]: Stopping server
[00:03:21] [Server thread/INFO]: Saving players
[00:03:21] [Server thread/INFO]: Saving worlds
[00:03:21] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[00:03:22] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[00:03:22] [Server thread/INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end

```

