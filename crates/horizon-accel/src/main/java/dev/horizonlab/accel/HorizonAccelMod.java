package dev.horizonlab.accel;

import net.fabricmc.api.ModInitializer;
import net.fabricmc.loader.api.FabricLoader;
import net.fabricmc.loader.api.ModContainer;
import net.fabricmc.loader.api.metadata.ModMetadata;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.InputStream;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.*;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;
import java.util.stream.Collectors;

/**
 * Horizon Fabric Accelerator mod.
 * Loads the native Rust library via JNI and coordinates with the Rust side
 * for conflict scanning, module management, and fallback handling.
 */
public class HorizonAccelMod implements ModInitializer {
    private static final Logger LOGGER = LoggerFactory.getLogger("horizon-accel");

    // Protocol version must match Rust side
    public static final int PROTOCOL_VERSION = 1;

    // Native library name (platform-specific)
    private static final String NATIVE_LIB_NAME = System.mapLibraryName("horizon_accel");

    // Manifest resource path
    private static final String MANIFEST_RESOURCE = "accel-manifest.json";

    // Module states reported by Rust
    public enum ModuleState {
        ACTIVE,
        DISABLED,
        FALLBACK_CONFLICT,
        FALLBACK_ERROR,
        FALLBACK_VERSION,
        NOT_COMPILED
    }

    // Loaded native library handle
    private static boolean nativeLoaded = false;

    // Native method declarations for Chunk I/O module (implemented in ChunkIoModule.java)
    // private static native byte[] nativeReadChunk(String regionPath, int chunkX, int chunkZ);
    // private static native boolean nativeWriteChunk(String regionPath, int chunkX, int chunkZ, byte[] data);
    // private static native Chunk nativeReadChunkFromPath(String path, Object registries);
    // private static native boolean nativeWriteChunkToPath(Object chunk, String path, Object registries);
    // private static native NbtCompound nativeParseNbt(byte[] data);
    // private static native byte[] nativeWriteNbt(NbtCompound compound);
    // private static native int nativeGetPalettedValue(Object container, int index);
    // private static native void nativeSetPalettedValue(Object container, int index, int value);

    // Chunk I/O module instance (set at runtime via JNI)
    private static Object chunkIoModule = null;

    public static boolean isNativeLoaded() {
        return nativeLoaded;
    }

    public static Object getNativeChunkIo() {
        return chunkIoModule;
    }

    public static void setNativeChunkIo(Object module) {
        chunkIoModule = module;
    }

    public static void reportNativeError(String method, String error) {
        AccelNative.reportModuleError("chunk_io", method + ": " + error);
    }

    @Override
    public void onInitialize() {
        LOGGER.info("Initializing Horizon Fabric Accelerator v{}", getModVersion());

        // 1. Load native library
        if (!loadNativeLibrary()) {
            LOGGER.error("Failed to load native library; accelerator disabled");
            return;
        }

        // 2. Read manifest
        String manifestJson = readManifest();
        if (manifestJson == null) {
            LOGGER.error("Failed to read manifest; accelerator disabled");
            return;
        }

        // 3. Perform handshake with Rust side
        int handshakeResult = AccelNative.initialize(PROTOCOL_VERSION);
        if (handshakeResult < 0) {
            LOGGER.error("Handshake failed with code {}", handshakeResult);
            return;
        }
        LOGGER.info("Handshake successful: protocol v{}", handshakeResult);

        // 4. Read config file for disabled modules
        Set<String> disabledModules = readConfigDisabledModules();

        // 5. Register modules with Rust side (pass disabled modules)
        int result = AccelNative.registerModules(manifestJson, disabledModulesToJson(disabledModules));
        if (result < 0) {
            LOGGER.error("Module registration failed with code {}", result);
            return;
        }

        // 6. Mark native as loaded before conflict scan
        nativeLoaded = true;

        // 7. Trigger initial conflict scan
        scanConflicts();

        LOGGER.info("Horizon Fabric Accelerator initialized successfully");
    }

