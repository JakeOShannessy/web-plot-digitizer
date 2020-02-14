use serde::{Serialize, Deserialize};
use std::fs::File;
use data_vector::{DataVector, Point};
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
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

    pub fn to_data_vecs(&self) -> Vec<DataVector> {
        self.datasets.iter().map(|ds| ds.to_data_vec()).collect()
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSet {
    pub name: String,
    #[serde(rename = "metadataKeys")]
    pub metadata_keys: Vec<String>,
    pub data: Vec<DataPoint>
}

impl DataSet {
    pub fn to_data_vec(&self) -> DataVector {
        let mut values = Vec::with_capacity(self.data.len());
        for dp in self.data.iter() {
            values.push(Point{x:dp.x(), y: dp.y()});
        }
        values.sort();
        DataVector {
            name: self.name.clone(),
            x_name: "Time".to_string(),
            x_units: "s".to_string(),
            y_name: "Time".to_string(),
            y_units: "kW".to_string(),
            values,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CalibrationPoint {
    pub px: f64,
    pub py: f64,
    pub dx: String,
    pub dy: String,
    pub dz: Option<String>,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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

fn create_hrr_ramp(name: String, data: &DataVector) {
    for dp in data.values.iter() {
        println!("&RAMP ID='{}' T={}, F={} /", name, dp.x, dp.y);
    }
}
