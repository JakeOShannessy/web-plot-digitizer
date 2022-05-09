use serde::{Serialize, Deserialize};
use std::fs::File;
use data_vector::{DataVector, Point};
use std::path::{PathBuf, Path};
use tar::Archive;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Wpd {
    pub version: (u32,u32),
    #[serde(rename = "axesColl")]
    pub axes: Vec<Axes>,
    #[serde(rename = "datasetColl")]
    pub datasets: Vec<DataSet>,
}

impl Wpd {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let data: Wpd = serde_json::from_reader(file)?;
        Ok(data)
    }

    pub fn from_tar_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let mut archive = Archive::new(file);
        // At the root there should be an info.json file that tells us about the
        // file.
        let mut entries = archive.entries()?;
        let info: Info = {
            let info_entry = loop {
                if let Some(entry) = entries.next() {
                    let entry = entry?;
                    if entry.path()?.file_name().unwrap() == &PathBuf::from("info.json") {
                        break entry;
                    }
                } else {
                    panic!("could not find info entry")
                }
            };
            // let mut s = String::new();
            // info_entry.read_to_string(&mut s).unwrap();
            serde_json::from_reader(info_entry).unwrap()
        };

        let wpd: Wpd = {
            let json_entry = loop {
                if let Some(entry) = entries.next() {
                    let entry = entry?;
                    if entry.path()?.file_name().unwrap() == &PathBuf::from(&info.json) {
                        break entry;
                    }
                } else {
                    panic!("could not find wpd entry")
                }
            };
            // let mut s = String::new();
            // info_entry.read_to_string(&mut s).unwrap();
            serde_json::from_reader(json_entry).unwrap()
        };
        Ok(wpd)
    }

    pub fn to_data_vecs(&self) -> Vec<DataVector<f64,f64>> {
        self.datasets.iter().map(|ds| ds.to_data_vec()).collect()
    }

    pub fn get(&self, name: &str) -> Option<&DataSet> {
        for dv in self.datasets.iter() {
            if dv.name == name {
                return Some(dv);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub version: Vec<usize>,
    pub json: String,
    pub images: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct DataSet {
    pub name: String,
    #[serde(rename = "metadataKeys")]
    pub metadata_keys: Vec<String>,
    pub data: Vec<DataPoint>
}

impl DataSet {
    pub fn to_data_vec(&self) -> DataVector<f64,f64> {
        let mut values = Vec::with_capacity(self.data.len());
        for dp in self.data.iter() {
            values.push(Point{x:dp.x(), y: dp.y()});
        }
        values.sort();
        DataVector::new(
            self.name.clone(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            values)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct Axes {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "isLogX")]
    pub is_log_x: bool,
    #[serde(rename = "isLogY")]
    pub is_log_y: bool,
    #[serde(rename = "calibrationPoints")]
    pub calibration_points: Vec<CalibrationPoint>,
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct CalibrationPoint {
    pub px: f64,
    pub py: f64,
    pub dx: String,
    pub dy: String,
    pub dz: Option<String>,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub value: (f64, f64),
}

impl DataPoint {
    pub fn x(&self) -> f64 {
        self.value.0
    }
    pub fn y(&self) -> f64 {
        self.value.1
    }
}

/// Find the average of a @Foldable@ of values.
pub fn average(samples: &Vec<f64>) -> f64 {
    let n = samples.len();
    (samples.into_iter().sum::<f64>()) / (n as f64)
}

pub fn maximum(samples: &Vec<f64>) -> f64 {
    if samples.len() == 0 {
        panic!("Cannot find the max of empty vec");
    }
    let mut max: f64 = samples[0];
    for s in samples {
        if *s > max {
            max = *s;
        }
    }
    max
}

pub fn minimum(samples: &Vec<f64>) -> f64 {
    if samples.len() == 0 {
        panic!("Cannot find the min of empty vec");
    }
    let mut min: f64 = samples[0];
    for s in samples {
        if *s < min {
            min = *s;
        }
    }
    min
}
