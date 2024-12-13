//! Calibration data for converting raw binary ADC values to voltages.
//! See [Labjack documentation](https://support.labjack.com/docs/20-0-internal-flash-t-series-datasheet)
//! for more info.

use crate::{Error, Result};
use derive_builder::Builder;

/// The starting address of internal flash where the calibration constants reside.
/// This is only valid for T4 and T7
pub const CAL_CONST_STARTING_ADDRESS: u32 = 0x3C4000;

/// Supported kinds of calibrations
#[derive(Debug)]
pub enum Calibrations {
    T4Calibrations(T4Calibrations),
    T7Calibrations(T7Calibrations),
}

/// Convert a [`T4Calibrations`] to a [`Calibrations`]
impl From<T4Calibrations> for Calibrations {
    fn from(cal: T4Calibrations) -> Self {
        Calibrations::T4Calibrations(cal)
    }
}

/// Convert a [`T7Calibrations`] to a [`Calibrations`]
impl From<T7Calibrations> for Calibrations {
    fn from(cal: T7Calibrations) -> Self {
        Calibrations::T7Calibrations(cal)
    }
}

impl TryInto<T7Calibrations> for Calibrations {
    type Error = Error;

    fn try_into(self) -> Result<T7Calibrations> {
        match self {
            Calibrations::T7Calibrations(cal) => Ok(cal),
            _ => Err(Error::Other(format!(
                "Expected T7Calibrations, got {:?}",
                self
            ))),
        }
    }
}

impl TryInto<T4Calibrations> for Calibrations {
    type Error = Error;

    fn try_into(self) -> Result<T4Calibrations> {
        match self {
            Calibrations::T4Calibrations(cal) => Ok(cal),
            _ => Err(Error::Other(format!(
                "Expected T4Calibrations, got {:?}",
                self
            ))),
        }
    }
}

/// Calibration constants for T4 high voltage analog input conversion.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for +/- 10V HV range AIN0.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct T4AinHVCalibration {
    #[builder(default = 0.0003235316)]
    pub slope: f32,
    #[builder(default = -10.532965)]
    pub offset: f32,
}

/// Calibration constants for T4 low voltage analog input conversion.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for 0-2.5V LV range AIN0.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct T4AinLVCalibration {
    #[builder(default = 0.00003826692)]
    pub slope: f32,
    #[builder(default = 0.002484)]
    pub offset: f32,
}

/// Calibration constants for T4 SpecV.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibration.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct T4SpecVCalibration {
    #[builder(default = -0.0000383942)]
    pub slope: f32,
    #[builder(default = 2.507430)]
    pub offset: f32,
}

/// Calibration constants for T4 DAC conversion.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for DAC0.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct T4DacCalibration {
    #[builder(default = 13107.68)]
    pub slope: f32,
    #[builder(default = 54.091066)]
    pub offset: f32,
}

/// All calibration constants for the T4.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations.
#[derive(Builder, Debug, PartialEq)]
pub struct T4Calibrations {
    #[builder(default = T4AinHVCalibrationBuilder::default().build().unwrap())]
    pub ain0_cal: T4AinHVCalibration,
    #[builder(default = T4AinHVCalibrationBuilder::default().build().unwrap())]
    pub ain1_cal: T4AinHVCalibration,
    #[builder(default = T4AinHVCalibrationBuilder::default().build().unwrap())]
    pub ain2_cal: T4AinHVCalibration,
    #[builder(default = T4AinHVCalibrationBuilder::default().build().unwrap())]
    pub ain3_cal: T4AinHVCalibration,
    #[builder(default = T4AinLVCalibrationBuilder::default().build().unwrap())]
    pub lv_cal: T4AinLVCalibration,
    #[builder(default = T4SpecVCalibrationBuilder::default().build().unwrap())]
    pub spec_v_cal: T4SpecVCalibration,
    #[builder(default = T4DacCalibrationBuilder::default().build().unwrap())]
    pub dac0_cal: T4DacCalibration,
    #[builder(default = T4DacCalibrationBuilder::default().build().unwrap())]
    pub dac1_cal: T4DacCalibration,
    #[builder(default = TemperatureCalibrationBuilder::default().build().unwrap())]
    pub temperature_cal: TemperatureCalibration,
    #[builder(default = 0.000000015)]
    pub ain_bias_current: f32,
}

