use std::path::Path;
use std::process::Command;

fn main() {
  let xml = Path::new("src/cboe_us_eq_pitch.xml");
  let codegen = Path::new("src/pitch_codegen.py");
  let out = Path::new("src/pitch.rs");

  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/cboe_us_eq_pitch.xml");

  println!("cargo:warning=Running codegen for Cboe US Equities PITCH");
  let output = Command::new("python3")
      .arg(codegen)
      .arg(xml)
      .output()
      .expect("Failed to execute pitch codegen");

  if !output.status.success() {
      panic!(
          "Codegen failed with status: {}. Output: {}",
          output.status,
          String::from_utf8_lossy(&output.stderr)
      );
  }
  let generated_code = String::from_utf8(output.stdout)
      .expect("Failed to convert output to string");
  std::fs::write(out, generated_code).expect("Failed to write generated code to file");

  println!("cargo:warning=Codegen completed successfully, output written to {}", out.display());
}
