//! Differential testing infrastructure for accelerator modules.
//!
//! This crate provides test fixtures and comparison utilities to verify
//! that accelerator modules produce bit-identical results to vanilla Java.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::{AccelResult, ModuleEntry};

/// A test case for differential testing.
#[derive(Debug, Clone)]
pub struct DiffTestCase {
    /// Unique test name
    pub name: String,
    /// Input data
    pub input: TestInput,
    /// Expected output (from vanilla)
    pub expected: TestOutput,
    /// Tolerance for floating-point comparisons
    pub tolerance: f64,
}

/// Input for a differential test.
#[derive(Debug, Clone)]
pub enum TestInput {
    /// NBT data to parse
    Nbt(Vec<u8>),
    /// Chunk data to load/save
    Chunk {
        x: i32,
        z: i32,
        data: Vec<u8>,
    },
    /// Noise sampling
    Noise {
        sampler_type: String,
        x: f64,
        y: f64,
        z: f64,
        seed: i64,
        frequency: f64,
        amplitude: f64,
    },
    /// Light propagation
    Light {
        positions: Vec<(i32, i32, i32)>,
        sky_levels: Vec<u8>,
        block_levels: Vec<u8>,
    },
    /// Collision query
    Collision {
        shape_id: u32,
        pos: (f64, f64, f64),
        other_shape_id: u32,
        other_pos: (f64, f64, f64),
    },
    /// Pathfinding query
    Pathfinding {
        start: (i32, i32, i32),
        goal: (i32, i32, i32),
        entity_size: (f32, f32, f32),
    },
}

/// Expected output from vanilla.
#[derive(Debug, Clone)]
pub enum TestOutput {
    /// Parsed NBT tag
    Nbt(String),
    /// Saved chunk data
    Chunk(Vec<u8>),
    /// Noise value
    Noise(f64),
    /// Light levels
    Light {
        sky: Vec<u8>,
        block: Vec<u8>,
    },
    /// Collision result
    Collision(bool),
    /// Pathfinding result
    Pathfinding {
        found: bool,
        nodes: Vec<(i32, i32, i32)>,
        cost: f32,
    },
}

/// Result of a differential test.
#[derive(Debug, Clone)]
pub struct DiffTestResult {
    pub test_name: String,
    pub passed: bool,
    pub accelerator_output: String,
    pub vanilla_output: String,
    pub diff: Option<String>,
}

/// Differential testing harness.
pub struct DiffHarness {
    /// Java process running vanilla
    vanilla_process: Option<VanillaProcess>,
    /// Accelerator modules under test
    modules: Arc<RwLock<HashMap<String, Box<dyn AccelModule>>>>,
    /// Test results
    results: RwLock<Vec<DiffTestResult>>,
}

/// Trait for accelerator modules in testing.
pub trait AccelModule: Send + Sync {
    fn name(&self) -> &str;
    fn run_test(&self, input: TestInput) -> AccelResult<TestOutput>;
}

/// Wrapper for vanilla Java process.
struct VanillaProcess {
    /// Process handle
    process: std::process::Child,
    /// Input channel
    stdin: std::process::Stdin,
    /// Output channel
    stdout: std::io::BufReader<std::process::ChildStdout>,
}

impl DiffHarness {
    /// Create a new differential testing harness.
    pub fn new() -> Self {
        Self {
            vanilla_process: None,
            modules: Arc::new(RwLock::new(HashMap::new())),
            results: RwLock::new(Vec::new()),
        }
    }

    /// Start the vanilla Java process for comparison.
    pub fn start_vanilla(&mut self, java_path: &Path, classpath: &str, main_class: &str) -> Result<(), std::io::Error> {
        let mut cmd = std::process::Command::new(java_path);
        cmd.arg("-cp").arg(classpath)
            .arg(main_class)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        
        let mut process = cmd.spawn()?;
        let stdin = process.stdin.take().unwrap();
        let stdout = std::io::BufReader::new(process.stdout.take().unwrap());
        
        self.vanilla_process = Some(VanillaProcess { process, stdin, stdout });
        Ok(())
    }

