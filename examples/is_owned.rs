/// Checks whether a devices is in use by either a set of partitions or a partition
/// type indicated by the superblock.
use std::{
    env,
    error::Error,
    fmt::{self, Display},
    path::Path,
};

use libblkid_rs::BlkidProbe;

#[derive(Debug)]
struct ExampleError(String);

impl ExampleError {
    fn new<D>(d: D) -> Self
    where
        D: Display,
    {
        ExampleError(d.to_string())
    }
}

impl Display for ExampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ExampleError {}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or_else(|| ExampleError::new("Path of device to check required as argument"))?;

    let mut probe = BlkidProbe::new_from_filename(Path::new(&path))?;
    probe.enable_superblocks(true)?;
    probe.enable_partitions(true)?;
    probe.do_safeprobe()?;

    let partitions = probe
        .get_partitions()
        .and_then(|mut list| list.number_of_partitions());
    let detected_use = probe.lookup_value("TYPE");

    if partitions.as_ref().map(|num| *num > 0).unwrap_or(false) || detected_use.is_ok() {
        println!("In use");
        if let Ok(num) = partitions {
            println!("{} partitions found on block device", num);
        }
        if let Ok(ty) = detected_use {
            println!("Device determined to be of type {}", ty);
        }
    } else {
        println!("Free");
    }

    Ok(())
}
