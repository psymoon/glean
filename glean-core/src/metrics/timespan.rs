// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::time_unit::TimeUnit;
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

/// A timespan metric.
///
/// Timespans are used to make a measurement of how much time is spent in a particular task.
#[derive(Debug)]
pub struct TimespanMetric {
    meta: CommonMetricData,
    time_unit: TimeUnit,
    start_time: Option<u64>,
}

impl MetricType for TimespanMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl TimespanMetric {
    /// Create a new timespan metric.
    pub fn new(meta: CommonMetricData, time_unit: TimeUnit) -> Self {
        Self {
            meta,
            time_unit,
            start_time: None,
        }
    }

    /// Start tracking time for the provided metric.
    ///
    /// This records an error if it's already tracking time (i.e. start was already
    /// called with no corresponding `stop`): in that case the original
    /// start time will be preserved.
    pub fn set_start(&mut self, glean: &Glean, start_time: u64) {
        if !self.should_record(glean) {
            return;
        }

        if self.start_time.is_some() {
            record_error(
                glean,
                &self.meta,
                ErrorType::InvalidValue,
                "Timespan already started",
                None,
            );
            return;
        }

        self.start_time = Some(start_time);
    }

    /// Stop tracking time for the provided metric. Sets the metric to the elapsed time.
    ///
    /// This will record an error if no `start` was called.
    pub fn set_stop(&mut self, glean: &Glean, stop_time: u64) {
        if self.start_time.is_none() {
            record_error(
                glean,
                &self.meta,
                ErrorType::InvalidValue,
                "Timespan not running",
                None,
            );
            return;
        }

        let duration = stop_time - self.start_time.take().unwrap();
        let duration = Duration::from_nanos(duration);
        self.set_raw(glean, duration, false);
    }

    /// Abort a previous `start` call. No error is recorded if no `start` was called.
    pub fn cancel(&mut self) {
        self.start_time = None;
    }

    /// Explicitly set the timespan value.
    ///
    /// This API should only be used if your library or application requires recording
    /// times in a way that can not make use of `start`/`stop`/`cancel`.
    ///
    /// Care should be taken using this if the ping lifetime might contain more than one
    /// timespan measurement. To be safe, `set_raw` should generally be followed by
    /// sending a custom ping containing the timespan.
    ///
    /// ## Arguments
    ///
    /// * `elapsed` - The elapsed time to record.
    /// * `overwrite` - Whether or not to overwrite existing data.
    pub fn set_raw(&self, glean: &Glean, elapsed: Duration, overwrite: bool) {
        if !self.should_record(glean) {
            return;
        }

        if self.start_time.is_some() {
            record_error(
                glean,
                &self.meta,
                ErrorType::InvalidValue,
                "Timespan already running. Raw value not recorded.",
            );
            return;
        }

        glean.storage().record_with(&self.meta, |old_value| {
            if overwrite {
                Metric::Timespan(elapsed, self.time_unit)
            } else {
                match old_value {
                    Some(old @ Metric::Timespan(..)) => old,
                    _ => Metric::Timespan(elapsed, self.time_unit),
                }
            }
        });
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as an integer.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<u64> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::Timespan(time, time_unit)) => Some(time_unit.duration_convert(time)),
            _ => None,
        }
    }
}
