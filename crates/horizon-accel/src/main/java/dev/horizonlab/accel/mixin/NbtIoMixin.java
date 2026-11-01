package dev.horizonlab.accel.mixin;

import dev.horizonlab.accel.HorizonAccelMod;
import net.minecraft.nbt.NbtCompound;
import net.minecraft.nbt.NbtIo;
import net.minecraft.nbt.NbtElement;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import java.io.DataInput;
import java.io.DataOutput;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.zip.GZIPInputStream;
import java.util.zip.GZIPOutputStream;

@Mixin(NbtIo.class)
public abstract class NbtIoMixin {
    private static final Logger LOGGER = LoggerFactory.getLogger("horizon-accel.mixin.NbtIo");

    @Inject(
        method = "readCompressed(Ljava/io/InputStream;)Lnet/minecraft/nbt/NbtCompound;",
        at = @At("HEAD"),
        cancellable = true
    )
    private static void onReadCompressed(InputStream inputStream, CallbackInfoReturnable<NbtCompound> cir) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            // Read all data
            byte[] data = inputStream.readAllBytes();
            
            // Try Rust NBT parsing
            NbtCompound compound = HorizonAccelMod.getNativeChunkIo().parseNbt(data);
            if (compound != null) {
                cir.setReturnValue(compound);
                LOGGER.debug("Rust NBT read compressed: {} bytes", data.length);
            }
        } catch (Exception e) {
            LOGGER.warn("Rust NBT read failed, falling back to vanilla: {}", e.getMessage());
            HorizonAccelMod.reportNativeError("NbtIo.readCompressed", e.getMessage());
        }
    }

    @Inject(
        method = "writeCompressed(Lnet/minecraft/nbt/NbtCompound;Ljava/io/OutputStream;)V",
        at = @At("HEAD"),
        cancellable = true
    )
    private static void onWriteCompressed(NbtCompound compound, OutputStream outputStream, CallbackInfo ci) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            byte[] data = HorizonAccelMod.getNativeChunkIo().writeNbt(compound);
            if (data != null) {
                outputStream.write(data);
                ci.cancel();
                LOGGER.debug("Rust NBT write compressed: {} bytes", data.length);
            }
        } catch (Exception e) {
            LOGGER.warn("Rust NBT write failed, falling back to vanilla: {}", e.getMessage());
            HorizonAccelMod.reportNativeError("NbtIo.writeCompressed", e.getMessage());
        }
    }

    @Inject(
        method = "read(Ljava/io/DataInput;)Lnet/minecraft/nbt/NbtElement;",
        at = @At("HEAD"),
        cancellable = true
    )
    private static void onRead(DataInput input, CallbackInfoReturnable<NbtElement> cir) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            // For uncompressed NBT, we'd need to read the full stream
            // This is more complex - skip for now and let vanilla handle it
        } catch (Exception e) {
            LOGGER.warn("Rust NBT read failed: {}", e.getMessage());
        }
    }

    @Inject(
        method = "write(Lnet/minecraft/nbt/NbtElement;Ljava/io/DataOutput;)V",
        at = @At("HEAD"),
        cancellable = true
    )
    private static void onWrite(NbtElement element, DataOutput output, CallbackInfo ci) {
        if (!HorizonAccelMod.isNativeLoaded()) {
            return;
        }

        try {
            // Similar complexity for uncompressed write
        } catch (Exception e) {
            LOGGER.warn("Rust NBT write failed: {}", e.getMessage());
        }
    }
}