# Fabric Mod Coverage Tracking

Driven by the [symbol census](census/census.json) of popular Fabric 26.3 mods.

## Legend
- ✅ **Works** - All referenced symbols implemented, mod loads and runs features
- ⚠️ **Partial** - Some symbols missing, limited functionality
- ❌ **Broken** - Critical symbols missing, fails to load or crashes
- 🚫 **Unsupported** - Fundamentally incompatible (patches vanilla internals Horizon replaced)

## Top 50 Symbols Implementation Status

| Rank | Symbol | Mods Using | Status | Notes |
|------|--------|-----------|--------|-------|
| 1 | `net.minecraft.client.Minecraft` | 15 | | Core client entry point |
| 2 | `net.minecraft.network.chat.Component` | 14 | | Text components |
| 3 | `net.minecraft.resources.Identifier` | 16 | | Resource identifiers |
| 4 | `net.minecraft.client.gui.GuiGraphicsExtractor` | 11 | | Rendering |
| 5 | `net.minecraft.core.BlockPos` | 11 | | Block positions |
| 6 | `net.minecraft.world.level.Level` | 12 | | World access |
| 7 | `net.minecraft.util.Mth` | 15 | | Math utilities |
| 8 | `net.minecraft.world.entity.Entity` | 13 | | Entity base |
| 9 | `net.minecraft.network.chat.MutableComponent` | 12 | | Text components |
| 10 | `net.minecraft.world.level.block.state.BlockState` | 10 | | Block states |
| 11 | `net.minecraft.client.gui.Font` | 11 | | Text rendering |
| 12 | `net.minecraft.resources.ResourceKey` | 10 | | Registry keys |
| 13 | `net.minecraft.client.gui.Gui` | 12 | | GUI base |
| 14 | `net.minecraft.client.gui.components.Button` | 9 | | GUI components |
| 15 | `com.mojang.blaze3d.vertex.PoseStack` | 8 | | Rendering matrix |
| 16 | `net.minecraft.client.gui.components.Button$OnPress` | 9 | | GUI callbacks |
| 17 | `net.minecraft.core.Direction` | 10 | | Directions |
| 18 | `com.mojang.blaze3d.systems.RenderSystem` | 9 | | GL state |
| 19 | `net.minecraft.client.gui.screens.Screen` | 10 | | Screens |
| 20 | `net.minecraft.client.multiplayer.ClientLevel` | 8 | | Client world |
| 21 | `net.minecraft.core.Holder` | 12 | | Registry holders |
| 22 | `net.minecraft.world.entity.player.Player` | 10 | | Player entity |
| 23 | `net.minecraft.world.item.ItemStack` | 11 | | Item stacks |
| 24 | `com.mojang.renderpearl.backend.opengl.GlStateManager` | 2 | | GL state (internal) |
| 25 | `net.minecraft.ChatFormatting` | 10 | | Text formatting |
| 26 | `com.mojang.renderpearl.api.device.GpuDevice` | 6 | | GPU device (internal) |
| 27 | `net.minecraft.client.input.MouseButtonEvent` | 8 | | Input |
| 28 | `net.minecraft.core.SectionPos` | 6 | | Chunk sections |
| 29 | `net.minecraft.world.phys.Vec3` | 11 | | Vectors |
| 30 | `com.mojang.blaze3d.vertex.PoseStack$Pose` | 8 | | Rendering |
| 31 | `net.minecraft.client.renderer.GameRenderer` | 9 | | Renderer |
| 32 | `net.minecraft.client.renderer.RenderPipelines` | 9 | | Render pipelines |
| 33 | `com.mojang.blaze3d.platform.InputConstants` | 8 | | Key constants |
| 34 | `net.minecraft.client.input.KeyEvent` | 7 | | Key events |
| 35 | `net.minecraft.client.model.geom.ModelPart` | 5 | | Models |
| 36 | `net.minecraft.core.registries.BuiltInRegistries` | 12 | | Registries |
| 37 | `net.minecraft.core.BlockPos$MutableBlockPos` | 7 | | Mutable block pos |
| 38 | `net.minecraft.world.level.block.Block` | 9 | | Block base |
| 39 | `com.mojang.blaze3d.vertex.VertexConsumer` | 10 | | Vertex buffer |
| 40 | `net.minecraft.network.chat.Style` | 11 | | Text styles |
| 41 | `net.minecraft.client.gui.components.EditBox` | 7 | | Text input |
| 42 | `net.minecraft.core.Registry` | 9 | | Registry API |
| 43 | `net.minecraft.world.entity.LivingEntity` | 8 | | Living entities |
| 44 | `net.minecraft.client.Options` | 10 | | Client options |
| 45 | `net.minecraft.world.level.block.Blocks` | 9 | | Vanilla blocks |
| 46 | `net.minecraft.client.resources.language.I18n` | 5 | | Localization |
| 47 | `net.minecraft.world.level.block.entity.BlockEntity` | 8 | | Block entities |
| 48 | `net.minecraft.client.player.AbstractClientPlayer` | 7 | | Client player |
| 49 | `net.minecraft.world.phys.AABB` | 7 | | Bounding boxes |
| 50 | `net.minecraft.client.player.LocalPlayer` | 10 | | Local player |

## Mod-Level Coverage

| Mod | Category | Status | Test |
|-----|----------|--------|------|
| fabric-api | Library | | |
| sodium | Rendering/Performance | 🚫 | Patches vanilla renderer internals |
| iris | Rendering/Shaders | 🚫 | Patches vanilla renderer internals |
| lithium | Performance | 🚫 | Patches vanilla internals |
| modmenu | Utility | | |
| ferritecore | Performance | 🚫 | Patches vanilla internals |
| cloth-config | Library | | |
| entityculling | Rendering | ⚠️ | Needs GuiGraphicsExtractor |
| immediatelyfast | Rendering | ⚠️ | Needs GuiGraphicsExtractor |
| xaerominimap | Utility/Map | ⚠️ | Needs client rendering |
| xaeroworldmap | Utility/Map | ⚠️ | Needs client rendering |
| veinminer | Gameplay | | |
| appleskin | UI/HUD | | |
| notenoughanimations | Visual | ⚠️ | Needs model/animation APIs |
| architectury-api | Library | | |
| entity_texture_features | Visual | | |
| entity_model_features | Visual | | |
| sodium-extra | Rendering | 🚫 | Depends on Sodium |
| yet_another_config_lib | Library | | |
| fabric-language-kotlin | Library | | |

## Coverage Metrics

- **Symbols implemented**: 0 / 50 (0%)
- **Mods working**: 0 / 20 (0%)
- **Mods partial**: 0 / 20 (0%)
- **Mods unsupported**: 3 / 20 (15%)

*Updated after each Phase 2 milestone*