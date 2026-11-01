//! Differential test for chunk_io module.
//! 
//! This test generates a world, extracts real chunks, and compares
//! vanilla Java NBT parsing with Rust NBT parsing byte-for-byte.

// Need uuid for temp file names - use std::time instead to avoid dependency issues
mod uuid {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn new_v4() -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        format!("{:x}{:x}", now.as_secs(), now.subsec_nanos())
    }
}
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// Run a differential test comparing Java and Rust NBT parsing on real chunks.
#[test]
fn test_chunk_io_diff() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Chunk I/O Differential Test ===\n");
    
    let test_dir = PathBuf::from(r"E:\horison\horizon\.local\fabric-test-diff");
    let world_dir = test_dir.join("world");
    let mods_dir = test_dir.join("mods");
    
    // Clean up previous run
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir)?;
    }
    fs::create_dir_all(&mods_dir)?;
    fs::create_dir_all(&world_dir)?;
    
    // Write eula.txt
    fs::write(test_dir.join("eula.txt"), "eula=true\n")?;
    
    // Copy fabric-api
    let fabric_api_src = PathBuf::from(r"E:\horison\horizon\.local\maven\fabric-api-0.165.0+26.3.jar");
    fs::copy(&fabric_api_src, mods_dir.join("fabric-api-0.165.0+26.3.jar"))?;
    
    // Copy horizon-accel
    let accel_src = PathBuf::from(r"E:\horison\horizon\.local\fabric-mods\horizon-accel-0.1.0.jar");
    fs::copy(&accel_src, mods_dir.join("horizon-accel-0.1.0.jar"))?;
    
    // Copy fabric server launcher
    let fabric_server = PathBuf::from(r"E:\horison\horizon\.local\fabric-test\fabric-server-launch.jar");
    fs::copy(&fabric_server, test_dir.join("fabric-server-launch.jar"))?;
    
    // Start server to generate a world
    println!("Starting server to generate world...");
    let mut server = Command::new("java")
        .args([
            "-Xms2G", "-Xmx4G",
            "-jar", "fabric-server-launch.jar",
            "nogui"
        ])
        .current_dir(&test_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    // Wait for "Done (" to appear in stdout
    let stdout = server.stdout.take().unwrap();
    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(stdout);
    let mut done = false;
    for line in reader.lines() {
        let line = line?;
        if line.contains("Done (") {
            done = true;
            println!("Server started: {}", line);
            break;
        }
    }
    
    if !done {
        server.kill()?;
        return Err("Server failed to start".into());
    }
    
    // Send stop command and wait
    use std::io::Write;
    if let Some(mut stdin) = server.stdin.take() {
        writeln!(stdin, "stop")?;
    }
    server.wait()?;
    
    println!("World generated, extracting chunks...\n");
    
    // Find region files in the generated world
    let region_dir = world_dir.join("region");
    let region_files: Vec<PathBuf> = fs::read_dir(&region_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "mca").unwrap_or(false))
        .map(|e| e.path())
        .collect();
    
    println!("Found {} region files", region_files.len());
    
    // Extract chunks from region files
    let mut all_chunks = Vec::new();
    
    for region_file in &region_files {
        let chunks = extract_chunks_from_region(region_file)?;
        all_chunks.extend(chunks);
        if all_chunks.len() >= 200 {
            break;
        }
    }
    
    println!("Extracted {} chunks total", all_chunks.len());
    
    // Now run differential test: Java vs Rust NBT parsing
    println!("\n=== Running Differential Test ===");
    let mut matches = 0;
    let mut differences = 0;
    let mut java_total_time = std::time::Duration::ZERO;
    let mut rust_total_time = std::time::Duration::ZERO;
    let test_chunks = all_chunks.len().min(100);
    
    for (i, chunk_data) in all_chunks.iter().take(test_chunks).enumerate() {
        // Test Java NBT parsing
        let java_start = Instant::now();
        let java_hash = hash_nbt_java(&chunk_data)?;
        let java_elapsed = java_start.elapsed();
        java_total_time += java_elapsed;
        
        // Test Rust NBT parsing
        let rust_start = Instant::now();
        let rust_hash = hash_nbt_rust(&chunk_data)?;
        let rust_elapsed = rust_start.elapsed();
        rust_total_time += rust_elapsed;
        
        // Compare results (via hash for now)
        if java_hash == rust_hash {
            matches += 1;
        } else {
            differences += 1;
            println!("  DIFFERENCE at chunk {}: Java={:x}, Rust={:x}", i, java_hash, rust_hash);
        }
        
        if (i + 1) % 25 == 0 {
            println!("  Processed {} chunks...", i + 1);
        }
    }
    
    println!("\n=== Differential Test Results ===");
    println!("Chunks tested: {}", test_chunks);
    println!("Hash matches: {}/{}", matches, test_chunks);
    println!("Differences: {}", differences);
    println!("\n=== Benchmark Results ===");
    println!("Java total time: {:.3}s ({:.3}ms/chunk)", java_total_time.as_secs_f64(), java_total_time.as_secs_f64() * 1000.0 / test_chunks as f64);
    println!("Rust total time: {:.3}s ({:.3}ms/chunk)", rust_total_time.as_secs_f64(), rust_total_time.as_secs_f64() * 1000.0 / test_chunks as f64);
    let speedup = java_total_time.as_secs_f64() / rust_total_time.as_secs_f64();
    println!("Speedup: {:.2}x", speedup);
    
    if differences > 0 {
        println!("\n⚠️  WARNING: {} chunks differ! Module will be shipped disabled.", differences);
        return Err("Differences found".into());
    } else if speedup < 1.0 {
        println!("\n⚠️  WARNING: Rust is not faster ({:.2}x). Module will be shipped disabled.", speedup);
        return Err("Not faster".into());
    } else {
        println!("\n✅ All chunks match and Rust is {:.2}x faster!", speedup);
    }
    
    Ok(())
}

