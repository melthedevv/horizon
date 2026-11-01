package dev.horizonlab.accel.mixin;

import dev.horizonlab.accel.HorizonAccelMod;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.world.chunk.serialization.ChunkSerializer;
import net.minecraft.nbt.NbtCompound;
import net.minecraft.registry.RegistryWrapper;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import java.io.IOException;
import java.nio.file.Path;

@Mixin(ChunkSerializer.class)
public abstract class ChunkSerializerMixin {
    private static final Logger LOGGER = LoggerFactory.getLogger("horizon-accel.mixin.ChunkSerializer");

    @Inject(
        method = "read(Ljava/nio/file/Path;Lnet/minecraft/registry/RegistryWrapper;)Lnet/minecraft/world/chunk/Chunk;",
        at = @At("HEAD"),
        cancellable = true
    )
    private static void onRead(Path path, RegistryWrapper.WrapperLookup registries, CallbackInfoReturnable<Chunk> cir) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            // Try to read via Rust
            Chunk chunk = HorizonAccelMod.getNativeChunkIo().readChunkFromPath(path, registries);
            if (chunk != null) {
                cir.setReturnValue(chunk);
                LOGGER.debug("Rust chunk read from path: {}", path);
            }
        } catch (Exception e) {
            LOGGER.warn("Rust chunk read from path failed, falling back to vanilla: {}", e.getMessage());
            HorizonAccelMod.reportNativeError("ChunkSerializer.read", e.getMessage());
        }
    }

    @Inject(
        method = "write(Lnet/minecraft/world/chunk/Chunk;Ljava/nio/file/Path;Lnet/minecraft/registry/RegistryWrapper;)V",
        at = @At("HEAD"),
        cancellable = true
    )
    private static void onWrite(Chunk chunk, Path path, RegistryWrapper.WrapperLookup registries, CallbackInfo ci) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            boolean success = HorizonAccelMod.getNativeChunkIo().writeChunkToPath(chunk, path, registries);
            if (success) {
                ci.cancel();
                LOGGER.debug("Rust chunk write to path: {}", path);
            }
        } catch (Exception e) {
            LOGGER.warn("Rust chunk write to path failed, falling back to vanilla: {}", e.getMessage());
            HorizonAccelMod.reportNativeError("ChunkSerializer.write", e.getMessage());
        }
    }
}