    /**
     * Load the native Rust library.
     */
    private boolean loadNativeLibrary() {
        if (nativeLoaded) {
            return true;
        }

        try {
            // Try to find the mod JAR first
            Path modJar = findModJar();
            if (modJar == null) {
                LOGGER.warn("Could not find horizon-accel mod JAR; trying system library path");
            } else {
                // Try to extract native library from mod JAR
                try (JarFile jar = new JarFile(modJar.toFile())) {
                    String nativePath = findNativeLibInJar(jar);
                    if (nativePath != null) {
                        Path tempDir = Files.createTempDirectory("horizon-accel-native");
                        Path tempLib = tempDir.resolve(NATIVE_LIB_NAME);
                        try (InputStream is = jar.getInputStream(jar.getEntry(nativePath))) {
                            Files.copy(is, tempLib, StandardCopyOption.REPLACE_EXISTING);
                        }
                        System.load(tempLib.toAbsolutePath().toString());
                        LOGGER.info("Loaded native library from mod JAR: {}", tempLib);
                        return true;
                    }
                } catch (IOException e) {
                    LOGGER.warn("Could not extract native library from JAR: {}", e.getMessage());
                }
            }

            // Fallback: try to load from system library path
            try {
                System.loadLibrary("horizon_accel");
                LOGGER.info("Loaded native library from system library path");
                return true;
            } catch (UnsatisfiedLinkError e) {
                LOGGER.error("Failed to load native library: {}", e.getMessage());
                return false;
            }
        } catch (Exception e) {
            LOGGER.error("Unexpected error loading native library", e);
            return false;
        }
    }

    private Path findModJar() {
        // Get mod container and extract path
        Optional<ModContainer> container = FabricLoader.getInstance().getModContainer("horizon-accel");
        if (container.isPresent()) {
            Path path = container.get().getPath("modid");
            if (path != null && path.toString().endsWith(".jar")) {
                return path;
            }
        }

        // Fallback: search mods folder
        Path modsDir = FabricLoader.getInstance().getGameDir().resolve("mods");
        try (var stream = Files.list(modsDir)) {
            return stream
                .filter(p -> p.getFileName().toString().startsWith("horizon-accel-") && p.toString().endsWith(".jar"))
                .findFirst()
                .orElse(null);
        } catch (IOException e) {
            LOGGER.warn("Could not scan mods directory", e);
            return null;
        }
    }

    /**
     * Find the native library entry in the JAR.
     */
    private String findNativeLibInJar(JarFile jar) {
        // Try platform-specific location first (META-INF/native/<os>/<arch>/)
        String os = System.getProperty("os.name").toLowerCase();
        String arch = System.getProperty("os.arch").toLowerCase();
        String osName;
        if (os.contains("win")) {
            osName = "windows";
        } else if (os.contains("mac") || os.contains("darwin")) {
            osName = "macos";
        } else {
            osName = "linux";
        }
        String archName;
        if (arch.equals("amd64") || arch.equals("x86_64")) {
            archName = "x86_64";
        } else if (arch.equals("aarch64") || arch.equals("arm64")) {
            archName = "aarch64";
        } else {
            archName = arch;
        }

        // Try platform-specific location first
        String[] possiblePaths = {
            "META-INF/native/" + osName + "/" + archName + "/" + NATIVE_LIB_NAME,
            "META-INF/native/" + NATIVE_LIB_NAME,
            "native/" + NATIVE_LIB_NAME,
            NATIVE_LIB_NAME
        };

        for (String path : possiblePaths) {
            if (jar.getEntry(path) != null) {
                return path;
            }
        }
        return null;
    }