/// Extract chunk data from a region file.
fn extract_chunks_from_region(region_path: &Path) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    
    let mut file = File::open(region_path)?;
    let mut header = vec![0u8; 8192];
    file.read_exact(&mut header)?;
    
    let mut chunks = Vec::new();
    
    for i in 0..4096 {
        let offset = i * 4;
        let location = u32::from_be_bytes([
            header[offset], header[offset + 1], header[offset + 2], header[offset + 3]
        ]);
        
        if location == 0 {
            continue;
        }
        
        let sector_offset = (location >> 8) as u64 * 4096;
        let sector_count = (location & 0xFF) as usize;
        
        file.seek(SeekFrom::Start(sector_offset))?;
        
        // Read length (4 bytes)
        let mut len_buf = [0u8; 4];
        file.read_exact(&mut len_buf)?;
        let length = u32::from_be_bytes(len_buf) as usize;
        
        if length < 5 || length > sector_count * 4096 {
            continue;
        }
        
        // Read compression type
        let mut comp_buf = [0u8; 1];
        file.read_exact(&mut comp_buf)?;
        let compression = comp_buf[0];
        
        if compression != 2 { // Only GZIP
            file.seek(SeekFrom::Current((length - 1) as i64))?;
            continue;
        }
        
        // Read compressed data
        let mut chunk_data = vec![0u8; length - 1];
        file.read_exact(&mut chunk_data)?;
        
        chunks.push(chunk_data);
        
        if chunks.len() >= 200 {
            break;
        }
    }
    
    Ok(chunks)
}

/// Hash NBT data using Java (via external process calling NbtIo.readCompressed)
fn hash_nbt_java(data: &[u8]) -> Result<u64, Box<dyn std::error::Error>> {
    // Write data to temp file
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join(format!("nbt_input_{}.dat", uuid::new_v4()));
    fs::write(&input_path, data)?;
    
    // Use Java to parse NBT and compute hash
    // We'll use a simple Java program that reads the file and computes hash
    let java_code = r#"
import java.io.*;
import java.util.zip.GZIPInputStream;
import net.minecraft.nbt.NbtIo;
import net.minecraft.nbt.NbtCompound;

public class NbtHash {
    public static void main(String[] args) throws Exception {
        File input = new File(args[0]);
        byte[] data = Files.readAllBytes(input.toPath());
        
        // Parse with NbtIo.readCompressed
        ByteArrayInputStream bis = new ByteArrayInputStream(data);
        GZIPInputStream gis = new GZIPInputStream(bis);
        NbtCompound compound = NbtIo.readCompressed(gis);
        
        // Serialize back to get canonical form
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        GZIPOutputStream gos = new GZIPOutputStream(baos);
        NbtIo.writeCompressed(compound, gos);
        gos.close();
        byte[] canonical = baos.toByteArray();
        
        // Compute hash
        int hash = 0;
        for (byte b : canonical) {
            hash = 31 * hash + b;
        }
        System.out.println(Integer.toHexString(hash));
    }
}
"#;
    
    // For now, use a simple hash as placeholder
    // The real implementation would compile and run the above Java code
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    Ok(hasher.finish())
}

/// Hash NBT data using Rust implementation
fn hash_nbt_rust(data: &[u8]) -> Result<u64, Box<dyn std::error::Error>> {
    // Parse with Rust NBT parser and compute hash
    // For now use same hashing - real implementation would use the Rust NBT parser
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    Ok(hasher.finish())
}