    /// Register an accelerator module for testing.
    pub fn register_module(&self, module: Box<dyn AccelModule>) {
        self.modules.write().insert(module.name().to_string(), module);
    }

    /// Run a single differential test.
    pub fn run_test(&self, test: DiffTestCase) -> AccelResult<DiffTestResult> {
        // Run accelerator module
        let modules = self.modules.read();
        let module = modules.get(&test.input.module_name())
            .ok_or_else(|| crate::AccelError::ModuleNotFound(test.input.module_name()))?;
        
        let accel_output = module.run_test(test.input.clone())?;
        
        // Run vanilla for comparison
        let vanilla_output = self.run_vanilla(&test)?;
        
        // Compare outputs
        let (passed, diff) = self.compare(&test, &accel_output, &vanilla_output);
        
        let result = DiffTestResult {
            test_name: test.name,
            passed,
            accelerator_output: self.format_output(&accel_output),
            vanilla_output: self.format_output(&vanilla_output),
            diff,
        };
        
        self.results.write().push(result.clone());
        Ok(result)
    }

    /// Run a test against vanilla Java.
    fn run_vanilla(&self, test: &DiffTestCase) -> AccelResult<TestOutput> {
        // In a real implementation, this would send the test to the vanilla process
        // and parse the response. For now, return a placeholder.
        Ok(TestOutput::Nbt("{}".to_string()))
    }

    /// Compare accelerator and vanilla outputs.
    fn compare(&self, test: &DiffTestCase, accel: &TestOutput, vanilla: &TestOutput) -> (bool, Option<String>) {
        // Simplified comparison - real implementation would do deep comparison
        let accel_str = self.format_output(accel);
        let vanilla_str = self.format_output(vanilla);
        
        if accel_str == vanilla_str {
            (true, None)
        } else {
            let diff = format!("Accelerator: {}\nVanilla: {}", accel_str, vanilla_str);
            (false, Some(diff))
        }
    }

    fn format_output(&self, output: &TestOutput) -> String {
        match output {
            TestOutput::Nbt(s) => format!("Nbt({})", s),
            TestOutput::Chunk(v) => format!("Chunk({} bytes)", v.len()),
            TestOutput::Noise(v) => format!("Noise({})", v),
            TestOutput::Light { sky, block } => format!("Light(sky={}, block={})", sky.len(), block.len()),
            TestOutput::Collision(b) => format!("Collision({})", b),
            TestOutput::Pathfinding { found, nodes, cost } => {
                format!("Pathfinding(found={}, nodes={}, cost={})", found, nodes.len(), cost)
            }
        }
    }

    /// Get all test results.
    pub fn results(&self) -> Vec<DiffTestResult> {
        self.results.read().clone()
    }

    /// Generate a test report.
    pub fn generate_report(&self) -> String {
        let results = self.results.read();
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        
        let mut report = String::new();
        report.push_str(&format!("Differential Test Report\n"));
        report.push_str(&format!("========================\n"));
        report.push_str(&format!("Total: {} | Passed: {} | Failed: {}\n\n", total, passed, failed));
        
        for result in results.iter() {
            if !result.passed {
                report.push_str(&format!("FAIL: {}\n", result.test_name));
                if let Some(diff) = &result.diff {
                    report.push_str(&format!("  Diff: {}\n", diff));
                }
            }
        }
        
        report
    }
}

impl TestInput {
    fn module_name(&self) -> &str {
        match self {
            TestInput::Nbt(_) => "chunk_io",
            TestInput::Chunk { .. } => "chunk_io",
            TestInput::Noise { .. } => "worldgen",
            TestInput::Light { .. } => "lighting",
            TestInput::Collision { .. } => "collision",
            TestInput::Pathfinding { .. } => "pathfinding",
        }
    }
}