/// Calibration constants for T7 analog input conversion.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for the +/- 10V range.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct T7AinCalibration {
    #[builder(default = 33523.0)]
    pub binary_center: f32,
    #[builder(default = 0.000315805780)]
    pub positive_slope: f32,
    #[builder(default = -0.000315805800)]
    pub negative_slope: f32,
    #[builder(default = -10.586956522)]
    pub voltage_offset: f32,
}

/// Calibration constants for T7 DAC conversion.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct T7DacCalibration {
    #[builder(default = 13200.0)]
    pub slope: f32,
    #[builder(default = 0.0)]
    pub offset: f32,
}

/// Calibration constants for T7 temperature conversion.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations.
#[derive(Builder, Debug, PartialEq, Clone, Copy)]
pub struct TemperatureCalibration {
    #[builder(default = -92.6)]
    pub slope: f32,
    #[builder(default = 467.6)]
    pub offset: f32,
}

/// All calibration constants for the T7.
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations.
#[derive(Builder, Debug, PartialEq)]
pub struct T7Calibrations {
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_1_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_10_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_100_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_1000_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_1_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_10_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_100_ain_cal: T7AinCalibration,
    #[builder(default = T7AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_1000_ain_cal: T7AinCalibration,
    #[builder(default = T7DacCalibrationBuilder::default().build().unwrap())]
    pub dac0_cal: T7DacCalibration,
    #[builder(default = T7DacCalibrationBuilder::default().build().unwrap())]
    pub dac1_cal: T7DacCalibration,
    #[builder(default = TemperatureCalibrationBuilder::default().build().unwrap())]
    pub temperature_cal: TemperatureCalibration,
    #[builder(default = 0.000010)]
    pub i_source_10u: f32,
    #[builder(default = 0.000200)]
    pub i_source_200u: f32,
    #[builder(default = 0.000000015)]
    pub ain_bias_current: f32,
}

/// Convert a binary analog input value (e.g. from streaming or reading `AIN<N>_BINARY`) to a floating
/// point voltage value using the provided AinCalibration.
pub fn t7_ain_binary_to_volts(ain_binary: u32, ain_calibration: &T7AinCalibration) -> f32 {
    let ain_bin_float: f32 = if ain_binary > (u16::MAX as u32) {
        // we're getting 24-bit precision values, labjack normalizes the conversions to 16-bits.
        // To compensate, we divide by 256.0 to approximate the 16-bit value.
        (ain_binary as f32) / 256.0
    } else {
        // we're getting 16-bit precision values
        ain_binary as f32
    };
    if ain_bin_float < ain_calibration.binary_center {
        return (ain_calibration.binary_center - ain_bin_float) * ain_calibration.negative_slope;
    }
    (ain_bin_float - ain_calibration.binary_center) * ain_calibration.positive_slope
}

