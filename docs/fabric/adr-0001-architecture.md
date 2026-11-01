# ADR 0001: Fabric Compatibility Layer Architecture

## Status
Accepted

## Context
Horizon is a Rust rewrite of Minecraft Java Edition 26.3. We need to run actual Fabric mods (`.jar` files) on Horizon. The Fabric ecosystem for 26.3 has a critical property: **Minecraft 26.1+ is unobfuscated** (Mojang removed obfuscation starting with 26.1). Fabric mods for 26.3 therefore reference **official Mojang names** (e.g., `net.minecraft.client.Minecraft`, `net.minecraft.resources.Identifier`) directly in their bytecode, not Intermediary (`class_1234`) or Yarn names.

Census of 20 popular Fabric 26.3 mods (fabric-api, sodium, iris, lithium, modmenu, etc.) shows top referenced symbols:
- Client rendering: `net.minecraft.client.Minecraft` (15 mods), `net.minecraft.client.gui.GuiGraphicsExtractor` (11), `com.mojang.blaze3d.vertex.PoseStack` (8)
- Core game: `net.minecraft.resources.Identifier` (16), `net.minecraft.core.BlockPos` (11), `net.minecraft.world.level.Level` (12), `net.minecraft.world.level.block.state.BlockState` (10)
- Text: `net.minecraft.network.chat.Component` (14)

## Decision

We choose **Architecture A**: Run the **real upstream Fabric Loader** inside an embedded HotSpot JVM, with a custom Horizon `GameProvider` that bridges Minecraft API calls to Horizon's Rust game state.

### Rationale for A over B (Rust reimplementation of Loader)
- Fabric Loader handles `fabric.mod.json` parsing (all schema versions), nested JARs (jar-in-jar), dependency resolution, environment filtering (`client`/`server`/`*`), entrypoint semantics (`main`, `client`, `server`, `preLaunch`), and Mixin integration correctly. Reimplementing this is a large surface for bugs.
- The census shows mods primarily call **Minecraft classes**, not Fabric Loader internals. The loader is just the bootstrap.
- Upstream Fabric Loader is Apache-2.0, same as our code. We can embed it legally.

### Minecraft API Surface Strategy: Java Shim Layer (Option 1)
- **Approach**: A small Java library (`horizon-shim.jar`) exposing the `net.minecraft.*` classes mods link against. Native methods forward over JNI into Horizon's Rust game state.
- **Coverage ceiling**: The census gives us a ranked list. We implement shim classes in strict descending order of usage frequency.
- **Unsupportable categories**: Mods that patch vanilla internals Horizon replaced (Sodium/Lithium rendering optimizations, deep Mixin overwrites of worldgen/chunk internals). These will fail loudly at load time with a clear error.

### Mixin
- Use real SpongePowered Mixin inside the JVM (bundled with Fabric Loader).
- Explicitly define supported injection kinds: `@Inject` (HEAD/RETURN/TAIL), `@Redirect`, `@ModifyVariable`, `@Shadow`, `@Unique`, accessors/invokers. Unsupported kinds (`@Overwrite`, soft-implements) fail loudly at load time.
- Per-mod compatibility report shown to player.

### JVM Management
- Embedded HotSpot JVM started via JNI Invocation API (`jni` crate 0.21).
- Reuse launcher's Java management (Horizon downloads its own JDK 21).
- One JVM per process, shutdown on game exit, crash capture with log forwarding.

## Consequences

### Positive
- Real Fabric mods load and run entrypoints today (verified by `horizon-fabric-integration` smoke test).
- Inherits correct Fabric Loader behavior (dependency resolution, jar-in-jar, entrypoints, Mixin).
- Clear coverage tracking via `docs/fabric/coverage.md` driven by census.

### Negative
- Requires JVM in the game process (memory overhead ~200-500MB).
- Java shim layer is never "complete" - always chasing census.
- Deep rendering/performance mods (Sodium, Lithium, Iris) cannot work fully because they patch vanilla rendering internals that Horizon reimplemented in Rust/wgpu.

### Neutral
- Mods that stay within their own classes + Fabric API + common Minecraft APIs (blocks, items, entities, text, commands, networking) will work.
- Mods that call `net.minecraft.client.renderer.*` internals will work only for the shimmed subset.

## Implementation Phases

1. **Phase 0** (this ADR): Architecture decision, census, verify naming scheme. ✓
2. **Phase 1**: Runtime foundation - JVM startup, Fabric Loader bootstrap, real `fabric-api` jar loading, fixture mod tests.
3. **Phase 2**: Minecraft API surface - implement shim classes in census order (Identifier, BlockPos, Level, Entity, ItemStack, Component, Registry, etc.), with Rust JNI bridges.
4. **Phase 3**: Mixin - real Mixin agent, supported injection kinds, compatibility report.
5. **Phase 4**: Launcher integration - "Horizon-Fabric" instance type, mod browser with compatibility badges, disable switch.

## References
- [Census data](docs/fabric/census/census.json)
- [Minecraft 26.1 unobfuscation announcement](https://www.minecraft.net/en-us/article/removing-obfuscation-in-java-edition)
- [Fabric Loader source](https://github.com/FabricMC/fabric-loader)
- [Mixin supported injections](https://github.com/SpongePowered/Mixin/wiki/Injection-Points)

## Security/Legal
- Fabric Loader: Apache-2.0 ✓
- Fabric API: Apache-2.0 ✓
- Mixin: MIT ✓
- Java shim: Clean-room from public class/interface names and descriptors only. No decompiled source bundled.
- No Mojang code/assets in repo. Prepared resources stay in `.local/` (gitignored).

## Rollback Plan
If Phase 1 reveals JVM embedding is unstable on target platforms, fall back to Architecture B (Rust loader) with reduced scope (only mods that don't need Minecraft API surface).