    /**
     * Read the manifest JSON from the mod JAR resources.
     */
    private String readManifest() {
        try (InputStream is = getClass().getClassLoader().getResourceAsStream("accel-manifest.json")) {
            if (is == null) {
                LOGGER.error("Manifest resource {} not found in classpath", "accel-manifest.json");
                return null;
            }
            return new String(is.readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
        } catch (IOException e) {
            LOGGER.error("Failed to read manifest", e);
            return null;
        }
    }

    /**
     * Read disabled modules from the instance config file.
     * Config file is at <instance_dir>/config/horizon-accel.json
     */
    private Set<String> readConfigDisabledModules() {
        Set<String> disabled = new HashSet<>();
        try {
            // Get the instance directory from FabricLoader
            Path gameDir = FabricLoader.getInstance().getGameDir();
            Path configFile = gameDir.resolve("config").resolve("horizon-accel.json");
            
            if (Files.exists(configFile)) {
                String content = Files.readString(configFile, java.nio.charset.StandardCharsets.UTF_8);
                disabled.addAll(parseDisabledModulesFromJson(content));
            }
        } catch (Exception e) {
            LOGGER.warn("Failed to read accelerator config: {}", e.getMessage());
        }
        return disabled;
    }

    /**
     * Parse disabled modules from JSON string using simple string parsing.
     * Expected format: {"modules": {"chunk_io": false, "worldgen": true}}
     */
    private Set<String> parseDisabledModulesFromJson(String json) {
        Set<String> disabled = new HashSet<>();
        try {
            // Find the "modules" object
            int modulesIdx = json.indexOf("\"modules\"");
            if (modulesIdx >= 0) {
                int braceIdx = json.indexOf('{', modulesIdx);
                if (braceIdx >= 0) {
                    int endIdx = findMatchingBrace(json, braceIdx);
                    if (endIdx > braceIdx) {
                        String modulesJson = json.substring(braceIdx + 1, endIdx);
                        // Parse key-value pairs
                        String[] pairs = modulesJson.split(",");
                        for (String pair : pairs) {
                            String[] kv = pair.split(":", 2);
                            if (kv.length == 2) {
                                String key = kv[0].trim().replace("\"", "").trim();
                                String value = kv[1].trim().toLowerCase();
                                if ("false".equals(value)) {
                                    disabled.add(key);
                                    LOGGER.info("Module '{}' disabled via config", key);
                                }
                            }
                        }
                    }
                }
            }
        } catch (Exception e) {
            LOGGER.warn("Failed to parse modules from JSON: {}", e.getMessage());
        }
        return disabled;
    }

    private int findMatchingBrace(String s, int startIdx) {
        int depth = 0;
        for (int i = startIdx; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '{') depth++;
            else if (c == '}') {
                depth--;
                if (depth == 0) return i;
            }
        }
        return -1;
    }

    /**
     * Convert disabled modules set to JSON array string.
     */
    private String disabledModulesToJson(Set<String> disabledModules) {
        StringBuilder sb = new StringBuilder();
        sb.append("[");
        boolean first = true;
        for (String module : disabledModules) {
            if (!first) sb.append(",");
            sb.append("\"").append(module).append("\"");
            first = false;
        }
        sb.append("]");
        return sb.toString();
    }

    /**
     * Get mod version from metadata.
     */
    private String getModVersion() {
        return FabricLoader.getInstance().getModContainer("horizon-accel")
            .map(c -> c.getMetadata().getVersion().getFriendlyString())
            .orElse("unknown");
    }

    /**
     * Decode handshake error code.
     */
    private String decodeHandshakeError(int code) {
        return switch (code) {
            case -1 -> "VERSION_TOO_OLD";
            case -2 -> "VERSION_TOO_NEW";
            case -3 -> "MISSING_SYMBOLS";
            case -4 -> "INIT_FAILED";
            default -> "UNKNOWN_ERROR_" + code;
        };
    }

    /**
     * Trigger a conflict scan with currently installed mods.
     * Called by the launcher after all mods are loaded, or on first server tick.
     */
    public static void scanConflicts() {
        if (!nativeLoaded) {
            LOGGER.warn("scanConflicts called but native library not loaded");
            return;
        }

        try {
            // Collect mod mixin data
            List<ModMixinData> modsData = collectModMixinData();
            String modsJson = toJson(modsData);

            int result = AccelNative.scanConflicts(modsJson);
            if (result < 0) {
                LOGGER.error("Conflict scan failed with code {}", result);
                return;
            }

            // Retrieve and log conflict report
            String reportJson = AccelNative.getConflictReport();
            LOGGER.info("Conflict scan complete: {}", reportJson);

        } catch (Exception e) {
            LOGGER.error("Error during conflict scan", e);
        }
    }