/// Convert a binary analog input value (e.g. from streaming or reading `AIN<N>_BINARY`) to a
/// floating point voltage value using the provided slope and offset. This only works on lower
/// precision (16-bit) ADC values. If using a t7 or t7-pro, it is recommended to use
/// t7_ain_binary_to_volts instead.
///
/// # Examples
///
/// ```
/// use async_labjack::helpers::calibrations::T4CalibrationsBuilder;
/// use async_labjack::helpers::calibrations::ain_binary_to_volts;
///
/// let calibrations = T4CalibrationsBuilder::default().build().unwrap();
/// ain_binary_to_volts(65535, calibrations.ain0_cal.slope, calibrations.ain0_cal.offset);
/// ```
pub fn ain_binary_to_volts(ain_binary: u16, slope: f32, offset: f32) -> f32 {
    (ain_binary as f32) * slope + offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t7_ain_binary_to_volts_16bit() {
        // default is +/- 10V
        let ain_calibration = T7AinCalibrationBuilder::default().build().unwrap();
        let ain_binary: u32 = 65535;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((10.1..=10.2).contains(&volts), "{volts}");

        let ain_binary: u32 = 0;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((-10.6..=-10.5).contains(&volts), "{volts}");

        let ain_binary: u32 = 32768;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((-0.3..=0.0).contains(&volts), "{volts}");
    }

    #[test]
    fn test_t7_ain_binary_to_volts_16bit_1v_range() {
        // use default values for +/- 1V
        let ain_calibration = T7AinCalibrationBuilder::default()
            .positive_slope(0.000031580578)
            .negative_slope(-0.0000315806)
            .binary_center(33523.0)
            .voltage_offset(-1.0586956)
            .build()
            .unwrap();
        let ain_binary: u32 = 65535;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((1.01..=1.02).contains(&volts), "{volts}");

        let ain_binary: u32 = 0;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((-1.06..=-1.05).contains(&volts), "{volts}");

        let ain_binary: u32 = 32768;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((-0.03..=-0.02).contains(&volts), "{volts}");
    }

    #[test]
    fn test_t7_ain_binary_to_volts_24bit() {
        // default is +/- 10V
        let ain_calibration = T7AinCalibrationBuilder::default().build().unwrap();
        let ain_binary: u32 = 16777215;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((10.1..=10.2).contains(&volts), "{volts}");

        let ain_binary: u32 = 0;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((-10.6..=-10.5).contains(&volts), "{volts}");

        let ain_binary: u32 = 8388607;
        let volts = t7_ain_binary_to_volts(ain_binary, &ain_calibration);
        assert!((-0.3..=-0.2).contains(&volts), "{volts}");
    }

    #[test]
    fn test_ain_binary_to_volts() {
        // default is +/- 10V
        let calibrations = T4CalibrationsBuilder::default().build().unwrap();
        let ain_binary: u16 = 65535;

        let volts = ain_binary_to_volts(
            ain_binary,
            calibrations.ain0_cal.slope,
            calibrations.ain0_cal.offset,
        );
        assert!((10.6..=10.7).contains(&volts), "{volts}");

        let ain_binary: u16 = 0;
        let volts = ain_binary_to_volts(
            ain_binary,
            calibrations.ain0_cal.slope,
            calibrations.ain0_cal.offset,
        );
        assert!((-10.6..=-10.5).contains(&volts), "{volts}");

        let ain_binary: u16 = 32768;
        let volts = ain_binary_to_volts(
            ain_binary,
            calibrations.ain0_cal.slope,
            calibrations.ain0_cal.offset,
        );
        assert!((0.0..=0.1).contains(&volts), "{volts}");
    }

    #[test]
    fn test_from_t7() {
        let ain_calibration = T7CalibrationsBuilder::default().build().unwrap();
        let cal: Calibrations = ain_calibration.into();
        assert!(matches!(cal, Calibrations::T7Calibrations(_)));
    }

    #[test]
    fn test_from_t4() {
        let ain_calibration = T4CalibrationsBuilder::default().build().unwrap();
        let cal: Calibrations = ain_calibration.into();
        assert!(matches!(cal, Calibrations::T4Calibrations(_)));
    }

    #[test]
    fn test_cal_to_t7() {
        let ain_calibration = T7CalibrationsBuilder::default().build().unwrap();
        let cal: Calibrations = ain_calibration.into();
        #[allow(clippy::useless_conversion)]
        match cal.try_into() {
            Ok(Calibrations::T7Calibrations(cal)) => {
                assert_eq!(cal.temperature_cal.slope, -92.6);
            }
            cal => {
                panic!("Unexpected conversion: {cal:?}")
            }
        }

        let ain_calibration = T7CalibrationsBuilder::default().build().unwrap();
        let cal: Calibrations = ain_calibration.into();
        match std::convert::TryInto::<T4Calibrations>::try_into(cal) {
            Err(_) => {}
            Ok(cal) => {
                panic!("Unexpected success on conversion to wrong type {cal:?}");
            }
        }
    }

    #[test]
    fn test_cal_to_t4() {
        let ain_calibration = T4CalibrationsBuilder::default().build().unwrap();
        let cal: Calibrations = ain_calibration.into();
        #[allow(clippy::useless_conversion)]
        match cal.try_into() {
            Ok(Calibrations::T4Calibrations(cal)) => {
                assert_eq!(cal.ain0_cal.offset, -10.532965);
            }
            cal => {
                panic!("Unexpected conversion: {cal:?}")
            }
        }

        let ain_calibration = T4CalibrationsBuilder::default().build().unwrap();
        let cal: Calibrations = ain_calibration.into();
        match std::convert::TryInto::<T7Calibrations>::try_into(cal) {
            Err(_) => {}
            Ok(cal) => {
                panic!("Unexpected success on conversion to wrong type {cal:?}");
            }
        }
    }
}
