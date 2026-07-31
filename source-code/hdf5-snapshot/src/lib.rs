use hdf5::filters::Filter;
use hdf5::types::VarLenUnicode;
use hdf5::{Error, File, Location, Result};
use ndarray::{Array1, Array2, s};
use std::path::Path;
use std::str::FromStr;

const BASELINE_TEMPERATURE: f64 = 280.0;
const PEAK_INCREASE: f64 = 40.0;
const GAUSSIAN_WIDTH: f64 = 0.25;
const COMPRESSION_LEVEL: u8 = 4;
const CHUNK_SIZE: usize = 16;

#[derive(Debug)]
pub struct Snapshot {
    pub x: Array1<f64>,
    pub y: Array1<f64>,
    pub temperature: Array2<f64>,
}

#[derive(Debug)]
pub struct SnapshotInfo {
    pub shape: Vec<usize>,
    pub chunk_shape: Option<Vec<usize>>,
    pub filters: Vec<Filter>,
    pub description: String,
    pub units: String,
    pub time_seconds: f64,
    pub step: u64,
    pub minimum_temperature: f64,
    pub maximum_temperature: f64,
    pub center: Array2<f64>,
}

fn write_string_attribute(location: &Location, name: &str, value: &str) -> Result<()> {
    let value =
        VarLenUnicode::from_str(value).map_err(|error| Error::Internal(error.to_string()))?;
    location
        .new_attr::<VarLenUnicode>()
        .shape(())
        .create(name)?
        .write_scalar(&value)
}

pub fn gaussian_snapshot(nx: usize, ny: usize) -> Snapshot {
    let x: Array1<f64> = Array1::linspace(-1.0, 1.0, nx);
    let y: Array1<f64> = Array1::linspace(-1.0, 1.0, ny);
    let temperature = Array2::from_shape_fn((ny, nx), |(row, column)| {
        let radius_squared = x[column].powi(2) + y[row].powi(2);
        BASELINE_TEMPERATURE
            + PEAK_INCREASE * (-radius_squared / (2.0 * GAUSSIAN_WIDTH.powi(2))).exp()
    });

    Snapshot { x, y, temperature }
}

pub fn write_snapshot(
    path: &Path,
    snapshot: &Snapshot,
    time_seconds: f64,
    step: u64,
) -> Result<()> {
    let file = File::create(path)?;
    let group = file.create_group("snapshot")?;

    write_string_attribute(&group, "description", "Gaussian temperature snapshot")?;
    group
        .new_attr::<f64>()
        .shape(())
        .create("time_seconds")?
        .write_scalar(&time_seconds)?;
    group
        .new_attr::<u64>()
        .shape(())
        .create("step")?
        .write_scalar(&step)?;

    let x_dataset = group
        .new_dataset_builder()
        .with_data(snapshot.x.view())
        .create("x")?;
    write_string_attribute(&x_dataset, "units", "m")?;

    let y_dataset = group
        .new_dataset_builder()
        .with_data(snapshot.y.view())
        .create("y")?;
    write_string_attribute(&y_dataset, "units", "m")?;

    let (rows, columns) = snapshot.temperature.dim();
    if rows == 0 || columns == 0 {
        return Err(Error::Internal("temperature must be non-empty".to_owned()));
    }
    if snapshot.x.len() != columns || snapshot.y.len() != rows {
        return Err(Error::Internal(format!(
            "coordinate lengths must match temperature shape: x={}, y={}, temperature=({rows}, {columns})",
            snapshot.x.len(),
            snapshot.y.len(),
        )));
    }

    let temperature_dataset = group
        .new_dataset_builder()
        .chunk((rows.min(CHUNK_SIZE), columns.min(CHUNK_SIZE)))
        .deflate(COMPRESSION_LEVEL)
        .with_data(snapshot.temperature.view())
        .create("temperature")?;
    write_string_attribute(&temperature_dataset, "units", "K")?;
    write_string_attribute(&temperature_dataset, "long_name", "temperature")?;
    let minimum_temperature = snapshot
        .temperature
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let maximum_temperature = snapshot
        .temperature
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    temperature_dataset
        .new_attr::<f64>()
        .shape(())
        .create("valid_min")?
        .write_scalar(&minimum_temperature)?;
    temperature_dataset
        .new_attr::<f64>()
        .shape(())
        .create("valid_max")?
        .write_scalar(&maximum_temperature)?;

    file.flush()
}

