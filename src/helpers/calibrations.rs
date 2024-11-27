use derive_builder::Builder;

/// Calibration constants for T7 analog input conversion
/// See [Labjack documentation](https://support.labjack.com/docs/20-0-1-t7-calibration-constants-t-series-datasheet)
/// Defaults to the nominal calibrations for the +/- 10V range.
#[derive(Builder, Debug, PartialEq, Clone)]
pub struct AinCalibration {
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
pub struct DacCalibration {
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
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_1_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_10_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_100_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hs_gain_1000_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_1_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_10_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_100_ain_cal: AinCalibration,
    #[builder(default = AinCalibrationBuilder::default().build().unwrap())]
    pub hr_gain_1000_ain_cal: AinCalibration,
    #[builder(default = DacCalibrationBuilder::default().build().unwrap())]
    pub dac0_cal: DacCalibration,
    #[builder(default = DacCalibrationBuilder::default().build().unwrap())]
    pub dac1_cal: DacCalibration,
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
/// Currently only supports T7
pub fn ain_binary_to_volts(ain_binary: u32, ain_calibration: &AinCalibration) -> f32 {
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
