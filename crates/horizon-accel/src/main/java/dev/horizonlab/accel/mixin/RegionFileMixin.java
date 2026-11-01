package dev.horizonlab.accel.mixin;

import dev.horizonlab.accel.HorizonAccelMod;
import net.minecraft.world.chunk.storage.RegionFile;
import net.minecraft.util.math.ChunkPos;
import net.minecraft.nbt.NbtIo;
import net.minecraft.nbt.NbtCompound;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import java.io.IOException;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.zip.GZIPInputStream;
import java.util.zip.GZIPOutputStream;

@Mixin(RegionFile.class)
public abstract class RegionFileMixin {
    private static final Logger LOGGER = LoggerFactory.getLogger("horizon-accel.mixin.RegionFile");

    @Inject(
        method = "read(Lnet/minecraft/util/math/ChunkPos;)Lnet/minecraft/nbt/NbtCompound;",
        at = @At("HEAD"),
        cancellable = true
    )
    private void onRead(ChunkPos chunkPos, CallbackInfoReturnable<NbtCompound> cir) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            RegionFile regionFile = (RegionFile) (Object) this;
            File regionFileFile = getRegionFileField(regionFile);
            if (regionFileFile == null) {
                return;
            }

            int chunkX = chunkPos.x;
            int chunkZ = chunkPos.z;

            // Try to read via Rust
            byte[] chunkData = HorizonAccelMod.getNativeChunkIo().readChunk(regionFileFile.toPath(), chunkX, chunkZ);
            if (chunkData != null) {
                // Parse NBT from chunk data
                NbtCompound compound = parseChunkData(chunkData);
                if (compound != null) {
                    cir.setReturnValue(compound);
                    LOGGER.debug("Rust chunk read: {} {}", chunkX, chunkZ);
                    return;
                }
            }
        } catch (Exception e) {
            LOGGER.warn("Rust chunk read failed, falling back to vanilla: {}", e.getMessage());
            HorizonAccelMod.reportNativeError("RegionFile.read", e.getMessage());
        }
    }

    @Inject(
        method = "write(Lnet/minecraft/util/math/ChunkPos;Lnet/minecraft/nbt/NbtCompound;)Z",
        at = @At("HEAD"),
        cancellable = true
    )
    private void onWrite(ChunkPos chunkPos, NbtCompound compound, CallbackInfoReturnable<Boolean> cir) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            RegionFile regionFile = (RegionFile) (Object) this;
            File regionFileFile = getRegionFileField(regionFile);
            if (regionFileFile == null) {
                return;
            }

            int chunkX = chunkPos.x;
            int chunkZ = chunkPos.z;

            // Serialize NBT to bytes
            byte[] chunkData = serializeChunkData(compound);
            if (chunkData != null) {
                boolean success = HorizonAccelMod.getNativeChunkIo().writeChunk(regionFileFile.toPath(), chunkX, chunkZ, chunkData);
                if (success) {
                    cir.setReturnValue(true);
                    LOGGER.debug("Rust chunk write: {} {}", chunkX, chunkZ);
                    return;
                }
            }
        } catch (Exception e) {
            LOGGER.warn("Rust chunk write failed, falling back to vanilla: {}", e.getMessage());
            HorizonAccelMod.reportNativeError("RegionFile.write", e.getMessage());
        }
    }

    private File getRegionFileField(RegionFile regionFile) {
        try {
            java.lang.reflect.Field field = RegionFile.class.getDeclaredField("file");
            field.setAccessible(true);
            return (File) field.get(regionFile);
        } catch (Exception e) {
            return null;
        }
    }

    private NbtCompound parseChunkData(byte[] data) {
        try {
            // Data format: length (4 bytes, big-endian) + compression type (1 byte) + compressed data
            if (data.length < 5) return null;
            
            int length = ((data[0] & 0xFF) << 24) | ((data[1] & 0xFF) << 16) | ((data[2] & 0xFF) << 8) | (data[3] & 0xFF);
            byte compressionType = data[4];
            
            if (compressionType != 2) return null; // Only GZIP supported
            
            byte[] compressed = new byte[length - 1];
            System.arraycopy(data, 5, compressed, 0, length - 1);
            
            try (GZIPInputStream gis = new GZIPInputStream(new java.io.ByteArrayInputStream(compressed))) {
                return NbtIo.read(gis);
            }
        } catch (Exception e) {
            LOGGER.warn("Failed to parse chunk data: {}", e.getMessage());
        }
        return null;
    }

    private byte[] serializeChunkData(NbtCompound compound) {
        try {
            java.io.ByteArrayOutputStream baos = new java.io.ByteArrayOutputStream();
            try (GZIPOutputStream gos = new GZIPOutputStream(baos)) {
                NbtIo.write(compound, gos);
            }
            byte[] compressed = baos.toByteArray();
            
            // Prepend length and compression type
            byte[] result = new byte[5 + compressed.length];
            int len = compressed.length + 1;
            result[0] = (byte) (len >> 24);
            result[1] = (byte) (len >> 16);
            result[2] = (byte) (len >> 8);
            result[3] = (byte) len;
            result[4] = 2; // GZIP compression
            System.arraycopy(compressed, 0, result, 5, compressed.length);
            
            return result;
        } catch (Exception e) {
            LOGGER.warn("Failed to serialize chunk data: {}", e.getMessage());
        }
        return null;
    }
}