    /**
     * Collect mixin targets from all installed Fabric mods.
     */
    private static List<ModMixinData> collectModMixinData() {
        List<ModMixinData> result = new ArrayList<>();

        for (ModContainer modContainer : FabricLoader.getInstance().getAllMods()) {
            String modId = modContainer.getMetadata().getId();
            if (modId.equals("horizon-accel") || modId.equals("fabricloader") || modId.equals("minecraft")) {
                continue; // Skip self and core
            }

            ModMixinData data = new ModMixinData();
            data.modId = modContainer.getMetadata().getId();
            data.version = modContainer.getMetadata().getVersion().getFriendlyString();

            // Extract mixin targets from mod JAR
            Optional<ModContainer> modContainerOpt = FabricLoader.getInstance().getModContainer(modId);
            Path modJar = modContainerOpt
                .map(c -> {
                    Path path = c.getPath("modid");
                    return (path != null && path.toString().endsWith(".jar")) ? path : null;
                })
                .filter(p -> p != null && Files.exists(p))
                .orElse(null);

            if (modJar != null && Files.exists(modJar)) {
                extractMixinTargets(modJar, data);
            }

            result.add(data);
        }

        return result;
    }

    /**
     * Extract mixin targets from a mod JAR.
     */
    private static void extractMixinTargets(Path jarPath, ModMixinData data) {
        try (JarFile jar = new JarFile(jarPath.toFile())) {
            // Find mixin config files
            for (JarEntry entry : Collections.list(jar.entries())) {
                String name = entry.getName();
                if (name.endsWith(".json") && (name.contains("mixin") || name.contains("mixins."))) {
                    try (InputStream is = jar.getInputStream(entry)) {
                        String json = new String(is.readAllBytes(), java.nio.charset.StandardCharsets.UTF_8);
                        parseMixinConfig(json, data);
                    } catch (Exception e) {
                        // Ignore parse errors
                    }
                }
            }
        } catch (IOException e) {
            // Ignore
        }
    }

    /**
     * Parse a mixin config JSON and extract target classes.
     */
    private static void parseMixinConfig(String json, ModMixinData data) {
        try {
            // Simple JSON parsing - extract class names from targets and mixin lists
            data.mixinTargets.addAll(extractClassNamesFromJson(json, "targets"));
            data.mixinTargets.addAll(extractClassNamesFromJson(json, "mixins"));
            data.mixinTargets.addAll(extractClassNamesFromJson(json, "client"));
            data.mixinTargets.addAll(extractClassNamesFromJson(json, "server"));
        } catch (Exception e) {
            // Ignore
        }
    }

    private static List<String> extractClassNamesFromJson(String json, String key) {
        List<String> result = new ArrayList<>();
        // Very simple extraction - real implementation would use Jackson/Gson
        int idx = json.indexOf("\"" + key + "\"");
        if (idx >= 0) {
            idx = json.indexOf('[', idx);
            if (idx >= 0) {
                int end = json.indexOf(']', idx);
                if (end > idx) {
                    String array = json.substring(idx + 1, end);
                    for (String part : array.split(",")) {
                        String cls = part.trim().replace("\"", "").trim();
                        if (!cls.isEmpty() && cls.contains(".")) {
                            result.add(cls);
                        }
                    }
                }
            }
        }
        return result;
    }

    /**
     * Convert object to JSON string.
     */
    private static String toJson(Object obj) {
        if (obj instanceof List<?>) {
            List<?> list = (List<?>) obj;
            StringBuilder sb = new StringBuilder();
            sb.append("[");
            for (int i = 0; i < list.size(); i++) {
                if (i > 0) sb.append(",");
                Object item = list.get(i);
                if (item instanceof ModMixinData) {
                    ModMixinData data = (ModMixinData) item;
                    sb.append("{");
                    sb.append("\"mod_id\":\"").append(escapeJson(data.modId)).append("\",");
                    sb.append("\"version\":\"").append(escapeJson(data.version)).append("\",");
                    sb.append("\"mixin_targets\":[");
                    for (int j = 0; j < data.mixinTargets.size(); j++) {
                        if (j > 0) sb.append(",");
                        sb.append("\"").append(escapeJson(data.mixinTargets.get(j))).append("\"");
                    }
                    sb.append("]");
                    sb.append("}");
                }
            }
            sb.append("]");
            return sb.toString();
        }
        return "{}";
    }

    private static String escapeJson(String s) {
        return s.replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\t", "\\t");
    }

    // ============================================================
    // Data classes
    // ============================================================

    /**
     * ModMixinData - holds mixin configuration for a mod.
     */
    public static class ModMixinData {
        public String modId;
        public String version;
        public List<String> mixinTargets = new ArrayList<>();
    }
}