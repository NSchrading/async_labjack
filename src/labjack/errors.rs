use crate::helpers::macros::back_to_enum;
use thiserror::Error;

back_to_enum! {
    /// Enum containing all labjack error codes. These can be obtained by reading the
    /// [`LAST_ERR_DETAIL`] tag
    #[derive(Debug, Error)]
    #[repr(u16)]
    pub enum LabjackError {
        #[error("LjSuccess")]
        LjSuccess = 0,
        #[error("FeatureNotImplemented")]
        FeatureNotImplemented = 254,
        /// Feature is not supported on this device.
        #[error("Feature is not supported on this device.")]
        FeatureNotSupported = 2012,
        /// Response packet greater than max packet size.
        #[error("Response packet greater than max packet size.")]
        ModbusRspOverflow = 2300,
        /// Command packet greater than max packet size
        #[error("Command packet greater than max packet size")]
        ModbusCmdOverflow = 2301,
        #[error("ModbusStringCmdTooBig")]
        ModbusStringCmdTooBig = 2310,
        #[error("ModbusStringParamTooBig")]
        ModbusStringParamTooBig = 2311,
        #[error("ModbusStringBadNumParams")]
        ModbusStringBadNumParams = 2312,
        /// Register data types does not match the number of registers in the request.
        #[error("Register data types does not match the number of registers in the request.")]
        ModbusInvalidNumRegisters = 2313,
        #[error("ModbusReadTooLarge")]
        ModbusReadTooLarge = 2314,
        /// Register or group of registers requires the access be in an even number of registers.
        #[error("Register or group of registers requires the access be in an even number of registers.")]
        ModbusNumRegsMustBeEven = 2315,
        /// Strings must be terminated with a null (\0, 0x00).
        #[error("Strings must be terminated with a null (\0, 0x00).")]
        ModbusStringMissingNull = 2316,
        /// The UDP discovery socket has been set to discovery only. Only a few registers may be read while in this mode.
        #[error("The UDP discovery socket has been set to discovery only. Only a few registers may be read while in this mode.")]
        UdpDiscoveryOnlyModeIsEnabled = 2317,
        #[error("StartupConfigInvalidCode")]
        StartupConfigInvalidCode = 2330,
        /// An attempt was made to read beyond the configuration structure.
        #[error("An attempt was made to read beyond the configuration structure.")]
        StartupConfigInvalidRead = 2331,
        /// The FIFO can not contain any data when data size is being changed.
        #[error("The FIFO can not contain any data when data size is being changed.")]
        UserRamFifoMustBeEmpty = 2340,
        /// The FIFO contains fewer values than requested.
        #[error("The FIFO contains fewer values than requested.")]
        UserRamFifoInsufficientValues = 2343,
        /// FIFO does not have enough free space to hold the requested write. No data was added to the FIFO.
        #[error("FIFO does not have enough free space to hold the requested write. No data was added to the FIFO.")]
        UserRamFifoInsufficientSpace = 2344,
        /// The number of bytes allocated to the FIFO must be even..
        #[error("The number of bytes allocated to the FIFO must be even..")]
        UserRamFifoSizeMustBeEven = 2345,
        #[error("IntflashAddInvalid")]
        IntflashAddInvalid = 2350,
        #[error("IntflashCodeInvalid")]
        IntflashCodeInvalid = 2351,
        /// Attempted to read or write a section that is not allowed.
        #[error("Attempted to read or write a section that is not allowed.")]
        IntflashOpProhibited = 2352,
        /// Attempted to write or read beyond the currently selected section.
        #[error("Attempted to write or read beyond the currently selected section.")]
        IntflashSectionOverwrite = 2353,
        /// Specified Key and Address mismatch.
        #[error("Specified Key and Address mismatch.")]
        IntflashKeyInvalid = 2354,
        /// A write to flash failed to set one or more bits to the desired values.
        #[error("A write to flash failed to set one or more bits to the desired values.")]
        FlashVerificationFailed = 2355,
        /// One or more bits failed to set during a flash erase operation.
        #[error("One or more bits failed to set during a flash erase operation.")]
        FlashEraseFailed = 2356,
        /// Flash can not be accessed due to the WiFi module booting up.
        #[error("Flash can not be accessed due to the WiFi module booting up.")]
        IntflashUnavailable = 2358,
        /// The file system can not be accessed due to the WiFi module booting up.
        #[error("The file system can not be accessed due to the WiFi module booting up.")]
        FileioUnavailable = 2359,
        /// Specified range not available on this device.
        #[error("Specified range not available on this device.")]
        AinRangeInvalid = 2370,
        /// Specified settling is greater than device max.
        #[error("Specified settling is greater than device max.")]
        AinSettlingInvalid = 2371,
        /// Analog input system currently set to binary. Some operations, such as AIN_EF, will fail.
        #[error("Analog input system currently set to binary. Some operations, such as AIN_EF, will fail.")]
        AinSetToBinary = 2372,
        /// For channel range 0-13: Negative channel must be even channel number + 1.  For extended channel range 48-127: Negative channel must be channel number + 8.
        #[error("For channel range 0-13: Negative channel must be even channel number + 1.  For extended channel range 48-127: Negative channel must be channel number + 8.")]
        AinNegativeChannelInvalid = 2373,
        /// Only a value of zero may be written to this address.
        #[error("Only a value of zero may be written to this address.")]
        AinAllZeroOnly = 2374,
        /// Selected resolution is invalid. Valid range is 0-5 for T4 and 0-12 for T7.
        #[error("Selected resolution is invalid. Valid range is 0-5 for T4 and 0-12 for T7.")]
        AinResolutionInvalid = 2375,
        /// The selected AIN rate configuration is invalid for the desired resolution index or vise-versa.
        #[error("The selected AIN rate configuration is invalid for the desired resolution index or vise-versa.")]
        AinRateInvalid = 2376,
        /// The AIN channel has been disabled
        #[error("The AIN channel has been disabled")]
        AinChnDisabled = 2377,
        #[error("LuaVmStateNoChange")]
        LuaVmStateNoChange = 2380,
        #[error("LuaInitializationError")]
        LuaInitializationError = 2381,
        /// Requested more than the max possible number of IO floats.
        #[error("Requested more than the max possible number of IO floats.")]
        LuaInvalidNumIoFloats = 2382,
        /// Attempt to read/write beyond the currently allocated IO space.
        #[error("Attempt to read/write beyond the currently allocated IO space.")]
        LuaIoFloatmemOverflow = 2383,
        #[error("LuaInvalidMode")]
        LuaInvalidMode = 2384,
        /// A running script is preventing the requested operation.
        #[error("A running script is preventing the requested operation.")]
        LuaIsRunning = 2385,
        /// Attempted to run a program that is not present.
        #[error("Attempted to run a program that is not present.")]
        LuaCodeBufferEmpty = 2386,
        /// Attempted to read from debug buffer while debug is disabled.
        #[error("Attempted to read from debug buffer while debug is disabled.")]
        LuaDebugIsDisabled = 2387,
        /// The Lua table provided is too small to process the request.
        #[error("The Lua table provided is too small to process the request.")]
        LuaTableSmallerThanSpecifiedSize = 2388,
        /// The Lua VM is being closed, usually takes less than 100 ms.
        #[error("The Lua VM is being closed, usually takes less than 100 ms.")]
        LuaIsClosing = 2389,
        /// Insufficient RAM to perform the requested action. Often occurs when a loaded Lua script is too large.
        #[error("Insufficient RAM to perform the requested action. Often occurs when a loaded Lua script is too large.")]
        SystemMemoryBereft = 2400,
        /// Attempted to overwrite a buffer.
        #[error("Attempted to overwrite a buffer.")]
        SystemMemoryOverwrite = 2401,
        /// Invalid code supplied when issuing a reboot.
        #[error("Invalid code supplied when issuing a reboot.")]
        SystemRebootCodeInvalid = 2402,
        #[error("SystemReadOverrun")]
        SystemReadOverrun = 2403,
        #[error("SystemInvalidPin")]
        SystemInvalidPin = 2404,
        /// NVRAM is not available on this device.
        #[error("NVRAM is not available on this device.")]
        SystemNvramUnavailable = 2405,
        /// The specified NVRAM location is not available on this device.
        #[error("The specified NVRAM location is not available on this device.")]
        SystemNvramInvalidAddress = 2406,
        /// The requested wait time is beyond the max allowed.
        #[error("The requested wait time is beyond the max allowed.")]
        SystemWaitTooLong = 2407,
        /// The firmware image is not compatible with this device. Typically due to flash chip incompatibility.
        #[error("The firmware image is not compatible with this device. Typically due to flash chip incompatibility.")]
        SystemIncompatibleFirmwareVersion = 2408,
        /// Attempted to write a device name with invalid characters.
        #[error("Attempted to write a device name with invalid characters.")]
        DeviceNameMustBeAlphanum = 2420,
        /// Unknown value specified.
        #[error("Unknown value specified.")]
        PowerInvalidSetting = 2450,
        /// Core must be running at 20MHz minimum for USB to operate.
        #[error("Core must be running at 20MHz minimum for USB to operate.")]
        PowerUsbNeeds20mhzOrMore = 2451,
        /// Core must be running at 20MHz minimum for Ethernet to operate.
        #[error("Core must be running at 20MHz minimum for Ethernet to operate.")]
        PowerEthNeeds20mhzOrMore = 2452,
        /// Core must be running at 20MHz minimum for AIN to operate.
        #[error("Core must be running at 20MHz minimum for AIN to operate.")]
        PowerAinNeeds20mhzOrMore = 2453,
        /// Core must be running at 20MHz minimum to use stream mode.
        #[error("Core must be running at 20MHz minimum to use stream mode.")]
        PowerStreamNeeds20mhzOrMore = 2454,
        /// Core must be running at 20MHz minimum to use the SD card.
        #[error("Core must be running at 20MHz minimum to use the SD card.")]
        PowerSdNeeds20mhzOrMore = 2455,
        /// Can not change the power level of the connected medium.
        #[error("Can not change the power level of the connected medium.")]
        PowerCanNotChangeUsedConnection = 2456,
        /// The written power mode is the same as the current setting.
        #[error("The written power mode is the same as the current setting.")]
        PowerNoChange = 2459,
        /// Analog input system is powered down.
        #[error("Analog input system is powered down.")]
        PowerAnalogOff = 2460,
        /// WiFi needs to be connected to a network before the requested action can be performed.
        #[error("WiFi needs to be connected to a network before the requested action can be performed.")]
        WifiNotAssociated = 2490,
        #[error("HwDioNotAvailable")]
        HwDioNotAvailable = 2500,
        /// The digital line addressed is set to analog. Digital operations cannot be performed.
        #[error("The digital line addressed is set to analog. Digital operations cannot be performed.")]
        DioSetToAnalog = 2501,
        /// Counter A is being used by another system. Typically this is due to a high speed counter being enabled while trying to enable a DIO_EF clock that requires the same resource or vise-versa
        #[error("Counter A is being used by another system. Typically this is due to a high speed counter being enabled while trying to enable a DIO_EF clock that requires the same resource or vise-versa")]
        HwCntraNotAvailable = 2508,
        /// Counter B is being used by another system. Typically this is due to a high speed counter being enabled while trying to enable a DIO_EF clock that requires the same resource or vise-versa
        #[error("Counter B is being used by another system. Typically this is due to a high speed counter being enabled while trying to enable a DIO_EF clock that requires the same resource or vise-versa")]
        HwCntrbNotAvailable = 2509,
        /// Counter C is being used by another system.
        #[error("Counter C is being used by another system.")]
        HwCntrcNotAvailable = 2510,
        /// Counter D is being used by another system. Typically this is due to a high speed counter that shares a resource with the stream clock being enabled while trying to enable stream mode or vise-versa
        #[error("Counter D is being used by another system. Typically this is due to a high speed counter that shares a resource with the stream clock being enabled while trying to enable stream mode or vise-versa")]
        HwCntrdNotAvailable = 2511,
        #[error("HwCio0NotAvailable")]
        HwCio0NotAvailable = 2520,
        #[error("HwCio1NotAvailable")]
        HwCio1NotAvailable = 2521,
        /// DAC1 is an active stream out target and therefore unavailable.
        #[error("DAC1 is an active stream out target and therefore unavailable.")]
        HwDac1NotAvailable = 2523,
        /// The LEDs cannot be controlled when the LED power mode is not set to manual.
        #[error("The LEDs cannot be controlled when the LED power mode is not set to manual.")]
        HwLedsNotAvailable = 2524,
        /// The DIO accessed does not support any extended features.
        #[error("The DIO accessed does not support any extended features.")]
        EfDioHasNoTncFeatures = 2550,
        /// The selected type is not recognized.
        #[error("The selected type is not recognized.")]
        EfInvalidType = 2551,
        /// The selected type is not recognized.
        #[error("The selected type is not recognized.")]
        EfTypeNotSupported = 2552,
        /// The requested type is not supported on this DIO pin.
        #[error("The requested type is not supported on this DIO pin.")]
        EfPinTypeMismatch = 2553,
        /// Attempted to disable a clock source that is not running.
        #[error("Attempted to disable a clock source that is not running.")]
        EfClockSourceNotEnabled = 2554,
        /// A number greater than 16-bits was written to a clock source configured for 16-bits. Commonly occurs when an incorrect value is applied to CONFIG_A or CONFIG_B for an output feature such as PWM or pulse output.
        #[error("A number greater than 16-bits was written to a clock source configured for 16-bits. Commonly occurs when an incorrect value is applied to CONFIG_A or CONFIG_B for an output feature such as PWM or pulse output.")]
        Ef32bitDataInto16bitReg = 2555,
        #[error("EfSetTo32bit")]
        EfSetTo32bit = 2556,
        #[error("EfSmoothValueOutOfRange")]
        EfSmoothValueOutOfRange = 2557,
        /// Both Clock_Source 1 and Clock_Source 2 must be disabled before enabling Clock_Source 0.
        #[error("Both Clock_Source 1 and Clock_Source 2 must be disabled before enabling Clock_Source 0.")]
        Ef32bitRequiresBothClock0and1 = 2558,
        /// The specified divisor value is not supported. Supported values are: 1,2,4,8,16,32,64,256. This code was formerly named EF_PRESCALE_VALUE_INVALID
        #[error("The specified divisor value is not supported. Supported values are: 1,2,4,8,16,32,64,256. This code was formerly named EF_PRESCALE_VALUE_INVALID")]
        EfDivisorValueInvalid = 2559,
        /// The pin is already in use by another system.
        #[error("The pin is already in use by another system.")]
        EfPinReserved = 2560,
        /// The specified DIO_EF address is not supported on this device.
        #[error("The specified DIO_EF address is not supported on this device.")]
        EfInvalidDioNumber = 2561,
        /// The DIO line must be set to output low to ensure proper signal generation.
        #[error("The DIO line must be set to output low to ensure proper signal generation.")]
        EfLineMustBeLowBeforeStarting = 2563,
        #[error("EfInvalidDivisor")]
        EfInvalidDivisor = 2564,
        /// The DIO_EF_CONFIG value written determines a DIO line transition point relative to the count of a ClockSource. To prevent signal glitches, the value must be less than the selected ClockSource's roll value.
        #[error("The DIO_EF_CONFIG value written determines a DIO line transition point relative to the count of a ClockSource. To prevent signal glitches, the value must be less than the selected ClockSource's roll value.")]
        EfValueGreaterThanPeriod = 2565,
        /// The index of a DIO_EF can not be changed while that DIO_EF is enabled.
        #[error("The index of a DIO_EF can not be changed while that DIO_EF is enabled.")]
        EfCanNotChangeIndexWhileEnabled = 2566,
        /// The specified type index is not supported.
        #[error("The specified type index is not supported.")]
        AinEfInvalidType = 2580,
        /// Too many samples specified.
        #[error("Too many samples specified.")]
        AinEfInvalidNumSamples = 2581,
        #[error("AinEfCalculationError")]
        AinEfCalculationError = 2582,
        /// The AIN_EF channel has not been initialized. To initialize, set the index configuration to a non-zero value.
        #[error("The AIN_EF channel has not been initialized. To initialize, set the index configuration to a non-zero value.")]
        AinEfChannelInactive = 2583,
        /// The final value calculated for the AIN_EF is outside of the range the feature supports. This often indicates that there is some configuration issue.
        #[error("The final value calculated for the AIN_EF is outside of the range the feature supports. This often indicates that there is some configuration issue.")]
        AinEfCalculationOutOfRange = 2584,
        /// AIN_EF is not supported on the specified channel, or the specified channel does not exist.
        #[error("AIN_EF is not supported on the specified channel, or the specified channel does not exist.")]
        AinEfInvalidChannel = 2585,
        /// The register address specified for CJC measurement is not supported.
        #[error("The register address specified for CJC measurement is not supported.")]
        AinEfInvalidCjcRegister = 2586,
        /// Could not start the data collection stream.
        #[error("Could not start the data collection stream.")]
        AinEfStreamStartFailure = 2587,
        /// Failed to detect a period to perform calculations over.
        #[error("Failed to detect a period to perform calculations over.")]
        AinEfCouldNotFindPeriod = 2588,
        /// The selected AIN must be set to differential.
        #[error("The selected AIN must be set to differential.")]
        AinEfMustBeDifferential = 2589,
        /// The data collection time (number of samples / scan_rate) is too big. Limit is set to 180 ms.
        #[error("The data collection time (number of samples / scan_rate) is too big. Limit is set to 180 ms.")]
        AinEfScanTimeTooLong = 2590,
        /// The excitation circuit index specified is not valid, use a different excitation circuit.
        #[error("The excitation circuit index specified is not valid, use a different excitation circuit.")]
        AinEfInvalidExcitationIndex = 2591,
        /// The list of channels to stream is empty.
        #[error("The list of channels to stream is empty.")]
        StreamNeedAtLeastOneChn = 2600,
        /// The stream clock base is read only.
        #[error("The stream clock base is read only.")]
        StreamClockBaseNotWritable = 2601,
        #[error("StreamExtclkAndGateMx")]
        StreamExtclkAndGateMx = 2602,
        /// Stream data can not be read with commands while in spontaneous mode.
        #[error("Stream data can not be read with commands while in spontaneous mode.")]
        StreamInSpontaneousMode = 2603,
        /// STREAM_SAMPLES_PER_PACKET is set to a value greater than the USB interface can support.
        #[error("STREAM_SAMPLES_PER_PACKET is set to a value greater than the USB interface can support.")]
        StreamUsbPktOverflow = 2604,
        /// The requested operation can not be performed while stream is running.
        #[error("The requested operation can not be performed while stream is running.")]
        StreamIsActive = 2605,
        /// Stream resolution can not be greater than 8.
        #[error("Stream resolution can not be greater than 8.")]
        StreamConfigInvalid = 2606,
        /// The channel list contains an unstreamable address.
        #[error("The channel list contains an unstreamable address.")]
        StreamChnListInvalid = 2607,
        /// The scan rate times the number of channels per scan is too great for this device.
        #[error("The scan rate times the number of channels per scan is too great for this device.")]
        StreamScanRateInvalid = 2608,
        #[error("StreamOutBuffTooBig")]
        StreamOutBuffTooBig = 2609,
        /// An invalid stream out number was specified.
        #[error("An invalid stream out number was specified.")]
        StreamOutNumInvalid = 2610,
        /// An unsupported data types was specified.
        #[error("An unsupported data types was specified.")]
        StreamDataTypeInvalid = 2611,
        /// Stream must be set to either spontaneous or command-response.
        #[error("Stream must be set to either spontaneous or command-response.")]
        StreamTargetConfigInvalid = 2612,
        /// Attempted to write more data than the buffer can hold. Extra data was discarded.
        #[error("Attempted to write more data than the buffer can hold. Extra data was discarded.")]
        StreamOutBuffFull = 2613,
        /// The specified address cannot be a stream out target.
        #[error("The specified address cannot be a stream out target.")]
        StreamOutTargetInvalid = 2614,
        /// The specified buffer was either too large or was not a power of 2.
        #[error("The specified buffer was either too large or was not a power of 2.")]
        StreamBuffSizeInvalid = 2615,
        #[error("StreamOutBuffLoopOverwrite")]
        StreamOutBuffLoopOverwrite = 2616,
        /// The buffer size must be set before data can be written to it.
        #[error("The buffer size must be set before data can be written to it.")]
        StreamOutBuffDne = 2617,
        /// The specified number of samples per packet is too large.
        #[error("The specified number of samples per packet is too large.")]
        StreamSamplesPerPktInvalid = 2618,
        #[error("StreamBufferDne")]
        StreamBufferDne = 2619,
        /// Stream was already disabled.
        #[error("Stream was already disabled.")]
        StreamNotRunning = 2620,
        /// Specified settling time is greater than the max possible.
        #[error("Specified settling time is greater than the max possible.")]
        StreamSettlingInvalid = 2621,
        /// The loop size is too big for the current buffer size.
        #[error("The loop size is too big for the current buffer size.")]
        StreamOutLoopTooBig = 2622,
        /// There is a mismatch between the stream out buffer type and target register.
        #[error("There is a mismatch between the stream out buffer type and target register.")]
        StreamOutDataTrgtMissmatch = 2623,
        /// Selected divisor can not be used.
        #[error("Selected divisor can not be used.")]
        StreamInvalidDivisor = 2624,
        /// The requested channel can not be streamed.
        #[error("The requested channel can not be streamed.")]
        StreamChnCanNotBeStreamed = 2625,
        /// The high resolution converter cannot be used while stream out is used to update a DAC.
        #[error("The high resolution converter cannot be used while stream out is used to update a DAC.")]
        StreamOutDacInUse = 2626,
        /// The STREAM_OUT#_ENABLE register must be set to 1 before writing STREAM_OUT#_LOOP_NUM_VALUES, STREAM_OUT#_SET_LOOP, or any of the STREAM_OUT#_BUFFER_ registers.
        #[error("The STREAM_OUT#_ENABLE register must be set to 1 before writing STREAM_OUT#_LOOP_NUM_VALUES, STREAM_OUT#_SET_LOOP, or any of the STREAM_OUT#_BUFFER_ registers.")]
        StreamOutNeedsToBeEnabled = 2627,
        /// DAC1_FREQUENCY_OUT_ENABLE should be disabled before streaming at rates above 10kHz due to poor frequency output performance at higher stream rates.
        #[error("DAC1_FREQUENCY_OUT_ENABLE should be disabled before streaming at rates above 10kHz due to poor frequency output performance at higher stream rates.")]
        CannotStreamFastWithDac1FrequencyOutEnabled = 2628,
        /// DAC1_FREQUENCY_OUT_ENABLE should not be enabled while streaming at rates above 10kHz due to poor frequency output performance at higher stream rates.
        #[error("DAC1_FREQUENCY_OUT_ENABLE should not be enabled while streaming at rates above 10kHz due to poor frequency output performance at higher stream rates.")]
        CannotEnableDac1FrequencyOutWithFastStream = 2629,
        /// The stream trigger index is not valid for this device
        #[error("The stream trigger index is not valid for this device")]
        StreamTriggerIndexInvalid = 2630,
        /// Stream can only be run at a max of 250Hz when streaming TEMPERATURE(0:7) registers
        #[error("Stream can only be run at a max of 250Hz when streaming TEMPERATURE(0:7) registers")]
        StreamRateInvalidForCjc = 2631,
        /// The specified Software Watchdog timeout is too short.
        #[error("The specified Software Watchdog timeout is too short.")]
        SwdtRolltInvalid = 2670,
        /// The watchdog must be disabled before the requested operation can be performed.
        #[error("The watchdog must be disabled before the requested operation can be performed.")]
        SwdtEnabled = 2671,
        /// The specified Software Watchdog DIO configurations are not valid.
        #[error("The specified Software Watchdog DIO configurations are not valid.")]
        SwdtDioSettingsInvalid = 2672,
        /// The specified Software Watchdog DAC0 configurations are not valid.
        #[error("The specified Software Watchdog DAC0 configurations are not valid.")]
        SwdtDac0SettingsInvalid = 2673,
        /// The specified Software Watchdog DAC1 configurations are not valid.
        #[error("The specified Software Watchdog DAC1 configurations are not valid.")]
        SwdtDac1SettingsInvalid = 2674,
        #[error("RtcTimeInvalid")]
        RtcTimeInvalid = 2690,
        /// The specified SNTP update interval is too short.
        #[error("The specified SNTP update interval is too short.")]
        RtcSntpTimeInvalid = 2691,
        /// The requested operation can not be performed on units without a real-time-clock.
        #[error("The requested operation can not be performed on units without a real-time-clock.")]
        RtcNotPresent = 2692,
        /// Valid modes are 0-3.
        #[error("Valid modes are 0-3.")]
        SpiModeInvalid = 2700,
        /// SPI RX data is not available in the RX buffer.
        #[error("SPI RX data is not available in the RX buffer.")]
        SpiNoDataAvailable = 2701,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        SpiCsPinInvalid = 2702,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        SpiClkPinInvalid = 2703,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        SpiMisoPinInvalid = 2704,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        SpiMosiPinInvalid = 2705,
        /// Selected pin is not available.
        #[error("Selected pin is not available.")]
        SpiCsPinReserved = 2706,
        /// Selected pin is not available.
        #[error("Selected pin is not available.")]
        SpiClkPinReserved = 2707,
        /// Selected pin is not available.
        #[error("Selected pin is not available.")]
        SpiMisoPinReserved = 2708,
        /// Selected pin is not available.
        #[error("Selected pin is not available.")]
        SpiMosiPinReserved = 2709,
        /// The specified TX buffer size is too large.
        #[error("The specified TX buffer size is too large.")]
        SpiTransferSizeTooLarge = 2710,
        /// One or both of the I2C lines are held low. Check hardware and reset the bus.
        #[error("One or both of the I2C lines are held low. Check hardware and reset the bus.")]
        I2cBusBusy = 2720,
        /// Attempted to read from an empty buffer.
        #[error("Attempted to read from an empty buffer.")]
        I2cNoDataAvailable = 2721,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        I2cSdaPinInvalid = 2722,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        I2cSclPinInvalid = 2723,
        /// The selected pin is not available.
        #[error("The selected pin is not available.")]
        I2cSdaPinReserved = 2724,
        /// The selected pin is not available.
        #[error("The selected pin is not available.")]
        I2cSclPinReserved = 2725,
        /// The specified TX buffer size is too large.
        #[error("The specified TX buffer size is too large.")]
        I2cTxSizeTooLarge = 2726,
        /// The specified RX buffer size is too large.
        #[error("The specified RX buffer size is too large.")]
        I2cRxSizeTooLarge = 2727,
        /// The data that was attempted to be written to the I2C TX buffer exceeded the maximum TX buffer size. Only data up to the maximum buffer size was placed in the TX buffer.
        #[error("The data that was attempted to be written to the I2C TX buffer exceeded the maximum TX buffer size. Only data up to the maximum buffer size was placed in the TX buffer.")]
        I2cBufferOverrun = 2728,
        /// The throttle setting is too low, watchdog may fire. Minimum value = 46000
        #[error("The throttle setting is too low, watchdog may fire. Minimum value = 46000")]
        I2cSpeedTooLow = 2729,
        /// Slave device did not respond.
        #[error("Slave device did not respond.")]
        SbusCommTimeOut = 2740,
        /// Slave device did not acknowledge the data transfer.
        #[error("Slave device did not acknowledge the data transfer.")]
        SbusNoAck = 2741,
        #[error("SbusCustomModeInvalid")]
        SbusCustomModeInvalid = 2742,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        SbusInvalidDioNum = 2743,
        /// Command-response reads can not be used while the background service is running.
        #[error("Command-response reads can not be used while the background service is running.")]
        SbusBackgroundServiceOn = 2744,
        /// SHT communication checksum failed.
        #[error("SHT communication checksum failed.")]
        SbusChecksumError = 2745,
        /// SCL must be even and SDA must be SCL+1.
        #[error("SCL must be even and SDA must be SCL+1.")]
        TdacSdaSclInvalid = 2760,
        /// Attempted to set an invalid pin.
        #[error("Attempted to set an invalid pin.")]
        TdacSclInvalid = 2761,
        /// The specified channel not supported on this device.
        #[error("The specified channel not supported on this device.")]
        TdacInvalidChannel = 2762,
        /// Failed to read the TDAC calibration.
        #[error("Failed to read the TDAC calibration.")]
        TdacCalReadFailure = 2763,
        /// The TDAC did not respond to communication attempts.
        #[error("The TDAC did not respond to communication attempts.")]
        TdacNotFound = 2764,
        /// A TDAC has not been initialized, try writing to the the TDAC first with the TDAC# register.
        #[error("A TDAC has not been initialized, try writing to the the TDAC first with the TDAC# register.")]
        TdacNotInitialized = 2765,
        /// Unknown function specified.
        #[error("Unknown function specified.")]
        OnewireUnsupportedFunction = 2770,
        /// Unable to detect any devices on the bus.
        #[error("Unable to detect any devices on the bus.")]
        OnewireNoPresencePulse = 2771,
        /// The specified number of data bits is not supported.
        #[error("The specified number of data bits is not supported.")]
        AsynchNumDataBitsInvalid = 2780,
        /// The number of bytes to send is invalid.
        #[error("The number of bytes to send is invalid.")]
        AsynchNumToWriteInvalid = 2781,
        /// The specified buffer size is invalid. Max is 2048.
        #[error("The specified buffer size is invalid. Max is 2048.")]
        AsynchReadBuffSizeInvalid = 2782,
        /// The baud rate is too high for this device.
        #[error("The baud rate is too high for this device.")]
        AsynchBaudTooHigh = 2783,
        /// The specified operation can not be performed while enabled.
        #[error("The specified operation can not be performed while enabled.")]
        AsynchIsEnabled = 2784,
        /// The specified operation can not be performed while disabled.
        #[error("The specified operation can not be performed while disabled.")]
        AsynchIsNotEnabled = 2785,
        /// The transmit buffer is full.
        #[error("The transmit buffer is full.")]
        AsynchTxBufferFull = 2786,
        /// Transmission timed out. Do not write more than 100 ms at a time.
        #[error("Transmission timed out. Do not write more than 100 ms at a time.")]
        AsynchTxTimeout = 2787,
        /// The baud rate is zero. Please specify a baud rate.
        #[error("The baud rate is zero. Please specify a baud rate.")]
        AsynchBaudZero = 2788,
        /// A hard error occurred in the low level disk I/O layer.
        #[error("A hard error occurred in the low level disk I/O layer.")]
        FileIoDiskError = 2801,
        /// Assertion failed.
        #[error("Assertion failed.")]
        FileIoInternalError = 2802,
        /// The physical drive cannot work.
        #[error("The physical drive cannot work.")]
        FileIoNotReady = 2803,
        /// Could not find the file.
        #[error("Could not find the file.")]
        FileIoNoFile = 2804,
        /// Could not find the path.
        #[error("Could not find the path.")]
        FileIoNoPath = 2805,
        /// The path name format is invalid.
        #[error("The path name format is invalid.")]
        FileIoPathNameInvalid = 2806,
        /// Access denied due to prohibited access or the directory is full.
        #[error("Access denied due to prohibited access or the directory is full.")]
        FileIoDenied = 2807,
        /// Access denied due to prohibited access.
        #[error("Access denied due to prohibited access.")]
        FileIoExist = 2808,
        /// The file/directory object is invalid. In the context of performing an ls command, this indicates that there are no more files.
        #[error("The file/directory object is invalid. In the context of performing an ls command, this indicates that there are no more files.")]
        FileIoInvalidObject = 2809,
        /// The physical drive is write protected.
        #[error("The physical drive is write protected.")]
        FileIoWriteProtected = 2810,
        /// The logical drive number is invalid.
        #[error("The logical drive number is invalid.")]
        FileIoInvalidDrive = 2811,
        /// The volume has no work area.
        #[error("The volume has no work area.")]
        FileIoNotEnabled = 2812,
        /// There is no valid FAT12, FAT16, or FAT32 volume.
        #[error("There is no valid FAT12, FAT16, or FAT32 volume.")]
        FileIoNoFilesystem = 2813,
        /// The f_mkfs() function aborted due to any parameter error.
        #[error("The f_mkfs() function aborted due to any parameter error.")]
        FileIoMkfsAborted = 2814,
        /// Could not get granted access to the volume within the defined timeout period.
        #[error("Could not get granted access to the volume within the defined timeout period.")]
        FileIoTimeout = 2815,
        /// The operation is rejected according to the file sharing policy.
        #[error("The operation is rejected according to the file sharing policy.")]
        FileIoLocked = 2816,
        /// LFN working buffer could not be allocated.
        #[error("LFN working buffer could not be allocated.")]
        FileIoNotEnoughCore = 2817,
        /// The number of open files is greater than the allowable limit (files > _FS_SHARE).
        #[error("The number of open files is greater than the allowable limit (files > _FS_SHARE).")]
        FileIoTooManyOpenFiles = 2818,
        /// The given parameter is invalid.
        #[error("The given parameter is invalid.")]
        FileIoInvalidParameter = 2819,
        /// The WiFi module associated to the network.
        #[error("The WiFi module associated to the network.")]
        WifiAssociated = 2900,
        /// The WiFi module is attempting to associate to the network.
        #[error("The WiFi module is attempting to associate to the network.")]
        WifiAssociating = 2901,
        /// The WiFi module failed to associate to the network.
        #[error("The WiFi module failed to associate to the network.")]
        WifiAssociationFailed = 2902,
        /// The WiFi module is not currently powered.
        #[error("The WiFi module is not currently powered.")]
        WifiUnpowered = 2903,
        /// The WiFi module is initializing.
        #[error("The WiFi module is initializing.")]
        WifiBootingUp = 2904,
        /// The WiFi module was unable to properly initialize.
        #[error("The WiFi module was unable to properly initialize.")]
        WifiCouldNotStart = 2905,
        /// The WiFi module is attempting to apply the desired network settings.
        #[error("The WiFi module is attempting to apply the desired network settings.")]
        WifiApplyingSettings = 2906,
        /// The WiFi module has begun the process of claiming a DHCP lease.
        #[error("The WiFi module has begun the process of claiming a DHCP lease.")]
        WifiDhcpStarted = 2907,
        /// The WiFi module is in an unspecified state.
        #[error("The WiFi module is in an unspecified state.")]
        WifiOther = 2909,
        /// The WiFi module is getting ready to start an update.
        #[error("The WiFi module is getting ready to start an update.")]
        WifiUpdateConfig = 2920,
        /// The WiFi module is in the process of updating.
        #[error("The WiFi module is in the process of updating.")]
        WifiUpdateInProg = 2921,
        /// The WiFi module has updated and will restart to apply the new changes.
        #[error("The WiFi module has updated and will restart to apply the new changes.")]
        WifiUpdateReboot = 2923,
        /// The WiFi module was successfully updated.
        #[error("The WiFi module was successfully updated.")]
        WifiUpdateSuccess = 2924,
        /// The WiFi module was not successfully updated.
        #[error("The WiFi module was not successfully updated.")]
        WifiUpdateFailed = 2925,
        /// The stream buffer reached its capacity and auto-recovery has begun to avoid an overflow. No new samples will be saved until there is free space in the buffer.
        #[error("The stream buffer reached its capacity and auto-recovery has begun to avoid an overflow. No new samples will be saved until there is free space in the buffer.")]
        StreamAutoRecoverActive = 2940,
        /// There is space available in the stream buffer for new samples again and auto-recovery has ended.
        #[error("There is space available in the stream buffer for new samples again and auto-recovery has ended.")]
        StreamAutoRecoverEnd = 2941,
        /// A new scan started before the previous scan finished. Generally occurs because ScanRate > MaxSampleRate/NumChannels.  Note that MaxSampleRate is impacted by Range, ResolutionIndex, and Settling. Try adding commands right before StreamStart to set AIN_ALL_RANGE=10, STREAM_RESOLUTION_INDEX=0, and STREAM_SETTLING_US=0.
        #[error("A new scan started before the previous scan finished. Generally occurs because ScanRate > MaxSampleRate/NumChannels.  Note that MaxSampleRate is impacted by Range, ResolutionIndex, and Settling. Try adding commands right before StreamStart to set AIN_ALL_RANGE=10, STREAM_RESOLUTION_INDEX=0, and STREAM_SETTLING_US=0.")]
        StreamScanOverlap = 2942,
        /// During stream auto-recovery, the variable tracking the number of skipped scans has reached an overflow condition.
        #[error("During stream auto-recovery, the variable tracking the number of skipped scans has reached an overflow condition.")]
        StreamAutoRecoverEndOverflow = 2943,
        /// The specified number of stream scans have been acquired, stream will be stopped automatically.
        #[error("The specified number of stream scans have been acquired, stream will be stopped automatically.")]
        StreamBurstComplete = 2944,
        /// The stream buffer reached capacity while auto-recovery was disabled. Stream has been stopped.
        #[error("The stream buffer reached capacity while auto-recovery was disabled. Stream has been stopped.")]
        StreamBufferFull = 2945,
        #[error("SelfdiagMainOscFail")]
        SelfdiagMainOscFail = 2950,
        /// The requested file was not found.
        #[error("The requested file was not found.")]
        FileIoNotFound = 2960,
        /// No SD card present or SC card could not be initialized.
        #[error("No SD card present or SC card could not be initialized.")]
        FileIoNoDisk = 2961,
        /// The file name is invalid.
        #[error("The file name is invalid.")]
        FileIoInvalidName = 2962,
        /// An open file is required to perform the requested operation.
        #[error("An open file is required to perform the requested operation.")]
        FileIoFileNotOpen = 2963,
        /// There are too many files open.
        #[error("There are too many files open.")]
        FileIoTooManyOpen = 2964,
        /// Failed to mount the SD card. Card may be bad or incompatible.
        #[error("Failed to mount the SD card. Card may be bad or incompatible.")]
        FileIoSdCardNotFound = 2965,
        /// There are no more files in the current working directory.
        #[error("There are no more files in the current working directory.")]
        FileIoEndOfCwd = 2966,
    }
}
