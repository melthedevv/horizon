package dev.horizonlab.accel;

import net.minecraft.nbt.NbtCompound;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.registry.RegistryWrapper;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Path;

/**
 * Java wrapper for the Chunk I/O native module.
 */
public class ChunkIoModule {
    private static final Logger LOGGER = LoggerFactory.getLogger("horizon-accel.ChunkIoModule");

    // Native methods
    private static native byte[] nativeReadChunk(String regionPath, int chunkX, int chunkZ);
    private static native boolean nativeWriteChunk(String regionPath, int chunkX, int chunkZ, byte[] data);
    private static native Chunk nativeReadChunkFromPath(String path, Object registries);
    private static native boolean nativeWriteChunkToPath(Object chunk, String path, Object registries);
    private static native NbtCompound nativeParseNbt(byte[] data);
    private static native byte[] nativeWriteNbt(NbtCompound compound);
    private static native int nativeGetPalettedValue(Object container, int index);
    private static native void nativeSetPalettedValue(Object container, int index, int value);

    /**
     * Read a chunk from a region file.
     */
    public byte[] readChunk(Path regionPath, int chunkX, int chunkZ) {
        try {
            return nativeReadChunk(regionPath.toString(), chunkX, chunkZ);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native readChunk not available: {}", e.getMessage());
            return null;
        } catch (Exception e) {
            LOGGER.warn("Native readChunk failed: {}", e.getMessage());
            return null;
        }
    }

    /**
     * Write a chunk to a region file.
     */
    public boolean writeChunk(Path regionPath, int chunkX, int chunkZ, byte[] data) {
        try {
            return nativeWriteChunk(regionPath.toString(), chunkX, chunkZ, data);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native writeChunk not available: {}", e.getMessage());
            return false;
        } catch (Exception e) {
            LOGGER.warn("Native writeChunk failed: {}", e.getMessage());
            return false;
        }
    }

    /**
     * Read a chunk from a path (for ChunkSerializer).
     */
    public Chunk readChunkFromPath(Path path, RegistryWrapper.WrapperLookup registries) {
        try {
            return nativeReadChunkFromPath(path.toString(), registries);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native readChunkFromPath not available: {}", e.getMessage());
            return null;
        } catch (Exception e) {
            LOGGER.warn("Native readChunkFromPath failed: {}", e.getMessage());
            return null;
        }
    }

    /**
     * Write a chunk to a path (for ChunkSerializer).
     */
    public boolean writeChunkToPath(Chunk chunk, Path path, RegistryWrapper.WrapperLookup registries) {
        try {
            return nativeWriteChunkToPath(chunk, path.toString(), registries);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native writeChunkToPath not available: {}", e.getMessage());
            return false;
        } catch (Exception e) {
            LOGGER.warn("Native writeChunkToPath failed: {}", e.getMessage());
            return false;
        }
    }

    /**
     * Parse NBT data.
     */
    public NbtCompound parseNbt(byte[] data) {
        try {
            return nativeParseNbt(data);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native parseNbt not available: {}", e.getMessage());
            return null;
        } catch (Exception e) {
            LOGGER.warn("Native parseNbt failed: {}", e.getMessage());
            return null;
        }
    }

    /**
     * Write NbtCompound to byte array.
     */
    public byte[] writeNbt(NbtCompound compound) {
        try {
            return nativeWriteNbt(compound);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native writeNbt not available: {}", e.getMessage());
            return null;
        } catch (Exception e) {
            LOGGER.warn("Native writeNbt failed: {}", e.getMessage());
            return null;
        }
    }

    /**
     * Get a value from a paletted container.
     */
    public int getPalettedValue(Object container, int index) {
        try {
            return nativeGetPalettedValue(container, index);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native getPalettedValue not available: {}", e.getMessage());
            return 0;
        } catch (Exception e) {
            LOGGER.warn("Native getPalettedValue failed: {}", e.getMessage());
            return 0;
        }
    }

    /**
     * Set a value in a paletted container.
     */
    public void setPalettedValue(Object container, int index, int value) {
        try {
            nativeSetPalettedValue(container, index, value);
        } catch (UnsatisfiedLinkError e) {
            LOGGER.warn("Native setPalettedValue not available: {}", e.getMessage());
        } catch (Exception e) {
            LOGGER.warn("Native setPalettedValue failed: {}", e.getMessage());
        }
    }
}