# ADR 0002: Fabric Hosted Accelerators (supersedes ADR-0001)

## Status
Accepted

## Context
Horizon is a Rust rewrite of Minecraft Java Edition 26.3. The original plan (ADR-0001) was to embed a JVM, run upstream Fabric Loader, and reimplement the Minecraft API surface in Rust via a shim layer.

**Reality check**: Fabric mods for 26.3 link against **official Mojang names** at runtime (Minecraft 26.1+ is unobfuscated). They also use Mixin to patch vanilla bytecode. A shim layer can never be complete — mods link against real `net.minecraft.*` classes, and Mixin patches real vanilla methods. Reimplementing the full API surface is infeasible.

Census of 20+ popular Fabric 26.3 mods confirms: mods reference real Minecraft classes (`net.minecraft.client.Minecraft`, `net.minecraft.resources.Identifier`, `net.minecraft.world.level.Level`, etc.) and Fabric API modules directly.

## Decision

**"Horizon-Fabric" = the real Minecraft Java game from the player's own copy, run with upstream Fabric Loader, upstream Fabric API, and upstream SpongePowered Mixin, with Horizon's Rust code plugged in as accelerator modules (`horizon-accel`) that replace vanilla subsystems one at a time.**

### Architecture
```
┌─────────────────────────────────────────────────────────────┐
│  Java Process (player's JDK 21, managed by launcher)       │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Fabric Loader (upstream)                              │  │
│  │  ├─ Fabric API (upstream)                             │  │
│  │  ├─ Mixin (upstream)                                  │  │
│  │  ├─ Player's Fabric mods (.jar)                       │  │
│  │  └─ horizon-accel (our Fabric mod)                    │  │
│  │       ├─ Java glue (dev/horizonlab/accel/*)           │  │
│  │       └─ JNI → horizon-accel.dll/so (Rust cdylib)     │  │
│  │             ├─ horizon-world (chunk IO, worldgen)     │  │
│  │             ├─ horizon-core (noise, math, palettes)   │  │
│  │             └─ ...                                    │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Invariants (the contract)

**Invariant A — Baseline.** With all accelerators off, Horizon-Fabric = Vanilla + Fabric. Any mod working there works here. Proven by running census mods on a Modded > Fabric baseline.

**Invariant B — No silent breaks.** Each module has an `accel-manifest.json` listing vanilla classes/methods it supersedes. At launch, a **conflict scanner** reads every installed mod's mixin targets, access wideners, and class references. If a mod touches something a module supersedes, that module auto-falls back to vanilla for this instance. Player is told which module was disabled and by which mod.

**Invariant C — Bit-identical to vanilla.** Every module is differential-tested against the real Java implementation using existing harness fixtures/inputs (281 harnesses). A module not bit-identical ships disabled.

**Invariant D — Worth it.** Every module needs a measured speed-up (microbenchmarks in `docs/fabric/benchmarks.md`) or it ships disabled.

**Fail safe.** Rust panic, native mismatch, or JNI error → caught at boundary, logged, module falls back to vanilla for session. Game never crashes due to accelerator.

## Consequences

### Retired crates (git history preserved)
| Crate | Reason |
|-------|--------|
| `horizon-fabric-jvm` | No embedded JVM; launcher manages Java |
| `horizon-fabric-jni` | Replaced by `horizon-accel` JNI layer |
| `horizon-fabric-mapping` | No shim layer; mods use official names |
| `horizon-fabric-loader` | Use upstream Fabric Loader |
| `horizon-fabric-mixin` | Use upstream Mixin |
| `horizon-fabric-api` | No Rust-side API surface needed |
| `horizon-fabric-integration` | No embedded integration |

### Kept / Reusable
- `tools/fabric_census.py` — extended to extract mixin targets per mod
- Census data in `docs/fabric/census/`
- Baseline test results in `docs/fabric/baseline.md`

### New crate
- `crates/horizon-accel` — `cdylib` + Java mod (`horizon-accel.jar`) with JNI bridge, manifest, conflict scanner, module registry, fallback plumbing

## Launcher Integration
- New tab **"Horizon-Fabric"** in New Instance dialog (`NewKind::HorizonFabric`)
- Creates a **Java instance** (`java: Some(JavaGame { loader: Some(Fabric), ... })`) pinned to 26.3
- New `Meta` field: `accel: Option<AccelSettings>` (module toggles, pinned version)
- Installs: Fabric Loader for 26.3 + `horizon-accel.jar` + native lib + optional Fabric API
- Instance page: Horizon Modules tab (on/off/auto-disabled), compatibility report, Mods tab (Fabric 26.3), "Run without Horizon modules" switch
- Experimental badge, honest description distinguishing from Modded > Fabric
- Reuses existing `loaders.rs`, `java_launch.rs`, `mods_page.rs`, `cli.rs`, export/import, `lang/*.json`

## Legal
- `horizon-accel` mod contains only our code referring to vanilla names (same as any Fabric mod)
- No Mojang code/assets/decompiled source in repo or jar
- `tools/check_public_tree.py` must pass
- Publish compiled jar only after maintainer confirmation

## Phases

**P0 (this PR).** Verify Fabric 26.3 runtime names (official), extend census tool to extract mixin targets, run on 30+ popular mods, create Modded > Fabric 26.3 baseline instance, record which census mods pass in `docs/fabric/baseline.md`.

**P1.** `horizon-accel` skeleton: mod jar, native lib load/handshake, fallback plumbing, manifest format, conflict scanner with tests, launcher option end-to-end. Zero modules is fine.

**P2+.** Modules (one PR each with manifest, diff tests, benchmark, fallback):
1. Chunk/region IO, NBT, paletted containers
2. World generation: noise, surface rules, structures
3. Lighting engine
4. Collision and voxel shapes
5. Section meshing → vertex data for Java renderer
6. Pathfinding, mob AI, redstone, block ticking (where clean boundary exists)

Rendering/GL context ownership: design note only, out of scope until above ship.

## Acceptance
- Clean profile → New instance > Horizon-Fabric > Create (GUI/CLI) → working Fabric instance with accelerator installed, toggles + report visible, smoke test passes (launch, load world, 2 min, no crash)
- 30 census mods behave identically to Modded > Fabric baseline (zero regressions), all conflicts auto-handled and shown
- ≥2 modules shipped, bit-identical in diff tests, measured speed-ups recorded
- Measured numbers reported; no "full support" claim without them