pub fn read_snapshot(path: &Path) -> Result<Snapshot> {
    let file = File::open(path)?;
    let group = file.group("snapshot")?;

    Ok(Snapshot {
        x: group.dataset("x")?.read_1d()?,
        y: group.dataset("y")?.read_1d()?,
        temperature: group.dataset("temperature")?.read_2d()?,
    })
}

pub fn inspect_snapshot(path: &Path) -> Result<SnapshotInfo> {
    let file = File::open(path)?;
    let group = file.group("snapshot")?;
    let dataset = group.dataset("temperature")?;
    let shape = dataset.shape();
    let (rows, columns) = match shape.as_slice() {
        [rows, columns] => (*rows, *columns),
        _ => {
            return Err(Error::Internal(format!(
                "temperature must be two-dimensional, found shape {shape:?}"
            )));
        }
    };

    let window_rows = rows.min(3);
    let window_columns = columns.min(3);
    let row_start = (rows - window_rows) / 2;
    let column_start = (columns - window_columns) / 2;
    let center = dataset.read_slice_2d(s![
        row_start..row_start + window_rows,
        column_start..column_start + window_columns
    ])?;

    let description: VarLenUnicode = group.attr("description")?.read_scalar()?;
    let units: VarLenUnicode = dataset.attr("units")?.read_scalar()?;

    Ok(SnapshotInfo {
        shape,
        chunk_shape: dataset.chunk(),
        filters: dataset.filters(),
        description: description.as_str().to_owned(),
        units: units.as_str().to_owned(),
        time_seconds: group.attr("time_seconds")?.read_scalar()?,
        step: group.attr("step")?.read_scalar()?,
        minimum_temperature: dataset.attr("valid_min")?.read_scalar()?,
        maximum_temperature: dataset.attr("valid_max")?.read_scalar()?,
        center,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn gaussian_is_centered_and_has_the_requested_shape() {
        let snapshot = gaussian_snapshot(7, 5);

        assert_eq!(snapshot.temperature.dim(), (5, 7));
        assert_eq!(snapshot.x[3], 0.0);
        assert_eq!(snapshot.y[2], 0.0);
        assert_eq!(snapshot.temperature[[2, 3]], 320.0);
        assert!(snapshot.temperature[[0, 0]] > BASELINE_TEMPERATURE);
        assert!(snapshot.temperature[[0, 0]] < 280.001);
    }

    #[test]
    fn snapshot_round_trip_preserves_data_and_metadata() -> Result<()> {
        let directory = tempdir().expect("temporary directory should be created");
        let path = directory.path().join("snapshot.h5");
        let expected = gaussian_snapshot(7, 5);

        write_snapshot(&path, &expected, 12.5, 42)?;

        let actual = read_snapshot(&path)?;
        assert_eq!(actual.x, expected.x);
        assert_eq!(actual.y, expected.y);
        assert_eq!(actual.temperature, expected.temperature);

        let info = inspect_snapshot(&path)?;
        assert_eq!(info.shape, vec![5, 7]);
        assert_eq!(info.chunk_shape, Some(vec![5, 7]));
        assert_eq!(info.filters, vec![Filter::Deflate(COMPRESSION_LEVEL)]);
        assert_eq!(info.description, "Gaussian temperature snapshot");
        assert_eq!(info.units, "K");
        assert_eq!(info.time_seconds, 12.5);
        assert_eq!(info.step, 42);
        assert_eq!(
            info.minimum_temperature,
            expected
                .temperature
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min)
        );
        assert_eq!(info.maximum_temperature, 320.0);
        assert_eq!(info.center, expected.temperature.slice(s![1..4, 2..5]));

        Ok(())
    }
}
