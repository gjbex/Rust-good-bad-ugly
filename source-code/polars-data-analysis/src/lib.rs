use polars::prelude::*;
use std::path::Path;

/// Build the lazy analysis pipeline without executing it.
pub fn patient_summary_query(metadata: LazyFrame, measurements: LazyFrame) -> LazyFrame {
    let metadata = metadata.select([col("patient"), col("gender"), col("condition")]);
    let measurements =
        measurements.select([col("patient"), col("dose"), col("date"), col("temperature")]);

    measurements
        .inner_join(metadata, col("patient"), col("patient"))
        .filter(col("temperature").is_not_null())
        .with_columns([
            (col("temperature") - lit(37.0)).alias("temperature_above_baseline"),
            when(col("temperature").gt_eq(lit(38.0)))
                .then(lit(1_u32))
                .otherwise(lit(0_u32))
                .alias("fever"),
        ])
        .group_by([col("condition"), col("gender")])
        .agg([
            col("patient").n_unique().alias("patients"),
            len().alias("n_obs"),
            col("temperature").mean().alias("mean_temp"),
            col("temperature").std(1).alias("temp_sd"),
            col("temperature_above_baseline").mean().alias("mean_delta"),
            col("fever").sum().alias("fever_n"),
        ])
        .sort(["condition", "gender"], Default::default())
}

/// Scan the two input files and build the analysis pipeline.
pub fn scan_patient_data(metadata: &Path, measurements: &Path) -> PolarsResult<LazyFrame> {
    let metadata = LazyCsvReader::new(PlRefPath::new(metadata.to_string_lossy()))
        .with_has_header(true)
        .finish()?;
    let measurements = LazyCsvReader::new(PlRefPath::new(measurements.to_string_lossy()))
        .with_has_header(true)
        .finish()?;

    Ok(patient_summary_query(metadata, measurements))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_filters_and_aggregates_patient_data() -> PolarsResult<()> {
        let metadata = df!(
            "patient" => [1_i64, 2],
            "gender" => ["F", "F"],
            "condition" => ["A", "A"],
        )?;
        let measurements = df!(
            "patient" => [1_i64, 1, 2, 3],
            "dose" => [0_i64, 2, 5, 0],
            "date" => ["t0", "t1", "t0", "t0"],
            "temperature" => [Some(37.0_f64), Some(38.0), Some(39.0), Some(41.0)],
        )?;

        let summary = patient_summary_query(metadata.lazy(), measurements.lazy()).collect()?;

        assert_eq!(summary.height(), 1);
        assert_eq!(summary.column("patients")?.u32()?.get(0), Some(2));
        assert_eq!(summary.column("n_obs")?.u32()?.get(0), Some(3));
        assert_eq!(summary.column("mean_temp")?.f64()?.get(0), Some(38.0));
        assert_eq!(summary.column("fever_n")?.u32()?.get(0), Some(2));

        Ok(())
    }

    #[test]
    fn ignores_missing_temperature_values() -> PolarsResult<()> {
        let metadata = df!(
            "patient" => [1_i64],
            "gender" => ["M"],
            "condition" => ["B"],
        )?;
        let measurements = df!(
            "patient" => [1_i64, 1],
            "dose" => [0_i64, 1],
            "date" => ["t0", "t1"],
            "temperature" => [Some(37.5_f64), None],
        )?;

        let summary = patient_summary_query(metadata.lazy(), measurements.lazy()).collect()?;

        assert_eq!(summary.column("n_obs")?.u32()?.get(0), Some(1));
        assert_eq!(summary.column("mean_temp")?.f64()?.get(0), Some(37.5));

        Ok(())
    }
}
