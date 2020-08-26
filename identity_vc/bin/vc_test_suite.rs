use anyhow::Result;
use identity_vc::prelude::*;
use serde_json::{from_reader, to_string};
use std::{env::args, fs::File, path::Path};

fn main() -> Result<()> {
  let args: Vec<String> = args().collect();

  match args[1].as_str() {
    "test-credential" => {
      let path: &Path = Path::new(&args[2]);
      let file: File = File::open(path)?;
      let data: VerifiableCredential = from_reader(file)?;

      println!("{}", to_string(&data)?);
    }
    "test-presentation" => {
      let path: &Path = Path::new(&args[2]);
      let file: File = File::open(path)?;
      let data: VerifiablePresentation = from_reader(file)?;

      println!("{}", to_string(&data)?);
    }
    test => {
      panic!("Unknown Test: {:?}", test);
    }
  }

  Ok(())
}