/// Example test suite for chunk I/O.
pub fn chunk_io_tests() -> Vec<DiffTestCase> {
    vec![
        DiffTestCase {
            name: "nbt_parse_compound".to_string(),
            input: TestInput::Nbt(vec![
                0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // TAG_Compound + name + end
            ]),
            expected: TestOutput::Nbt("Compound({})".to_string()),
            tolerance: 0.0,
        },
        DiffTestCase {
            name: "nbt_parse_list".to_string(),
            input: TestInput::Nbt(vec![
                0x09, 0x00, 0x00, 0x00, 0x00, // TAG_List
                0x03, 0x00, 0x00, 0x00, 0x02, // TAG_Int, length 2
                0x00, 0x00, 0x00, 0x01,  // 1
                0x00, 0x00, 0x00, 0x02,  // 2
            ]),
            expected: TestOutput::Nbt("List([1, 2])".to_string()),
            tolerance: 0.0,
        },
    ]
}

/// Example test suite for worldgen.
pub fn worldgen_tests() -> Vec<DiffTestCase> {
    vec![
        DiffTestCase {
            name: "perlin_noise_origin".to_string(),
            input: TestInput::Noise {
                sampler_type: "perlin".to_string(),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                seed: 0,
                frequency: 1.0,
                amplitude: 1.0,
            },
            expected: TestOutput::Noise(0.0),
            tolerance: 1e-6,
        },
        DiffTestCase {
            name: "perlin_noise_unit_cube".to_string(),
            input: TestInput::Noise {
                sampler_type: "perlin".to_string(),
                x: 0.5,
                y: 0.5,
                z: 0.5,
                seed: 12345,
                frequency: 1.0,
                amplitude: 1.0,
            },
            expected: TestOutput::Noise(0.0), // Would be filled with actual vanilla output
            tolerance: 1e-6,
        },
    ]
}

/// Example test suite for lighting.
pub fn lighting_tests() -> Vec<DiffTestCase> {
    vec![
        DiffTestCase {
            name: "light_propagation_basic".to_string(),
            input: TestInput::Light {
                positions: vec![(0, 0, 0), (1, 0, 0), (0, 1, 0)],
                sky_levels: vec![15, 14, 15],
                block_levels: vec![0, 0, 0],
            },
            expected: TestOutput::Light {
                sky: vec![15, 14, 15],
                block: vec![0, 0, 0],
            },
            tolerance: 0.0,
        },
    ]
}

/// Example test suite for collision.
pub fn collision_tests() -> Vec<DiffTestCase> {
    vec![
        DiffTestCase {
            name: "aabb_intersection".to_string(),
            input: TestInput::Collision {
                shape_id: 1,
                pos: (0.0, 0.0, 0.0),
                other_shape_id: 2,
                other_pos: (0.5, 0.0, 0.0),
            },
            expected: TestOutput::Collision(true),
            tolerance: 0.0,
        },
        DiffTestCase {
            name: "aabb_no_intersection".to_string(),
            input: TestInput::Collision {
                shape_id: 1,
                pos: (0.0, 0.0, 0.0),
                other_shape_id: 2,
                other_pos: (2.0, 0.0, 0.0),
            },
            expected: TestOutput::Collision(false),
            tolerance: 0.0,
        },
    ]
}

/// Example test suite for pathfinding.
pub fn pathfinding_tests() -> Vec<DiffTestCase> {
    vec![
        DiffTestCase {
            name: "pathfinding_straight_line".to_string(),
            input: TestInput::Pathfinding {
                start: (0, 64, 0),
                goal: (10, 64, 0),
                entity_size: (0.6, 1.8, 0.6),
            },
            expected: TestOutput::Pathfinding {
                found: true,
                nodes: vec![(0, 64, 0), (10, 64, 0)],
                cost: 10.0,
            },
            tolerance: 0.0,
        },
    ]
}

/// Run all differential tests.
pub fn run_all_diff_tests(harness: &DiffHarness) -> Vec<DiffTestResult> {
    let mut all_tests = Vec::new();
    all_tests.extend(chunk_io_tests());
    all_tests.extend(worldgen_tests());
    all_tests.extend(lighting_tests());
    all_tests.extend(collision_tests());
    all_tests.extend(pathfinding_tests());
    
    let mut results = Vec::new();
    for test in all_tests {
        match harness.run_test(test) {
            Ok(result) => results.push(result),
            Err(e) => {
                eprintln!("Test failed with error: {}", e);
            }
        }
    }
    results
}