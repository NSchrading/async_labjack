use derive_builder::Builder;

/// Supported kinds of calibrations
#[derive(Debug)]
pub enum Calibrations {
    T4Calibrations(T4Calibrations),
    T7Calibrations(T7Calibrations),
}

impl From<T4Calibrations> for Calibrations {
    fn from(cal: T4Calibrations) -> Self {
        Calibrations::T4Calibrations(cal)
    }
}

impl From<T7Calibrations> for Calibrations {
    fn from(cal: T7Calibrations) -> Self {
        Calibrations::T7Calibrations(cal)
    }
}

/// Calibration constants for T4 high voltage analog input conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for +/- 10V HV range AIN0.
#[derive(Builder, Debug, PartialEq, Clone)]
pub struct T4AinHVCalibration {
    #[builder(default = 0.0003235316)]
    pub slope: f32,
    #[builder(default = -10.532965)]
    pub offset: f32,
}

/// Calibration constants for T4 low voltage analog input conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for 0-2.5V LV range AIN0.
#[derive(Builder, Debug, PartialEq, Clone)]
pub struct T4AinLVCalibration {
    #[builder(default = 0.00003826692)]
    pub slope: f32,
    #[builder(default = 0.002484)]
    pub offset: f32,
}

/// Calibration constants for T4 SpecV
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibration.
#[derive(Builder, Debug, PartialEq, Clone)]
pub struct T4SpecVCalibration {
    #[builder(default = -0.0000383942)]
    pub slope: f32,
    #[builder(default = 2.507430)]
    pub offset: f32,
}

/// Calibration constants for T4 DAC conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-0-t4-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for DAC0.
#[derive(Builder, Debug, PartialEq, Clone)]
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

/// Calibration constants for T7 analog input conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for the +/- 10V range.
#[derive(Builder, Debug, PartialEq, Clone)]
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

/// Calibration constants for T7 DAC conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations.
#[derive(Builder, Debug, PartialEq, Clone)]
pub struct T7DacCalibration {
    #[builder(default = 13200.0)]
    pub slope: f32,
    #[builder(default = 0.0)]
    pub offset: f32,
}

/// Calibration constants for T7 temperature conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations.
#[derive(Builder, Debug, PartialEq, Clone)]
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

/// Convert a binary analog input value (e.g. from streaming or reading AIN<N>_BINARY) to a floating
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

/// Convert a binary analog input value (e.g. from streaming or reading AIN<N>_BINARY) to a floating
/// point voltage value using the provided slope and offset. This only works on lower precision
/// (16-bit) ADC values. If using a t7 or t7-pro, it is recommended to use
/// t7_ain_binary_to_volts instead.
///
/// # Examples
///
/// ```
/// use tokio_labjack_lib::helpers::calibrations::T4CalibrationsBuilder;
/// use tokio_labjack_lib::helpers::calibrations::ain_binary_to_volts;
/// let calibrations = T4CalibrationsBuilder::default().build().unwrap();
/// ain_binary_to_volts(65535, calibrations.ain0_cal.slope, calibrations.ain0_cal.offset);
/// ```
pub fn ain_binary_to_volts(ain_binary: u16, slope: f32, offset: f32) -> f32 {
    (ain_binary as f32) * slope + offset
}
