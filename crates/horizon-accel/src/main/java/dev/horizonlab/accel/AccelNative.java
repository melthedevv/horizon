package dev.horizonlab.accel;

/**
 * JNI native method declarations for the Horizon Accelerator.
 * These correspond to the `Java_dev_horizonlab_accel_AccelNative_*` functions
 * in the Rust library.
 */
public final class AccelNative {
    private AccelNative() {}

    // Protocol version (must match Rust)
    public static final int PROTOCOL_VERSION = 1;

    /**
     * Initialize the accelerator library and perform handshake.
     *
     * @param javaProtocolVersion The protocol version the Java side expects.
     * @return The agreed protocol version on success, or a negative error code:
     *         -1 = version too old, -2 = version too new, -3 = missing symbols, -4 = init failed.
     */
    public static native int initialize(int javaProtocolVersion);

    /**
     * Register accelerator modules from the manifest JSON.
     *
     * @param manifestJson The accel-manifest.json content as a string.
     * @param disabledModulesJson JSON array of disabled module IDs (e.g., ["chunk_io", "worldgen"]).
     * @return Number of modules registered on success, or negative error code.
     */
    public static native int registerModules(String manifestJson, String disabledModulesJson);

    /**
     * Scan installed mods for conflicts with accelerator modules.
     *
     * @param modsDataJson JSON array of ModMixinData objects.
     * @return 0 on success, negative on error.
     */
    public static native int scanConflicts(String modsDataJson);

    /**
     * Get the conflict report from the last scan.
     *
     * @return JSON string of ConflictReport, or "{}" if none.
     */
    public static native String getConflictReport();

    /**
     * Check if a module is currently active (not in fallback).
     *
     * @param moduleName The module ID.
     * @return true if active, false if in fallback or not found.
     */
    public static native boolean isModuleActive(String moduleName);

    /**
     * Report an error from a module's Rust code, triggering fallback.
     *
     * @param moduleName The module ID.
     * @param errorMsg The error message.
     */
    public static native void reportModuleError(String moduleName, String errorMsg);

    /**
     * Get the state of all modules as JSON.
     *
     * @return JSON object mapping module IDs to their state info.
     */
    public static native String getModuleStates();
}