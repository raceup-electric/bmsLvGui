// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"can2.dbc"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Messages {
    /// Arduino1
    Arduino1(Arduino1),
    /// LVError
    LvError(LvError),
    /// ResGO
    ResGo(ResGo),
    /// EbsStatus
    EbsStatus(EbsStatus),
    /// Asms
    Asms(Asms),
    /// CarMission
    CarMission(CarMission),
    /// PcuFault
    PcuFault(PcuFault),
    /// Paddle
    Paddle(Paddle),
    /// Driver
    Driver(Driver),
    /// BmsLv1
    BmsLv1(BmsLv1),
    /// BmsLv2
    BmsLv2(BmsLv2),
    /// BmsHv1
    BmsHv1(BmsHv1),
    /// BmsHv2
    BmsHv2(BmsHv2),
    /// Imu1
    Imu1(Imu1),
    /// Imu2
    Imu2(Imu2),
    /// Imu3
    Imu3(Imu3),
    /// IMUCalib
    ImuCalib(ImuCalib),
    /// Map
    Map(Map),
    /// CarStatus
    CarStatus(CarStatus),
    /// CarSettings
    CarSettings(CarSettings),
    /// CheckASBReq
    CheckAsbReq(CheckAsbReq),
    /// EbsBrakeReq
    EbsBrakeReq(EbsBrakeReq),
    /// ResStatus
    ResStatus(ResStatus),
    /// LapStart
    LapStart(LapStart),
    /// CarMissionStatus
    CarMissionStatus(CarMissionStatus),
    /// Imu4
    Imu4(Imu4),
    /// Imu5
    Imu5(Imu5),
    /// Temp1
    Temp1(Temp1),
    /// Temp2
    Temp2(Temp2),
    /// SuspRear
    SuspRear(SuspRear),
    /// RESERVED2
    Reserved2(Reserved2),
    /// SuspFront
    SuspFront(SuspFront),
    /// TempFrontR
    TempFrontR(TempFrontR),
    /// HydraulicPressure
    HydraulicPressure(HydraulicPressure),
    /// TempImu
    TempImu(TempImu),
    /// InvVolt
    InvVolt(InvVolt),
    /// Pcu
    Pcu(Pcu),
    /// Calib
    Calib(Calib),
    /// CalibAck
    CalibAck(CalibAck),
    /// PcuSwControl
    PcuSwControl(PcuSwControl),
    /// PcuRfAck
    PcuRfAck(PcuRfAck),
    /// EmbeddedAliveCheck
    EmbeddedAliveCheck(EmbeddedAliveCheck),
    /// CalibRegen
    CalibRegen(CalibRegen),
    /// CalibRegenAck
    CalibRegenAck(CalibRegenAck),
    /// PcuAdc1
    PcuAdc1(PcuAdc1),
    /// PcuAdc2
    PcuAdc2(PcuAdc2),
    /// PcuAdc3
    PcuAdc3(PcuAdc3),
    /// coolingControl
    CoolingControl(CoolingControl),
    /// Balancing
    Balancing(Balancing),
    /// DisplayACK
    DisplayAck(DisplayAck),
    /// MapAck
    MapAck(MapAck),
    /// VcuErrTrace
    VcuErrTrace(VcuErrTrace),
    /// CsLog_1
    CsLog1(CsLog1),
    /// CsLog_2
    CsLog2(CsLog2),
    /// CsLog_3
    CsLog3(CsLog3),
    /// BmsLvCellControl
    BmsLvCellControl(BmsLvCellControl),
    /// BmsLvCell_1
    BmsLvCell1(BmsLvCell1),
    /// BmsLvCell_2
    BmsLvCell2(BmsLvCell2),
    /// BmsLvCell_3
    BmsLvCell3(BmsLvCell3),
    /// BmsLvTemps
    BmsLvTemps(BmsLvTemps),
    /// Lem
    Lem(Lem),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            8 => Messages::Arduino1(Arduino1::try_from(payload)?),
            20 => Messages::LvError(LvError::try_from(payload)?),
            32 => Messages::ResGo(ResGo::try_from(payload)?),
            60 => Messages::EbsStatus(EbsStatus::try_from(payload)?),
            65 => Messages::Asms(Asms::try_from(payload)?),
            71 => Messages::CarMission(CarMission::try_from(payload)?),
            81 => Messages::PcuFault(PcuFault::try_from(payload)?),
            82 => Messages::Paddle(Paddle::try_from(payload)?),
            83 => Messages::Driver(Driver::try_from(payload)?),
            84 => Messages::BmsLv1(BmsLv1::try_from(payload)?),
            85 => Messages::BmsLv2(BmsLv2::try_from(payload)?),
            87 => Messages::BmsHv1(BmsHv1::try_from(payload)?),
            88 => Messages::BmsHv2(BmsHv2::try_from(payload)?),
            96 => Messages::Imu1(Imu1::try_from(payload)?),
            97 => Messages::Imu2(Imu2::try_from(payload)?),
            98 => Messages::Imu3(Imu3::try_from(payload)?),
            99 => Messages::ImuCalib(ImuCalib::try_from(payload)?),
            100 => Messages::Map(Map::try_from(payload)?),
            101 => Messages::CarStatus(CarStatus::try_from(payload)?),
            102 => Messages::CarSettings(CarSettings::try_from(payload)?),
            104 => Messages::CheckAsbReq(CheckAsbReq::try_from(payload)?),
            105 => Messages::EbsBrakeReq(EbsBrakeReq::try_from(payload)?),
            106 => Messages::ResStatus(ResStatus::try_from(payload)?),
            112 => Messages::LapStart(LapStart::try_from(payload)?),
            113 => Messages::CarMissionStatus(CarMissionStatus::try_from(payload)?),
            128 => Messages::Imu4(Imu4::try_from(payload)?),
            129 => Messages::Imu5(Imu5::try_from(payload)?),
            256 => Messages::Temp1(Temp1::try_from(payload)?),
            257 => Messages::Temp2(Temp2::try_from(payload)?),
            258 => Messages::SuspRear(SuspRear::try_from(payload)?),
            259 => Messages::Reserved2(Reserved2::try_from(payload)?),
            260 => Messages::SuspFront(SuspFront::try_from(payload)?),
            261 => Messages::TempFrontR(TempFrontR::try_from(payload)?),
            264 => Messages::HydraulicPressure(HydraulicPressure::try_from(payload)?),
            277 => Messages::TempImu(TempImu::try_from(payload)?),
            288 => Messages::InvVolt(InvVolt::try_from(payload)?),
            304 => Messages::Pcu(Pcu::try_from(payload)?),
            305 => Messages::Calib(Calib::try_from(payload)?),
            306 => Messages::CalibAck(CalibAck::try_from(payload)?),
            307 => Messages::PcuSwControl(PcuSwControl::try_from(payload)?),
            308 => Messages::PcuRfAck(PcuRfAck::try_from(payload)?),
            310 => Messages::EmbeddedAliveCheck(EmbeddedAliveCheck::try_from(payload)?),
            328 => Messages::CalibRegen(CalibRegen::try_from(payload)?),
            329 => Messages::CalibRegenAck(CalibRegenAck::try_from(payload)?),
            331 => Messages::PcuAdc1(PcuAdc1::try_from(payload)?),
            332 => Messages::PcuAdc2(PcuAdc2::try_from(payload)?),
            333 => Messages::PcuAdc3(PcuAdc3::try_from(payload)?),
            334 => Messages::CoolingControl(CoolingControl::try_from(payload)?),
            420 => Messages::Balancing(Balancing::try_from(payload)?),
            600 => Messages::DisplayAck(DisplayAck::try_from(payload)?),
            700 => Messages::MapAck(MapAck::try_from(payload)?),
            800 => Messages::VcuErrTrace(VcuErrTrace::try_from(payload)?),
            820 => Messages::CsLog1(CsLog1::try_from(payload)?),
            821 => Messages::CsLog2(CsLog2::try_from(payload)?),
            822 => Messages::CsLog3(CsLog3::try_from(payload)?),
            869 => Messages::BmsLvCellControl(BmsLvCellControl::try_from(payload)?),
            870 => Messages::BmsLvCell1(BmsLvCell1::try_from(payload)?),
            871 => Messages::BmsLvCell2(BmsLvCell2::try_from(payload)?),
            872 => Messages::BmsLvCell3(BmsLvCell3::try_from(payload)?),
            873 => Messages::BmsLvTemps(BmsLvTemps::try_from(payload)?),
            962 => Messages::Lem(Lem::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// Arduino1
///
/// - ID: 8 (0x8)
/// - Size: 8 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct Arduino1 {
    raw: [u8; 8],
}

impl Arduino1 {
    pub const MESSAGE_ID: u32 = 8;
    
    pub const TEMP_1_MIN: i32 = 0_i32;
    pub const TEMP_1_MAX: i32 = 0_i32;
    pub const TEMP_2_MIN: i32 = 0_i32;
    pub const TEMP_2_MAX: i32 = 0_i32;
    
    /// Construct new Arduino1 from values
    pub fn new(temp_1: i32, temp_2: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_temp_1(temp_1)?;
        res.set_temp_2(temp_2)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// temp_1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_1(&self) -> i32 {
        self.temp_1_raw()
    }
    
    /// Get raw value of temp_1
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn temp_1_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of temp_1
    #[inline(always)]
    pub fn set_temp_1(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 8 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// temp_2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_2(&self) -> i32 {
        self.temp_2_raw()
    }
    
    /// Get raw value of temp_2
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn temp_2_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of temp_2
    #[inline(always)]
    pub fn set_temp_2(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 8 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Arduino1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Arduino1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Arduino1")
                .field("temp_1", &self.temp_1())
                .field("temp_2", &self.temp_2())
            .finish()
        } else {
            f.debug_tuple("Arduino1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Arduino1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let temp_1 = u.int_in_range(0..=0)?;
        let temp_2 = u.int_in_range(0..=0)?;
        Arduino1::new(temp_1,temp_2).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// LVError
///
/// - ID: 20 (0x14)
/// - Size: 1 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct LvError {
    raw: [u8; 1],
}

impl LvError {
    pub const MESSAGE_ID: u32 = 20;
    
    pub const ERROR_LV_MIN: u8 = 0_u8;
    pub const ERROR_LV_MAX: u8 = 1_u8;
    
    /// Construct new LVError from values
    pub fn new(error_lv: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_error_lv(error_lv)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// error_lv
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn error_lv(&self) -> LvErrorErrorLv {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            1 => LvErrorErrorLv::Error,
            0 => LvErrorErrorLv::NoError,
            _ => LvErrorErrorLv::_Other(self.error_lv_raw()),
        }
    }
    
    /// Get raw value of error_lv
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn error_lv_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of error_lv
    #[inline(always)]
    pub fn set_error_lv(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 20 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for LvError {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for LvError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("LvError")
                .field("error_lv", &self.error_lv())
            .finish()
        } else {
            f.debug_tuple("LvError").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for LvError {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let error_lv = u.int_in_range(0..=1)?;
        LvError::new(error_lv).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for error_lv
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum LvErrorErrorLv {
    Error,
    NoError,
    _Other(u8),
}

impl From<LvErrorErrorLv> for u8 {
    fn from(val: LvErrorErrorLv) -> u8 {
        match val {
            LvErrorErrorLv::Error => 1,
            LvErrorErrorLv::NoError => 0,
            LvErrorErrorLv::_Other(x) => x,
        }
    }
}


/// ResGO
///
/// - ID: 32 (0x20)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct ResGo {
    raw: [u8; 1],
}

impl ResGo {
    pub const MESSAGE_ID: u32 = 32;
    
    
    /// Construct new ResGO from values
    pub fn new(go_signal: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_go_signal(go_signal)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// go_signal
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn go_signal(&self) -> bool {
        self.go_signal_raw()
    }
    
    /// Get raw value of go_signal
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn go_signal_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of go_signal
    #[inline(always)]
    pub fn set_go_signal(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ResGo {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for ResGo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("ResGo")
                .field("go_signal", &self.go_signal())
            .finish()
        } else {
            f.debug_tuple("ResGo").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for ResGo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let go_signal = u.int_in_range(0..=1)? == 1;
        ResGo::new(go_signal).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// EbsStatus
///
/// - ID: 60 (0x3c)
/// - Size: 5 bytes
/// - Transmitter: EBS
#[derive(Clone, Copy)]
pub struct EbsStatus {
    raw: [u8; 5],
}

impl EbsStatus {
    pub const MESSAGE_ID: u32 = 60;
    
    pub const PRESS_LEFT_TANK_MIN: f32 = 0_f32;
    pub const PRESS_LEFT_TANK_MAX: f32 = 10_f32;
    pub const PRESS_RIGHT_TANK_MIN: f32 = 0_f32;
    pub const PRESS_RIGHT_TANK_MAX: f32 = 10_f32;
    
    /// Construct new EbsStatus from values
    pub fn new(system_check: bool, sanity_left_sensor: bool, sanity_right_sensor: bool, asb_check: bool, brakes_engaged: bool, brake_consistency: bool, tank_brake_coherence: bool, xnot_in_use: bool, press_left_tank: f32, press_right_tank: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 5] };
        res.set_system_check(system_check)?;
        res.set_sanity_left_sensor(sanity_left_sensor)?;
        res.set_sanity_right_sensor(sanity_right_sensor)?;
        res.set_asb_check(asb_check)?;
        res.set_brakes_engaged(brakes_engaged)?;
        res.set_brake_consistency(brake_consistency)?;
        res.set_tank_brake_coherence(tank_brake_coherence)?;
        res.set_xnot_in_use(xnot_in_use)?;
        res.set_press_left_tank(press_left_tank)?;
        res.set_press_right_tank(press_right_tank)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 5] {
        &self.raw
    }
    
    /// system_check
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn system_check(&self) -> bool {
        self.system_check_raw()
    }
    
    /// Get raw value of system_check
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn system_check_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of system_check
    #[inline(always)]
    pub fn set_system_check(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// sanity_left_sensor
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sanity_left_sensor(&self) -> bool {
        self.sanity_left_sensor_raw()
    }
    
    /// Get raw value of sanity_left_sensor
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sanity_left_sensor_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of sanity_left_sensor
    #[inline(always)]
    pub fn set_sanity_left_sensor(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
    /// sanity_right_sensor
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sanity_right_sensor(&self) -> bool {
        self.sanity_right_sensor_raw()
    }
    
    /// Get raw value of sanity_right_sensor
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sanity_right_sensor_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of sanity_right_sensor
    #[inline(always)]
    pub fn set_sanity_right_sensor(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }
    
    /// ASB_check
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn asb_check(&self) -> bool {
        self.asb_check_raw()
    }
    
    /// Get raw value of ASB_check
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn asb_check_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ASB_check
    #[inline(always)]
    pub fn set_asb_check(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }
    
    /// brakes_engaged
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brakes_engaged(&self) -> bool {
        self.brakes_engaged_raw()
    }
    
    /// Get raw value of brakes_engaged
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brakes_engaged_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of brakes_engaged
    #[inline(always)]
    pub fn set_brakes_engaged(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// brake_consistency
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_consistency(&self) -> bool {
        self.brake_consistency_raw()
    }
    
    /// Get raw value of brake_consistency
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_consistency_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of brake_consistency
    #[inline(always)]
    pub fn set_brake_consistency(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
    /// tank_brake_coherence
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_brake_coherence(&self) -> bool {
        self.tank_brake_coherence_raw()
    }
    
    /// Get raw value of tank_brake_coherence
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_brake_coherence_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of tank_brake_coherence
    #[inline(always)]
    pub fn set_tank_brake_coherence(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }
    
    /// _NOT_IN_USE
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn xnot_in_use(&self) -> bool {
        self.xnot_in_use_raw()
    }
    
    /// Get raw value of _NOT_IN_USE
    ///
    /// - Start bit: 7
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn xnot_in_use_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[7..8].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of _NOT_IN_USE
    #[inline(always)]
    pub fn set_xnot_in_use(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[7..8].store_le(value);
        Ok(())
    }
    
    /// press_left_tank
    ///
    /// - Min: 0
    /// - Max: 10
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn press_left_tank(&self) -> f32 {
        self.press_left_tank_raw()
    }
    
    /// Get raw value of press_left_tank
    ///
    /// - Start bit: 8
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_left_tank_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[8..24].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of press_left_tank
    #[inline(always)]
    pub fn set_press_left_tank(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 60 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[8..24].store_le(value);
        Ok(())
    }
    
    /// press_right_tank
    ///
    /// - Min: 0
    /// - Max: 10
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn press_right_tank(&self) -> f32 {
        self.press_right_tank_raw()
    }
    
    /// Get raw value of press_right_tank
    ///
    /// - Start bit: 24
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_right_tank_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[24..40].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of press_right_tank
    #[inline(always)]
    pub fn set_press_right_tank(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 10_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 60 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[24..40].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EbsStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 5 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 5];
        raw.copy_from_slice(&payload[..5]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for EbsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("EbsStatus")
                .field("system_check", &self.system_check())
                .field("sanity_left_sensor", &self.sanity_left_sensor())
                .field("sanity_right_sensor", &self.sanity_right_sensor())
                .field("asb_check", &self.asb_check())
                .field("brakes_engaged", &self.brakes_engaged())
                .field("brake_consistency", &self.brake_consistency())
                .field("tank_brake_coherence", &self.tank_brake_coherence())
                .field("xnot_in_use", &self.xnot_in_use())
                .field("press_left_tank", &self.press_left_tank())
                .field("press_right_tank", &self.press_right_tank())
            .finish()
        } else {
            f.debug_tuple("EbsStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for EbsStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let system_check = u.int_in_range(0..=1)? == 1;
        let sanity_left_sensor = u.int_in_range(0..=1)? == 1;
        let sanity_right_sensor = u.int_in_range(0..=1)? == 1;
        let asb_check = u.int_in_range(0..=1)? == 1;
        let brakes_engaged = u.int_in_range(0..=1)? == 1;
        let brake_consistency = u.int_in_range(0..=1)? == 1;
        let tank_brake_coherence = u.int_in_range(0..=1)? == 1;
        let xnot_in_use = u.int_in_range(0..=1)? == 1;
        let press_left_tank = u.float_in_range(0_f32..=10_f32)?;
        let press_right_tank = u.float_in_range(0_f32..=10_f32)?;
        EbsStatus::new(system_check,sanity_left_sensor,sanity_right_sensor,asb_check,brakes_engaged,brake_consistency,tank_brake_coherence,xnot_in_use,press_left_tank,press_right_tank).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Asms
///
/// - ID: 65 (0x41)
/// - Size: 1 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct Asms {
    raw: [u8; 1],
}

impl Asms {
    pub const MESSAGE_ID: u32 = 65;
    
    
    /// Construct new Asms from values
    pub fn new(asms_sens: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_asms_sens(asms_sens)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Asms_sens
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "High"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn asms_sens(&self) -> bool {
        self.asms_sens_raw()
    }
    
    /// Get raw value of Asms_sens
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn asms_sens_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of Asms_sens
    #[inline(always)]
    pub fn set_asms_sens(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Asms {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Asms {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Asms")
                .field("asms_sens", &self.asms_sens())
            .finish()
        } else {
            f.debug_tuple("Asms").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Asms {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let asms_sens = u.int_in_range(0..=1)? == 1;
        Asms::new(asms_sens).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CarMission
///
/// - ID: 71 (0x47)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct CarMission {
    raw: [u8; 1],
}

impl CarMission {
    pub const MESSAGE_ID: u32 = 71;
    
    pub const MISSION_MIN: u8 = 0_u8;
    pub const MISSION_MAX: u8 = 7_u8;
    
    /// Construct new CarMission from values
    pub fn new(mission: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_mission(mission)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Mission
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn mission(&self) -> CarMissionMission {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        match signal {
            7 => CarMissionMission::DvInspection,
            6 => CarMissionMission::DvEbsTest,
            5 => CarMissionMission::DvTrackdrive,
            4 => CarMissionMission::DvAutocross,
            3 => CarMissionMission::DvSkidpad,
            2 => CarMissionMission::DvAcceleration,
            1 => CarMissionMission::Manualy,
            0 => CarMissionMission::None,
            _ => CarMissionMission::_Other(self.mission_raw()),
        }
    }
    
    /// Get raw value of Mission
    ///
    /// - Start bit: 0
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mission_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        signal
    }
    
    /// Set value of Mission
    #[inline(always)]
    pub fn set_mission(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 71 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..3].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarMission {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CarMission {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CarMission")
                .field("mission", &self.mission())
            .finish()
        } else {
            f.debug_tuple("CarMission").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CarMission {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mission = u.int_in_range(0..=7)?;
        CarMission::new(mission).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for Mission
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CarMissionMission {
    DvInspection,
    DvEbsTest,
    DvTrackdrive,
    DvAutocross,
    DvSkidpad,
    DvAcceleration,
    Manualy,
    None,
    _Other(u8),
}

impl From<CarMissionMission> for u8 {
    fn from(val: CarMissionMission) -> u8 {
        match val {
            CarMissionMission::DvInspection => 7,
            CarMissionMission::DvEbsTest => 6,
            CarMissionMission::DvTrackdrive => 5,
            CarMissionMission::DvAutocross => 4,
            CarMissionMission::DvSkidpad => 3,
            CarMissionMission::DvAcceleration => 2,
            CarMissionMission::Manualy => 1,
            CarMissionMission::None => 0,
            CarMissionMission::_Other(x) => x,
        }
    }
}


/// PcuFault
///
/// - ID: 81 (0x51)
/// - Size: 1 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuFault {
    raw: [u8; 1],
}

impl PcuFault {
    pub const MESSAGE_ID: u32 = 81;
    
    
    /// Construct new PcuFault from values
    pub fn new(fault_12v: bool, fault_dv: bool, fault_24v: bool, fault_pumpl: bool, fault_pumpr: bool, fault_fanbattr: bool, fault_fanbattl: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_fault_12v(fault_12v)?;
        res.set_fault_dv(fault_dv)?;
        res.set_fault_24v(fault_24v)?;
        res.set_fault_pumpl(fault_pumpl)?;
        res.set_fault_pumpr(fault_pumpr)?;
        res.set_fault_fanbattr(fault_fanbattr)?;
        res.set_fault_fanbattl(fault_fanbattl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// fault_12v
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_12v(&self) -> bool {
        self.fault_12v_raw()
    }
    
    /// Get raw value of fault_12v
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_12v_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_12v
    #[inline(always)]
    pub fn set_fault_12v(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// fault_dv
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_dv(&self) -> bool {
        self.fault_dv_raw()
    }
    
    /// Get raw value of fault_dv
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_dv_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_dv
    #[inline(always)]
    pub fn set_fault_dv(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
    /// fault_24v
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_24v(&self) -> bool {
        self.fault_24v_raw()
    }
    
    /// Get raw value of fault_24v
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_24v_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_24v
    #[inline(always)]
    pub fn set_fault_24v(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }
    
    /// fault_pumpl
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_pumpl(&self) -> bool {
        self.fault_pumpl_raw()
    }
    
    /// Get raw value of fault_pumpl
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_pumpl_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_pumpl
    #[inline(always)]
    pub fn set_fault_pumpl(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }
    
    /// fault_pumpr
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_pumpr(&self) -> bool {
        self.fault_pumpr_raw()
    }
    
    /// Get raw value of fault_pumpr
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_pumpr_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_pumpr
    #[inline(always)]
    pub fn set_fault_pumpr(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// fault_fanbattr
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_fanbattr(&self) -> bool {
        self.fault_fanbattr_raw()
    }
    
    /// Get raw value of fault_fanbattr
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_fanbattr_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_fanbattr
    #[inline(always)]
    pub fn set_fault_fanbattr(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
    /// fault_fanbattl
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_fanbattl(&self) -> bool {
        self.fault_fanbattl_raw()
    }
    
    /// Get raw value of fault_fanbattl
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_fanbattl_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_fanbattl
    #[inline(always)]
    pub fn set_fault_fanbattl(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuFault {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for PcuFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PcuFault")
                .field("fault_12v", &self.fault_12v())
                .field("fault_dv", &self.fault_dv())
                .field("fault_24v", &self.fault_24v())
                .field("fault_pumpl", &self.fault_pumpl())
                .field("fault_pumpr", &self.fault_pumpr())
                .field("fault_fanbattr", &self.fault_fanbattr())
                .field("fault_fanbattl", &self.fault_fanbattl())
            .finish()
        } else {
            f.debug_tuple("PcuFault").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PcuFault {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let fault_12v = u.int_in_range(0..=1)? == 1;
        let fault_dv = u.int_in_range(0..=1)? == 1;
        let fault_24v = u.int_in_range(0..=1)? == 1;
        let fault_pumpl = u.int_in_range(0..=1)? == 1;
        let fault_pumpr = u.int_in_range(0..=1)? == 1;
        let fault_fanbattr = u.int_in_range(0..=1)? == 1;
        let fault_fanbattl = u.int_in_range(0..=1)? == 1;
        PcuFault::new(fault_12v,fault_dv,fault_24v,fault_pumpl,fault_pumpr,fault_fanbattr,fault_fanbattl).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Paddle
///
/// - ID: 82 (0x52)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct Paddle {
    raw: [u8; 1],
}

impl Paddle {
    pub const MESSAGE_ID: u32 = 82;
    
    pub const REGEN_MIN: u8 = 0_u8;
    pub const REGEN_MAX: u8 = 100_u8;
    
    /// Construct new Paddle from values
    pub fn new(regen: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_regen(regen)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// regen
    ///
    /// % of regen paddle travel
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn regen(&self) -> u8 {
        self.regen_raw()
    }
    
    /// Get raw value of regen
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn regen_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of regen
    #[inline(always)]
    pub fn set_regen(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 82 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Paddle {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Paddle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Paddle")
                .field("regen", &self.regen())
            .finish()
        } else {
            f.debug_tuple("Paddle").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Paddle {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let regen = u.int_in_range(0..=100)?;
        Paddle::new(regen).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Driver
///
/// - ID: 83 (0x53)
/// - Size: 4 bytes
/// - Transmitter: SMUF
#[derive(Clone, Copy)]
pub struct Driver {
    raw: [u8; 4],
}

impl Driver {
    pub const MESSAGE_ID: u32 = 83;
    
    pub const THROTTLE_MIN: u8 = 0_u8;
    pub const THROTTLE_MAX: u8 = 100_u8;
    pub const BRAKE_MIN: u8 = 0_u8;
    pub const BRAKE_MAX: u8 = 100_u8;
    pub const STEERING_MIN: i16 = -120_i16;
    pub const STEERING_MAX: i16 = 120_i16;
    
    /// Construct new Driver from values
    pub fn new(throttle: u8, brake: u8, steering: i16, no_implausibility: bool, bre_implausibility: bool, pad_implausibility: bool, pot_implausibility: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_throttle(throttle)?;
        res.set_brake(brake)?;
        res.set_steering(steering)?;
        res.set_no_implausibility(no_implausibility)?;
        res.set_bre_implausibility(bre_implausibility)?;
        res.set_pad_implausibility(pad_implausibility)?;
        res.set_pot_implausibility(pot_implausibility)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// throttle
    ///
    /// % of throttle
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn throttle(&self) -> u8 {
        self.throttle_raw()
    }
    
    /// Get raw value of throttle
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn throttle_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of throttle
    #[inline(always)]
    pub fn set_throttle(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 83 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// brake
    ///
    /// % of brake pedal
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn brake(&self) -> u8 {
        self.brake_raw()
    }
    
    /// Get raw value of brake
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of brake
    #[inline(always)]
    pub fn set_brake(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 83 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// steering
    ///
    /// Steering angle in milli radians
    ///
    /// - Min: -120
    /// - Max: 120
    /// - Unit: "deg"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn steering(&self) -> i16 {
        self.steering_raw()
    }
    
    /// Get raw value of steering
    ///
    /// - Start bit: 16
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn steering_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..28].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of steering
    #[inline(always)]
    pub fn set_steering(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -120_i16 || 120_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 83 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..28].store_le(value);
        Ok(())
    }
    
    /// no_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn no_implausibility(&self) -> bool {
        self.no_implausibility_raw()
    }
    
    /// Get raw value of no_implausibility
    ///
    /// - Start bit: 28
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn no_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of no_implausibility
    #[inline(always)]
    pub fn set_no_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
        Ok(())
    }
    
    /// bre_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bre_implausibility(&self) -> bool {
        self.bre_implausibility_raw()
    }
    
    /// Get raw value of bre_implausibility
    ///
    /// - Start bit: 29
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bre_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of bre_implausibility
    #[inline(always)]
    pub fn set_bre_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
        Ok(())
    }
    
    /// pad_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pad_implausibility(&self) -> bool {
        self.pad_implausibility_raw()
    }
    
    /// Get raw value of pad_implausibility
    ///
    /// - Start bit: 30
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pad_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[30..31].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of pad_implausibility
    #[inline(always)]
    pub fn set_pad_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[30..31].store_le(value);
        Ok(())
    }
    
    /// pot_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pot_implausibility(&self) -> bool {
        self.pot_implausibility_raw()
    }
    
    /// Get raw value of pot_implausibility
    ///
    /// - Start bit: 31
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pot_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[31..32].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of pot_implausibility
    #[inline(always)]
    pub fn set_pot_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[31..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Driver {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Driver {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Driver")
                .field("throttle", &self.throttle())
                .field("brake", &self.brake())
                .field("steering", &self.steering())
                .field("no_implausibility", &self.no_implausibility())
                .field("bre_implausibility", &self.bre_implausibility())
                .field("pad_implausibility", &self.pad_implausibility())
                .field("pot_implausibility", &self.pot_implausibility())
            .finish()
        } else {
            f.debug_tuple("Driver").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Driver {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let throttle = u.int_in_range(0..=100)?;
        let brake = u.int_in_range(0..=100)?;
        let steering = u.int_in_range(-120..=120)?;
        let no_implausibility = u.int_in_range(0..=1)? == 1;
        let bre_implausibility = u.int_in_range(0..=1)? == 1;
        let pad_implausibility = u.int_in_range(0..=1)? == 1;
        let pot_implausibility = u.int_in_range(0..=1)? == 1;
        Driver::new(throttle,brake,steering,no_implausibility,bre_implausibility,pad_implausibility,pot_implausibility).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLv1
///
/// - ID: 84 (0x54)
/// - Size: 8 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLv1 {
    raw: [u8; 8],
}

impl BmsLv1 {
    pub const MESSAGE_ID: u32 = 84;
    
    pub const MAX_VOLT_MIN: f32 = 0_f32;
    pub const MAX_VOLT_MAX: f32 = 0_f32;
    pub const MIN_VOLT_MIN: f32 = 0_f32;
    pub const MIN_VOLT_MAX: f32 = 0_f32;
    pub const AVG_VOLT_MIN: f32 = 0_f32;
    pub const AVG_VOLT_MAX: f32 = 0_f32;
    pub const TOT_VOLT_MIN: f32 = 0_f32;
    pub const TOT_VOLT_MAX: f32 = 100_f32;
    
    /// Construct new BmsLv1 from values
    pub fn new(max_volt: f32, min_volt: f32, avg_volt: f32, tot_volt: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_max_volt(max_volt)?;
        res.set_min_volt(min_volt)?;
        res.set_avg_volt(avg_volt)?;
        res.set_tot_volt(tot_volt)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// max_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn max_volt(&self) -> f32 {
        self.max_volt_raw()
    }
    
    /// Get raw value of max_volt
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_volt
    #[inline(always)]
    pub fn set_max_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 84 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn min_volt(&self) -> f32 {
        self.min_volt_raw()
    }
    
    /// Get raw value of min_volt
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of min_volt
    #[inline(always)]
    pub fn set_min_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 84 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn avg_volt(&self) -> f32 {
        self.avg_volt_raw()
    }
    
    /// Get raw value of avg_volt
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of avg_volt
    #[inline(always)]
    pub fn set_avg_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 84 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// tot_volt
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tot_volt(&self) -> f32 {
        self.tot_volt_raw()
    }
    
    /// Get raw value of tot_volt
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tot_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tot_volt
    #[inline(always)]
    pub fn set_tot_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 100_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 84 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLv1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLv1")
                .field("max_volt", &self.max_volt())
                .field("min_volt", &self.min_volt())
                .field("avg_volt", &self.avg_volt())
                .field("tot_volt", &self.tot_volt())
            .finish()
        } else {
            f.debug_tuple("BmsLv1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLv1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let max_volt = u.float_in_range(0_f32..=0_f32)?;
        let min_volt = u.float_in_range(0_f32..=0_f32)?;
        let avg_volt = u.float_in_range(0_f32..=0_f32)?;
        let tot_volt = u.float_in_range(0_f32..=100_f32)?;
        BmsLv1::new(max_volt,min_volt,avg_volt,tot_volt).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLv2
///
/// - ID: 85 (0x55)
/// - Size: 8 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLv2 {
    raw: [u8; 8],
}

impl BmsLv2 {
    pub const MESSAGE_ID: u32 = 85;
    
    pub const MAX_TEMP_MIN: f32 = 0_f32;
    pub const MAX_TEMP_MAX: f32 = 0_f32;
    pub const MIN_TEMP_MIN: f32 = 0_f32;
    pub const MIN_TEMP_MAX: f32 = 0_f32;
    pub const CURRENT_MIN: f32 = 0_f32;
    pub const CURRENT_MAX: f32 = 0_f32;
    
    /// Construct new BmsLv2 from values
    pub fn new(max_temp: f32, min_temp: f32, current: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_max_temp(max_temp)?;
        res.set_min_temp(min_temp)?;
        res.set_current(current)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// max_temp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn max_temp(&self) -> f32 {
        self.max_temp_raw()
    }
    
    /// Get raw value of max_temp
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_temp_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_temp
    #[inline(always)]
    pub fn set_max_temp(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 85 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_temp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn min_temp(&self) -> f32 {
        self.min_temp_raw()
    }
    
    /// Get raw value of min_temp
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_temp_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of min_temp
    #[inline(always)]
    pub fn set_min_temp(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 85 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// current
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mA"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn current(&self) -> f32 {
        self.current_raw()
    }
    
    /// Get raw value of current
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of current
    #[inline(always)]
    pub fn set_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 85 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLv2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLv2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLv2")
                .field("max_temp", &self.max_temp())
                .field("min_temp", &self.min_temp())
                .field("current", &self.current())
            .finish()
        } else {
            f.debug_tuple("BmsLv2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLv2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let max_temp = u.float_in_range(0_f32..=0_f32)?;
        let min_temp = u.float_in_range(0_f32..=0_f32)?;
        let current = u.float_in_range(0_f32..=0_f32)?;
        BmsLv2::new(max_temp,min_temp,current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsHv1
///
/// - ID: 87 (0x57)
/// - Size: 7 bytes
/// - Transmitter: BMSHV
#[derive(Clone, Copy)]
pub struct BmsHv1 {
    raw: [u8; 7],
}

impl BmsHv1 {
    pub const MESSAGE_ID: u32 = 87;
    
    pub const MAX_VOLT_MIN: f32 = 0_f32;
    pub const MAX_VOLT_MAX: f32 = 0_f32;
    pub const MIN_VOLT_MIN: f32 = 0_f32;
    pub const MIN_VOLT_MAX: f32 = 0_f32;
    pub const AVG_VOLT_MIN: f32 = 0_f32;
    pub const AVG_VOLT_MAX: f32 = 0_f32;
    pub const SOC_MIN: u8 = 0_u8;
    pub const SOC_MAX: u8 = 100_u8;
    
    /// Construct new BmsHv1 from values
    pub fn new(max_volt: f32, min_volt: f32, avg_volt: f32, soc: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_max_volt(max_volt)?;
        res.set_min_volt(min_volt)?;
        res.set_avg_volt(avg_volt)?;
        res.set_soc(soc)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// max_volt
    ///
    /// Maximum cell voltage in mv
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn max_volt(&self) -> f32 {
        self.max_volt_raw()
    }
    
    /// Get raw value of max_volt
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_volt
    #[inline(always)]
    pub fn set_max_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 87 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_volt
    ///
    /// Minimum cell voltage in mv
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn min_volt(&self) -> f32 {
        self.min_volt_raw()
    }
    
    /// Get raw value of min_volt
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of min_volt
    #[inline(always)]
    pub fn set_min_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 87 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_volt
    ///
    /// Average cell voltage in mv
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn avg_volt(&self) -> f32 {
        self.avg_volt_raw()
    }
    
    /// Get raw value of avg_volt
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of avg_volt
    #[inline(always)]
    pub fn set_avg_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 87 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// soc
    ///
    /// HV battery SOC in %
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn soc(&self) -> u8 {
        self.soc_raw()
    }
    
    /// Get raw value of soc
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn soc_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        signal
    }
    
    /// Set value of soc
    #[inline(always)]
    pub fn set_soc(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 87 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsHv1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsHv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsHv1")
                .field("max_volt", &self.max_volt())
                .field("min_volt", &self.min_volt())
                .field("avg_volt", &self.avg_volt())
                .field("soc", &self.soc())
            .finish()
        } else {
            f.debug_tuple("BmsHv1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsHv1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let max_volt = u.float_in_range(0_f32..=0_f32)?;
        let min_volt = u.float_in_range(0_f32..=0_f32)?;
        let avg_volt = u.float_in_range(0_f32..=0_f32)?;
        let soc = u.int_in_range(0..=100)?;
        BmsHv1::new(max_volt,min_volt,avg_volt,soc).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsHv2
///
/// - ID: 88 (0x58)
/// - Size: 7 bytes
/// - Transmitter: BMSHV
#[derive(Clone, Copy)]
pub struct BmsHv2 {
    raw: [u8; 7],
}

impl BmsHv2 {
    pub const MESSAGE_ID: u32 = 88;
    
    pub const MAX_TEMP_MIN: u16 = 0_u16;
    pub const MAX_TEMP_MAX: u16 = 0_u16;
    pub const MIN_TEMP_MIN: u16 = 0_u16;
    pub const MIN_TEMP_MAX: u16 = 0_u16;
    pub const AVG_TEMP_MIN: u16 = 0_u16;
    pub const AVG_TEMP_MAX: u16 = 0_u16;
    pub const FAN_SPEED_MIN: u8 = 0_u8;
    pub const FAN_SPEED_MAX: u8 = 100_u8;
    
    /// Construct new BmsHv2 from values
    pub fn new(max_temp: u16, min_temp: u16, avg_temp: u16, fan_speed: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_max_temp(max_temp)?;
        res.set_min_temp(min_temp)?;
        res.set_avg_temp(avg_temp)?;
        res.set_fan_speed(fan_speed)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// max_temp
    ///
    /// Maximum cell temperature in celsius
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn max_temp(&self) -> u16 {
        self.max_temp_raw()
    }
    
    /// Get raw value of max_temp
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of max_temp
    #[inline(always)]
    pub fn set_max_temp(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 88 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_temp
    ///
    /// Minimum cell temperature in celsius
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn min_temp(&self) -> u16 {
        self.min_temp_raw()
    }
    
    /// Get raw value of min_temp
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of min_temp
    #[inline(always)]
    pub fn set_min_temp(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 88 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_temp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn avg_temp(&self) -> u16 {
        self.avg_temp_raw()
    }
    
    /// Get raw value of avg_temp
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of avg_temp
    #[inline(always)]
    pub fn set_avg_temp(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 88 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// fan_speed
    ///
    /// % of HV battery fan speed
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn fan_speed(&self) -> u8 {
        self.fan_speed_raw()
    }
    
    /// Get raw value of fan_speed
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fan_speed_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        signal
    }
    
    /// Set value of fan_speed
    #[inline(always)]
    pub fn set_fan_speed(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 88 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsHv2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsHv2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsHv2")
                .field("max_temp", &self.max_temp())
                .field("min_temp", &self.min_temp())
                .field("avg_temp", &self.avg_temp())
                .field("fan_speed", &self.fan_speed())
            .finish()
        } else {
            f.debug_tuple("BmsHv2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsHv2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let max_temp = u.int_in_range(0..=0)?;
        let min_temp = u.int_in_range(0..=0)?;
        let avg_temp = u.int_in_range(0..=0)?;
        let fan_speed = u.int_in_range(0..=100)?;
        BmsHv2::new(max_temp,min_temp,avg_temp,fan_speed).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Imu1
///
/// - ID: 96 (0x60)
/// - Size: 8 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct Imu1 {
    raw: [u8; 8],
}

impl Imu1 {
    pub const MESSAGE_ID: u32 = 96;
    
    pub const ACC_X_MIN: i32 = 0_i32;
    pub const ACC_X_MAX: i32 = 0_i32;
    pub const ACC_Y_MIN: i32 = 0_i32;
    pub const ACC_Y_MAX: i32 = 0_i32;
    
    /// Construct new Imu1 from values
    pub fn new(acc_x: i32, acc_y: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_x(acc_x)?;
        res.set_acc_y(acc_y)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// acc_x
    ///
    /// Acceleration on x axis in m/s^2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn acc_x(&self) -> i32 {
        self.acc_x_raw()
    }
    
    /// Get raw value of acc_x
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_x_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of acc_x
    #[inline(always)]
    pub fn set_acc_x(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 96 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// acc_y
    ///
    /// Acceleration on y axis in m/s^2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn acc_y(&self) -> i32 {
        self.acc_y_raw()
    }
    
    /// Get raw value of acc_y
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_y_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of acc_y
    #[inline(always)]
    pub fn set_acc_y(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 96 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Imu1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Imu1")
                .field("acc_x", &self.acc_x())
                .field("acc_y", &self.acc_y())
            .finish()
        } else {
            f.debug_tuple("Imu1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Imu1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let acc_x = u.int_in_range(0..=0)?;
        let acc_y = u.int_in_range(0..=0)?;
        Imu1::new(acc_x,acc_y).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Imu2
///
/// - ID: 97 (0x61)
/// - Size: 8 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct Imu2 {
    raw: [u8; 8],
}

impl Imu2 {
    pub const MESSAGE_ID: u32 = 97;
    
    pub const ACC_Z_MIN: i32 = 0_i32;
    pub const ACC_Z_MAX: i32 = 0_i32;
    pub const OMEGA_X_MIN: i32 = 0_i32;
    pub const OMEGA_X_MAX: i32 = 0_i32;
    
    /// Construct new Imu2 from values
    pub fn new(acc_z: i32, omega_x: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_z(acc_z)?;
        res.set_omega_x(omega_x)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// acc_z
    ///
    /// Acceleration on z axis in m/s^2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn acc_z(&self) -> i32 {
        self.acc_z_raw()
    }
    
    /// Get raw value of acc_z
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_z_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of acc_z
    #[inline(always)]
    pub fn set_acc_z(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 97 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// omega_x
    ///
    /// Angular speed on x axis in rad/s
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn omega_x(&self) -> i32 {
        self.omega_x_raw()
    }
    
    /// Get raw value of omega_x
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn omega_x_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of omega_x
    #[inline(always)]
    pub fn set_omega_x(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 97 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Imu2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Imu2")
                .field("acc_z", &self.acc_z())
                .field("omega_x", &self.omega_x())
            .finish()
        } else {
            f.debug_tuple("Imu2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Imu2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let acc_z = u.int_in_range(0..=0)?;
        let omega_x = u.int_in_range(0..=0)?;
        Imu2::new(acc_z,omega_x).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Imu3
///
/// - ID: 98 (0x62)
/// - Size: 8 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct Imu3 {
    raw: [u8; 8],
}

impl Imu3 {
    pub const MESSAGE_ID: u32 = 98;
    
    pub const OMEGA_Y_MIN: i32 = 0_i32;
    pub const OMEGA_Y_MAX: i32 = 0_i32;
    pub const OMEGA_Z_MIN: i32 = 0_i32;
    pub const OMEGA_Z_MAX: i32 = 0_i32;
    
    /// Construct new Imu3 from values
    pub fn new(omega_y: i32, omega_z: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_omega_y(omega_y)?;
        res.set_omega_z(omega_z)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// omega_y
    ///
    /// Angular speed on x axis in rad/s
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn omega_y(&self) -> i32 {
        self.omega_y_raw()
    }
    
    /// Get raw value of omega_y
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn omega_y_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of omega_y
    #[inline(always)]
    pub fn set_omega_y(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 98 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// omega_z
    ///
    /// Angular speed on x axis in rad/s
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn omega_z(&self) -> i32 {
        self.omega_z_raw()
    }
    
    /// Get raw value of omega_z
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn omega_z_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of omega_z
    #[inline(always)]
    pub fn set_omega_z(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 98 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Imu3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Imu3")
                .field("omega_y", &self.omega_y())
                .field("omega_z", &self.omega_z())
            .finish()
        } else {
            f.debug_tuple("Imu3").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Imu3 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let omega_y = u.int_in_range(0..=0)?;
        let omega_z = u.int_in_range(0..=0)?;
        Imu3::new(omega_y,omega_z).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// IMUCalib
///
/// - ID: 99 (0x63)
/// - Size: 1 bytes
/// - Transmitter: IMU
///
/// RESERVER FOR IMU mask - DO NOT USE
#[derive(Clone, Copy)]
pub struct ImuCalib {
    raw: [u8; 1],
}

impl ImuCalib {
    pub const MESSAGE_ID: u32 = 99;
    
    
    /// Construct new IMUCalib from values
    pub fn new(start_imu_calibration: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_start_imu_calibration(start_imu_calibration)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// start_imu_calibration
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn start_imu_calibration(&self) -> bool {
        self.start_imu_calibration_raw()
    }
    
    /// Get raw value of start_imu_calibration
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn start_imu_calibration_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of start_imu_calibration
    #[inline(always)]
    pub fn set_start_imu_calibration(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ImuCalib {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for ImuCalib {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("ImuCalib")
                .field("start_imu_calibration", &self.start_imu_calibration())
            .finish()
        } else {
            f.debug_tuple("ImuCalib").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for ImuCalib {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let start_imu_calibration = u.int_in_range(0..=1)? == 1;
        ImuCalib::new(start_imu_calibration).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Map
///
/// - ID: 100 (0x64)
/// - Size: 2 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct Map {
    raw: [u8; 2],
}

impl Map {
    pub const MESSAGE_ID: u32 = 100;
    
    pub const POWER_MIN: u8 = 1_u8;
    pub const POWER_MAX: u8 = 12_u8;
    pub const REGEN_MIN: u8 = 1_u8;
    pub const REGEN_MAX: u8 = 12_u8;
    pub const TORQUE_REP_MIN: u8 = 0_u8;
    pub const TORQUE_REP_MAX: u8 = 12_u8;
    
    /// Construct new Map from values
    pub fn new(power: u8, regen: u8, torque_rep: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_power(power)?;
        res.set_regen(regen)?;
        res.set_torque_rep(torque_rep)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// power
    ///
    /// Map selected number
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn power(&self) -> u8 {
        self.power_raw()
    }
    
    /// Get raw value of power
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn power_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        signal
    }
    
    /// Set value of power
    #[inline(always)]
    pub fn set_power(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 100 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// regen
    ///
    /// Map selected for regen braking
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn regen(&self) -> u8 {
        self.regen_raw()
    }
    
    /// Get raw value of regen
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn regen_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of regen
    #[inline(always)]
    pub fn set_regen(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 100 });
        }
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    
    /// torque_rep
    ///
    /// Map selected for torque repeartition
    ///
    /// - Min: 0
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_rep(&self) -> u8 {
        self.torque_rep_raw()
    }
    
    /// Get raw value of torque_rep
    ///
    /// - Start bit: 8
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_rep_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
        
        signal
    }
    
    /// Set value of torque_rep
    #[inline(always)]
    pub fn set_torque_rep(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 100 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Map {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Map {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Map")
                .field("power", &self.power())
                .field("regen", &self.regen())
                .field("torque_rep", &self.torque_rep())
            .finish()
        } else {
            f.debug_tuple("Map").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Map {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let power = u.int_in_range(1..=12)?;
        let regen = u.int_in_range(1..=12)?;
        let torque_rep = u.int_in_range(0..=12)?;
        Map::new(power,regen,torque_rep).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CarStatus
///
/// - ID: 101 (0x65)
/// - Size: 5 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CarStatus {
    raw: [u8; 5],
}

impl CarStatus {
    pub const MESSAGE_ID: u32 = 101;
    
    pub const RUNNING_STATUS_MIN: u8 = 0_u8;
    pub const RUNNING_STATUS_MAX: u8 = 3_u8;
    pub const SPEED_MIN: u8 = 0_u8;
    pub const SPEED_MAX: u8 = 0_u8;
    pub const BRAKE_FRONT_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_FRONT_PRESS_MAX: f32 = 60_f32;
    pub const BRAKE_REAR_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_REAR_PRESS_MAX: f32 = 60_f32;
    
    /// Construct new CarStatus from values
    pub fn new(hv: bool, air1: bool, precharge: bool, as_node: bool, scs: bool, rtd_req: bool, running_status: u8, speed: u8, brake_front_press: f32, brake_rear_press: f32, bre_impl: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 5] };
        res.set_hv(hv)?;
        res.set_air1(air1)?;
        res.set_precharge(precharge)?;
        res.set_as_node(as_node)?;
        res.set_scs(scs)?;
        res.set_rtd_req(rtd_req)?;
        res.set_running_status(running_status)?;
        res.set_speed(speed)?;
        res.set_brake_front_press(brake_front_press)?;
        res.set_brake_rear_press(brake_rear_press)?;
        res.set_bre_impl(bre_impl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 5] {
        &self.raw
    }
    
    /// HV
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Off/On"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hv(&self) -> bool {
        self.hv_raw()
    }
    
    /// Get raw value of HV
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hv_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of HV
    #[inline(always)]
    pub fn set_hv(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// AIR1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Closed/Open"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn air1(&self) -> bool {
        self.air1_raw()
    }
    
    /// Get raw value of AIR1
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn air1_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of AIR1
    #[inline(always)]
    pub fn set_air1(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
    /// precharge
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Closed/Open"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn precharge(&self) -> bool {
        self.precharge_raw()
    }
    
    /// Get raw value of precharge
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn precharge_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of precharge
    #[inline(always)]
    pub fn set_precharge(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }
    
    /// AS_NODE
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Open/Closed"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn as_node(&self) -> bool {
        self.as_node_raw()
    }
    
    /// Get raw value of AS_NODE
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn as_node_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of AS_NODE
    #[inline(always)]
    pub fn set_as_node(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }
    
    /// SCS
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Open/Closed"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn scs(&self) -> bool {
        self.scs_raw()
    }
    
    /// Get raw value of SCS
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn scs_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of SCS
    #[inline(always)]
    pub fn set_scs(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// rtd_req
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Open/Closed"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rtd_req(&self) -> bool {
        self.rtd_req_raw()
    }
    
    /// Get raw value of rtd_req
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rtd_req_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rtd_req
    #[inline(always)]
    pub fn set_rtd_req(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
    /// RunningStatus
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: " Phase"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn running_status(&self) -> CarStatusRunningStatus {
        let signal = self.raw.view_bits::<Lsb0>()[6..8].load_le::<u8>();
        
        match signal {
            3 => CarStatusRunningStatus::Running,
            2 => CarStatusRunningStatus::TsReady,
            1 => CarStatusRunningStatus::PrechargeStarted,
            0 => CarStatusRunningStatus::SystemOff,
            _ => CarStatusRunningStatus::_Other(self.running_status_raw()),
        }
    }
    
    /// Get raw value of RunningStatus
    ///
    /// - Start bit: 6
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn running_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[6..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of RunningStatus
    #[inline(always)]
    pub fn set_running_status(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 101 });
        }
        self.raw.view_bits_mut::<Lsb0>()[6..8].store_le(value);
        Ok(())
    }
    
    /// speed
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " km/h"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed(&self) -> u8 {
        self.speed_raw()
    }
    
    /// Get raw value of speed
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn speed_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of speed
    #[inline(always)]
    pub fn set_speed(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 101 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// brake_front_press
    ///
    /// - Min: 0
    /// - Max: 60
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_front_press(&self) -> f32 {
        self.brake_front_press_raw()
    }
    
    /// Get raw value of brake_front_press
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 0.25
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_front_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_front_press
    #[inline(always)]
    pub fn set_brake_front_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 101 });
        }
        let factor = 0.25_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// brake_rear_press
    ///
    /// - Min: 0
    /// - Max: 60
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_rear_press(&self) -> f32 {
        self.brake_rear_press_raw()
    }
    
    /// Get raw value of brake_rear_press
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 0.25
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_rear_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_rear_press
    #[inline(always)]
    pub fn set_brake_rear_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 101 });
        }
        let factor = 0.25_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// bre_impl
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bre_impl(&self) -> CarStatusBreImpl {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        match signal {
            1 => CarStatusBreImpl::On,
            0 => CarStatusBreImpl::Off,
            _ => CarStatusBreImpl::_Other(self.bre_impl_raw()),
        }
    }
    
    /// Get raw value of bre_impl
    ///
    /// - Start bit: 32
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bre_impl_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of bre_impl
    #[inline(always)]
    pub fn set_bre_impl(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 5 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 5];
        raw.copy_from_slice(&payload[..5]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CarStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CarStatus")
                .field("hv", &self.hv())
                .field("air1", &self.air1())
                .field("precharge", &self.precharge())
                .field("as_node", &self.as_node())
                .field("scs", &self.scs())
                .field("rtd_req", &self.rtd_req())
                .field("running_status", &self.running_status())
                .field("speed", &self.speed())
                .field("brake_front_press", &self.brake_front_press())
                .field("brake_rear_press", &self.brake_rear_press())
                .field("bre_impl", &self.bre_impl())
            .finish()
        } else {
            f.debug_tuple("CarStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CarStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let hv = u.int_in_range(0..=1)? == 1;
        let air1 = u.int_in_range(0..=1)? == 1;
        let precharge = u.int_in_range(0..=1)? == 1;
        let as_node = u.int_in_range(0..=1)? == 1;
        let scs = u.int_in_range(0..=1)? == 1;
        let rtd_req = u.int_in_range(0..=1)? == 1;
        let running_status = u.int_in_range(0..=3)?;
        let speed = u.int_in_range(0..=0)?;
        let brake_front_press = u.float_in_range(0_f32..=60_f32)?;
        let brake_rear_press = u.float_in_range(0_f32..=60_f32)?;
        let bre_impl = u.int_in_range(0..=1)? == 1;
        CarStatus::new(hv,air1,precharge,as_node,scs,rtd_req,running_status,speed,brake_front_press,brake_rear_press,bre_impl).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for RunningStatus
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CarStatusRunningStatus {
    Running,
    TsReady,
    PrechargeStarted,
    SystemOff,
    _Other(u8),
}

impl From<CarStatusRunningStatus> for u8 {
    fn from(val: CarStatusRunningStatus) -> u8 {
        match val {
            CarStatusRunningStatus::Running => 3,
            CarStatusRunningStatus::TsReady => 2,
            CarStatusRunningStatus::PrechargeStarted => 1,
            CarStatusRunningStatus::SystemOff => 0,
            CarStatusRunningStatus::_Other(x) => x,
        }
    }
}

/// Defined values for bre_impl
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CarStatusBreImpl {
    On,
    Off,
    _Other(bool),
}

impl From<CarStatusBreImpl> for bool {
    fn from(val: CarStatusBreImpl) -> bool {
        match val {
            CarStatusBreImpl::On => true,
            CarStatusBreImpl::Off => false,
            CarStatusBreImpl::_Other(x) => x,
        }
    }
}


/// CarSettings
///
/// - ID: 102 (0x66)
/// - Size: 8 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CarSettings {
    raw: [u8; 8],
}

impl CarSettings {
    pub const MESSAGE_ID: u32 = 102;
    
    pub const MAX_REGEN_CURRENT_MIN: u8 = 0_u8;
    pub const MAX_REGEN_CURRENT_MAX: u8 = 150_u8;
    pub const PWR_LIMIT_MIN: u8 = 0_u8;
    pub const PWR_LIMIT_MAX: u8 = 80_u8;
    pub const SPEED_LIM_MIN: u8 = 0_u8;
    pub const SPEED_LIM_MAX: u8 = 0_u8;
    pub const MAX_POS_TRQ_MIN: u8 = 0_u8;
    pub const MAX_POS_TRQ_MAX: u8 = 0_u8;
    pub const MAX_NEG_TRQ_MIN: i8 = 0_i8;
    pub const MAX_NEG_TRQ_MAX: i8 = 0_i8;
    pub const FRONT_MOTOR_REPARTITION_MIN: u8 = 0_u8;
    pub const FRONT_MOTOR_REPARTITION_MAX: u8 = 0_u8;
    pub const REAR_MOTOR_REPARTITION_MIN: u8 = 0_u8;
    pub const REAR_MOTOR_REPARTITION_MAX: u8 = 0_u8;
    
    /// Construct new CarSettings from values
    pub fn new(max_regen_current: u8, pwr_limit: u8, speed_lim: u8, max_pos_trq: u8, max_neg_trq: i8, front_motor_repartition: u8, rear_motor_repartition: u8, torque_vectoring: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_max_regen_current(max_regen_current)?;
        res.set_pwr_limit(pwr_limit)?;
        res.set_speed_lim(speed_lim)?;
        res.set_max_pos_trq(max_pos_trq)?;
        res.set_max_neg_trq(max_neg_trq)?;
        res.set_front_motor_repartition(front_motor_repartition)?;
        res.set_rear_motor_repartition(rear_motor_repartition)?;
        res.set_torque_vectoring(torque_vectoring)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// max_regen_current
    ///
    /// Maximum Regen Current
    ///
    /// - Min: 0
    /// - Max: 150
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_regen_current(&self) -> u8 {
        self.max_regen_current_raw()
    }
    
    /// Get raw value of max_regen_current
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_regen_current_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of max_regen_current
    #[inline(always)]
    pub fn set_max_regen_current(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 150_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// pwr_limit
    ///
    /// Maximum power limit
    ///
    /// - Min: 0
    /// - Max: 80
    /// - Unit: "kW"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwr_limit(&self) -> u8 {
        self.pwr_limit_raw()
    }
    
    /// Get raw value of pwr_limit
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwr_limit_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of pwr_limit
    #[inline(always)]
    pub fn set_pwr_limit(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 80_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// speed_lim
    ///
    /// Maximum Speed Limit
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "krpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed_lim(&self) -> u8 {
        self.speed_lim_raw()
    }
    
    /// Get raw value of speed_lim
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn speed_lim_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        signal
    }
    
    /// Set value of speed_lim
    #[inline(always)]
    pub fn set_speed_lim(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// max_pos_trq
    ///
    /// Maximum Positive Torque
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_pos_trq(&self) -> u8 {
        self.max_pos_trq_raw()
    }
    
    /// Get raw value of max_pos_trq
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_pos_trq_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        signal
    }
    
    /// Set value of max_pos_trq
    #[inline(always)]
    pub fn set_max_pos_trq(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// max_neg_trq
    ///
    /// Maximum Negative Torque
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_neg_trq(&self) -> i8 {
        self.max_neg_trq_raw()
    }
    
    /// Get raw value of max_neg_trq
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn max_neg_trq_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<u8>();
        
        let signal  = i8::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of max_neg_trq
    #[inline(always)]
    pub fn set_max_neg_trq(&mut self, value: i8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// front_motor_repartition
    ///
    /// Torque Repartition Front
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn front_motor_repartition(&self) -> u8 {
        self.front_motor_repartition_raw()
    }
    
    /// Get raw value of front_motor_repartition
    ///
    /// - Start bit: 40
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn front_motor_repartition_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
        
        signal
    }
    
    /// Set value of front_motor_repartition
    #[inline(always)]
    pub fn set_front_motor_repartition(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
        Ok(())
    }
    
    /// rear_motor_repartition
    ///
    /// Torque Repartition Rear
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rear_motor_repartition(&self) -> u8 {
        self.rear_motor_repartition_raw()
    }
    
    /// Get raw value of rear_motor_repartition
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rear_motor_repartition_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        signal
    }
    
    /// Set value of rear_motor_repartition
    #[inline(always)]
    pub fn set_rear_motor_repartition(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 102 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
    /// torque_vectoring
    ///
    /// Torque Vectoring enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_vectoring(&self) -> bool {
        self.torque_vectoring_raw()
    }
    
    /// Get raw value of torque_vectoring
    ///
    /// - Start bit: 56
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_vectoring_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[56..57].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of torque_vectoring
    #[inline(always)]
    pub fn set_torque_vectoring(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[56..57].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarSettings {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CarSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CarSettings")
                .field("max_regen_current", &self.max_regen_current())
                .field("pwr_limit", &self.pwr_limit())
                .field("speed_lim", &self.speed_lim())
                .field("max_pos_trq", &self.max_pos_trq())
                .field("max_neg_trq", &self.max_neg_trq())
                .field("front_motor_repartition", &self.front_motor_repartition())
                .field("rear_motor_repartition", &self.rear_motor_repartition())
                .field("torque_vectoring", &self.torque_vectoring())
            .finish()
        } else {
            f.debug_tuple("CarSettings").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CarSettings {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let max_regen_current = u.int_in_range(0..=150)?;
        let pwr_limit = u.int_in_range(0..=80)?;
        let speed_lim = u.int_in_range(0..=0)?;
        let max_pos_trq = u.int_in_range(0..=0)?;
        let max_neg_trq = u.int_in_range(0..=0)?;
        let front_motor_repartition = u.int_in_range(0..=0)?;
        let rear_motor_repartition = u.int_in_range(0..=0)?;
        let torque_vectoring = u.int_in_range(0..=1)? == 1;
        CarSettings::new(max_regen_current,pwr_limit,speed_lim,max_pos_trq,max_neg_trq,front_motor_repartition,rear_motor_repartition,torque_vectoring).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CheckASBReq
///
/// - ID: 104 (0x68)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CheckAsbReq {
    raw: [u8; 1],
}

impl CheckAsbReq {
    pub const MESSAGE_ID: u32 = 104;
    
    
    /// Construct new CheckASBReq from values
    pub fn new(req: bool, req_ack: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_req(req)?;
        res.set_req_ack(req_ack)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// req
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn req(&self) -> bool {
        self.req_raw()
    }
    
    /// Get raw value of req
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn req_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of req
    #[inline(always)]
    pub fn set_req(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// reqAck
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn req_ack(&self) -> bool {
        self.req_ack_raw()
    }
    
    /// Get raw value of reqAck
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn req_ack_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of reqAck
    #[inline(always)]
    pub fn set_req_ack(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CheckAsbReq {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CheckAsbReq {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CheckAsbReq")
                .field("req", &self.req())
                .field("req_ack", &self.req_ack())
            .finish()
        } else {
            f.debug_tuple("CheckAsbReq").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CheckAsbReq {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let req = u.int_in_range(0..=1)? == 1;
        let req_ack = u.int_in_range(0..=1)? == 1;
        CheckAsbReq::new(req,req_ack).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// EbsBrakeReq
///
/// - ID: 105 (0x69)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct EbsBrakeReq {
    raw: [u8; 1],
}

impl EbsBrakeReq {
    pub const MESSAGE_ID: u32 = 105;
    
    
    /// Construct new EbsBrakeReq from values
    pub fn new(req: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_req(req)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// req
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn req(&self) -> bool {
        self.req_raw()
    }
    
    /// Get raw value of req
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn req_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of req
    #[inline(always)]
    pub fn set_req(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EbsBrakeReq {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for EbsBrakeReq {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("EbsBrakeReq")
                .field("req", &self.req())
            .finish()
        } else {
            f.debug_tuple("EbsBrakeReq").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for EbsBrakeReq {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let req = u.int_in_range(0..=1)? == 1;
        EbsBrakeReq::new(req).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// ResStatus
///
/// - ID: 106 (0x6a)
/// - Size: 1 bytes
#[derive(Clone, Copy)]
pub struct ResStatus {
    raw: [u8; 1],
}

impl ResStatus {
    pub const MESSAGE_ID: u32 = 106;
    
    
    /// Construct new ResStatus from values
    pub fn new(data: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_data(data)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// data
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn data(&self) -> bool {
        self.data_raw()
    }
    
    /// Get raw value of data
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn data_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of data
    #[inline(always)]
    pub fn set_data(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ResStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for ResStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("ResStatus")
                .field("data", &self.data())
            .finish()
        } else {
            f.debug_tuple("ResStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for ResStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let data = u.int_in_range(0..=1)? == 1;
        ResStatus::new(data).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// LapStart
///
/// - ID: 112 (0x70)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct LapStart {
    raw: [u8; 1],
}

impl LapStart {
    pub const MESSAGE_ID: u32 = 112;
    
    pub const START_MIN: u8 = 0_u8;
    pub const START_MAX: u8 = 1_u8;
    
    /// Construct new LapStart from values
    pub fn new(start: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_start(start)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// start
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "start"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn start(&self) -> u8 {
        self.start_raw()
    }
    
    /// Get raw value of start
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn start_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of start
    #[inline(always)]
    pub fn set_start(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 112 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for LapStart {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for LapStart {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("LapStart")
                .field("start", &self.start())
            .finish()
        } else {
            f.debug_tuple("LapStart").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for LapStart {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let start = u.int_in_range(0..=1)?;
        LapStart::new(start).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CarMissionStatus
///
/// - ID: 113 (0x71)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CarMissionStatus {
    raw: [u8; 1],
}

impl CarMissionStatus {
    pub const MESSAGE_ID: u32 = 113;
    
    pub const MISSION_MIN: u8 = 0_u8;
    pub const MISSION_MAX: u8 = 7_u8;
    pub const MISSION_STATUS_MIN: u8 = 0_u8;
    pub const MISSION_STATUS_MAX: u8 = 2_u8;
    pub const AS_STATUS_MIN: u8 = 0_u8;
    pub const AS_STATUS_MAX: u8 = 7_u8;
    
    /// Construct new CarMissionStatus from values
    pub fn new(mission: u8, mission_status: u8, as_status: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_mission(mission)?;
        res.set_mission_status(mission_status)?;
        res.set_as_status(as_status)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Mission
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn mission(&self) -> CarMissionStatusMission {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        match signal {
            7 => CarMissionStatusMission::DvInspection,
            6 => CarMissionStatusMission::DvEbsTest,
            5 => CarMissionStatusMission::DvTrackdrive,
            4 => CarMissionStatusMission::DvAutocross,
            3 => CarMissionStatusMission::DvSkidpad,
            2 => CarMissionStatusMission::DvAcceleration,
            1 => CarMissionStatusMission::Manualy,
            0 => CarMissionStatusMission::None,
            _ => CarMissionStatusMission::_Other(self.mission_raw()),
        }
    }
    
    /// Get raw value of Mission
    ///
    /// - Start bit: 0
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mission_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        signal
    }
    
    /// Set value of Mission
    #[inline(always)]
    pub fn set_mission(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 113 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..3].store_le(value);
        Ok(())
    }
    
    /// MissionStatus
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn mission_status(&self) -> CarMissionStatusMissionStatus {
        let signal = self.raw.view_bits::<Lsb0>()[3..5].load_le::<u8>();
        
        match signal {
            2 => CarMissionStatusMissionStatus::MissionFinished,
            1 => CarMissionStatusMissionStatus::MissionRunning,
            0 => CarMissionStatusMissionStatus::MissionNotRunning,
            _ => CarMissionStatusMissionStatus::_Other(self.mission_status_raw()),
        }
    }
    
    /// Get raw value of MissionStatus
    ///
    /// - Start bit: 3
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mission_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[3..5].load_le::<u8>();
        
        signal
    }
    
    /// Set value of MissionStatus
    #[inline(always)]
    pub fn set_mission_status(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 2_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 113 });
        }
        self.raw.view_bits_mut::<Lsb0>()[3..5].store_le(value);
        Ok(())
    }
    
    /// AsStatus
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn as_status(&self) -> CarMissionStatusAsStatus {
        let signal = self.raw.view_bits::<Lsb0>()[5..8].load_le::<u8>();
        
        match signal {
            5 => CarMissionStatusAsStatus::Finish,
            4 => CarMissionStatusAsStatus::EmergencyBrake,
            3 => CarMissionStatusAsStatus::Driving,
            2 => CarMissionStatusAsStatus::Ready,
            1 => CarMissionStatusAsStatus::Off,
            _ => CarMissionStatusAsStatus::_Other(self.as_status_raw()),
        }
    }
    
    /// Get raw value of AsStatus
    ///
    /// - Start bit: 5
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn as_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[5..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of AsStatus
    #[inline(always)]
    pub fn set_as_status(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 113 });
        }
        self.raw.view_bits_mut::<Lsb0>()[5..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarMissionStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CarMissionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CarMissionStatus")
                .field("mission", &self.mission())
                .field("mission_status", &self.mission_status())
                .field("as_status", &self.as_status())
            .finish()
        } else {
            f.debug_tuple("CarMissionStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CarMissionStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mission = u.int_in_range(0..=7)?;
        let mission_status = u.int_in_range(0..=2)?;
        let as_status = u.int_in_range(0..=7)?;
        CarMissionStatus::new(mission,mission_status,as_status).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for Mission
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CarMissionStatusMission {
    DvInspection,
    DvEbsTest,
    DvTrackdrive,
    DvAutocross,
    DvSkidpad,
    DvAcceleration,
    Manualy,
    None,
    _Other(u8),
}

impl From<CarMissionStatusMission> for u8 {
    fn from(val: CarMissionStatusMission) -> u8 {
        match val {
            CarMissionStatusMission::DvInspection => 7,
            CarMissionStatusMission::DvEbsTest => 6,
            CarMissionStatusMission::DvTrackdrive => 5,
            CarMissionStatusMission::DvAutocross => 4,
            CarMissionStatusMission::DvSkidpad => 3,
            CarMissionStatusMission::DvAcceleration => 2,
            CarMissionStatusMission::Manualy => 1,
            CarMissionStatusMission::None => 0,
            CarMissionStatusMission::_Other(x) => x,
        }
    }
}

/// Defined values for MissionStatus
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CarMissionStatusMissionStatus {
    MissionFinished,
    MissionRunning,
    MissionNotRunning,
    _Other(u8),
}

impl From<CarMissionStatusMissionStatus> for u8 {
    fn from(val: CarMissionStatusMissionStatus) -> u8 {
        match val {
            CarMissionStatusMissionStatus::MissionFinished => 2,
            CarMissionStatusMissionStatus::MissionRunning => 1,
            CarMissionStatusMissionStatus::MissionNotRunning => 0,
            CarMissionStatusMissionStatus::_Other(x) => x,
        }
    }
}

/// Defined values for AsStatus
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CarMissionStatusAsStatus {
    Finish,
    EmergencyBrake,
    Driving,
    Ready,
    Off,
    _Other(u8),
}

impl From<CarMissionStatusAsStatus> for u8 {
    fn from(val: CarMissionStatusAsStatus) -> u8 {
        match val {
            CarMissionStatusAsStatus::Finish => 5,
            CarMissionStatusAsStatus::EmergencyBrake => 4,
            CarMissionStatusAsStatus::Driving => 3,
            CarMissionStatusAsStatus::Ready => 2,
            CarMissionStatusAsStatus::Off => 1,
            CarMissionStatusAsStatus::_Other(x) => x,
        }
    }
}


/// Imu4
///
/// - ID: 128 (0x80)
/// - Size: 8 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct Imu4 {
    raw: [u8; 8],
}

impl Imu4 {
    pub const MESSAGE_ID: u32 = 128;
    
    pub const MAG_X_MIN: i32 = 0_i32;
    pub const MAG_X_MAX: i32 = 0_i32;
    pub const MAG_Y_MIN: i32 = 0_i32;
    pub const MAG_Y_MAX: i32 = 0_i32;
    
    /// Construct new Imu4 from values
    pub fn new(mag_x: i32, mag_y: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mag_x(mag_x)?;
        res.set_mag_y(mag_y)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// mag_x
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn mag_x(&self) -> i32 {
        self.mag_x_raw()
    }
    
    /// Get raw value of mag_x
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn mag_x_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of mag_x
    #[inline(always)]
    pub fn set_mag_x(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 128 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// mag_y
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn mag_y(&self) -> i32 {
        self.mag_y_raw()
    }
    
    /// Get raw value of mag_y
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn mag_y_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of mag_y
    #[inline(always)]
    pub fn set_mag_y(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 128 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu4 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Imu4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Imu4")
                .field("mag_x", &self.mag_x())
                .field("mag_y", &self.mag_y())
            .finish()
        } else {
            f.debug_tuple("Imu4").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Imu4 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mag_x = u.int_in_range(0..=0)?;
        let mag_y = u.int_in_range(0..=0)?;
        Imu4::new(mag_x,mag_y).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Imu5
///
/// - ID: 129 (0x81)
/// - Size: 8 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct Imu5 {
    raw: [u8; 8],
}

impl Imu5 {
    pub const MESSAGE_ID: u32 = 129;
    
    pub const MAG_Z_MIN: i32 = 0_i32;
    pub const MAG_Z_MAX: i32 = 0_i32;
    pub const NULL_MIN: i32 = 0_i32;
    pub const NULL_MAX: i32 = 0_i32;
    
    /// Construct new Imu5 from values
    pub fn new(mag_z: i32, null: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_mag_z(mag_z)?;
        res.set_null(null)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// mag_z
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn mag_z(&self) -> i32 {
        self.mag_z_raw()
    }
    
    /// Get raw value of mag_z
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn mag_z_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of mag_z
    #[inline(always)]
    pub fn set_mag_z(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 129 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// null
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn null(&self) -> i32 {
        self.null_raw()
    }
    
    /// Get raw value of null
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn null_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of null
    #[inline(always)]
    pub fn set_null(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 129 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu5 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Imu5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Imu5")
                .field("mag_z", &self.mag_z())
                .field("null", &self.null())
            .finish()
        } else {
            f.debug_tuple("Imu5").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Imu5 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mag_z = u.int_in_range(0..=0)?;
        let null = u.int_in_range(0..=0)?;
        Imu5::new(mag_z,null).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Temp1
///
/// - ID: 256 (0x100)
/// - Size: 8 bytes
/// - Transmitter: SMUR
#[derive(Clone, Copy)]
pub struct Temp1 {
    raw: [u8; 8],
}

impl Temp1 {
    pub const MESSAGE_ID: u32 = 256;
    
    pub const TEMP_INVERTER_POST_R_PIN_MIN: u16 = 0_u16;
    pub const TEMP_INVERTER_POST_R_PIN_MAX: u16 = 0_u16;
    pub const TEMP_RAD_POST_R_PIN_MIN: u16 = 0_u16;
    pub const TEMP_RAD_POST_R_PIN_MAX: u16 = 0_u16;
    pub const TEMP_MOTOR_POST_RR_PIN_MIN: u16 = 0_u16;
    pub const TEMP_MOTOR_POST_RR_PIN_MAX: u16 = 0_u16;
    pub const TEMP_RAD_POST_L_PIN_MIN: u16 = 0_u16;
    pub const TEMP_RAD_POST_L_PIN_MAX: u16 = 0_u16;
    
    /// Construct new Temp1 from values
    pub fn new(temp_inverter_post_r_pin: u16, temp_rad_post_r_pin: u16, temp_motor_post_rr_pin: u16, temp_rad_post_l_pin: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_temp_inverter_post_r_pin(temp_inverter_post_r_pin)?;
        res.set_temp_rad_post_r_pin(temp_rad_post_r_pin)?;
        res.set_temp_motor_post_rr_pin(temp_motor_post_rr_pin)?;
        res.set_temp_rad_post_l_pin(temp_rad_post_l_pin)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// TEMP_INVERTER_POST_R_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_inverter_post_r_pin(&self) -> u16 {
        self.temp_inverter_post_r_pin_raw()
    }
    
    /// Get raw value of TEMP_INVERTER_POST_R_PIN
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_inverter_post_r_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_INVERTER_POST_R_PIN
    #[inline(always)]
    pub fn set_temp_inverter_post_r_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// TEMP_RAD_POST_R_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_rad_post_r_pin(&self) -> u16 {
        self.temp_rad_post_r_pin_raw()
    }
    
    /// Get raw value of TEMP_RAD_POST_R_PIN
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_rad_post_r_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_RAD_POST_R_PIN
    #[inline(always)]
    pub fn set_temp_rad_post_r_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// TEMP_MOTOR_POST_RR_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_motor_post_rr_pin(&self) -> u16 {
        self.temp_motor_post_rr_pin_raw()
    }
    
    /// Get raw value of TEMP_MOTOR_POST_RR_PIN
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_motor_post_rr_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_MOTOR_POST_RR_PIN
    #[inline(always)]
    pub fn set_temp_motor_post_rr_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// TEMP_RAD_POST_L_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_rad_post_l_pin(&self) -> u16 {
        self.temp_rad_post_l_pin_raw()
    }
    
    /// Get raw value of TEMP_RAD_POST_L_PIN
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_rad_post_l_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_RAD_POST_L_PIN
    #[inline(always)]
    pub fn set_temp_rad_post_l_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 256 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Temp1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Temp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Temp1")
                .field("temp_inverter_post_r_pin", &self.temp_inverter_post_r_pin())
                .field("temp_rad_post_r_pin", &self.temp_rad_post_r_pin())
                .field("temp_motor_post_rr_pin", &self.temp_motor_post_rr_pin())
                .field("temp_rad_post_l_pin", &self.temp_rad_post_l_pin())
            .finish()
        } else {
            f.debug_tuple("Temp1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Temp1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let temp_inverter_post_r_pin = u.int_in_range(0..=0)?;
        let temp_rad_post_r_pin = u.int_in_range(0..=0)?;
        let temp_motor_post_rr_pin = u.int_in_range(0..=0)?;
        let temp_rad_post_l_pin = u.int_in_range(0..=0)?;
        Temp1::new(temp_inverter_post_r_pin,temp_rad_post_r_pin,temp_motor_post_rr_pin,temp_rad_post_l_pin).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Temp2
///
/// - ID: 257 (0x101)
/// - Size: 7 bytes
/// - Transmitter: SMUR
#[derive(Clone, Copy)]
pub struct Temp2 {
    raw: [u8; 7],
}

impl Temp2 {
    pub const MESSAGE_ID: u32 = 257;
    
    pub const TEMP_MOTOR_POST_RL_PIN_MIN: u16 = 0_u16;
    pub const TEMP_MOTOR_POST_RL_PIN_MAX: u16 = 0_u16;
    pub const TEMP_INVERTER_POST_L_PIN_MIN: u16 = 0_u16;
    pub const TEMP_INVERTER_POST_L_PIN_MAX: u16 = 0_u16;
    pub const TEMP_EMBEDDED_POST_PIN_MIN: u16 = 0_u16;
    pub const TEMP_EMBEDDED_POST_PIN_MAX: u16 = 0_u16;
    
    /// Construct new Temp2 from values
    pub fn new(temp_motor_post_rl_pin: u16, temp_inverter_post_l_pin: u16, temp_embedded_post_pin: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_temp_motor_post_rl_pin(temp_motor_post_rl_pin)?;
        res.set_temp_inverter_post_l_pin(temp_inverter_post_l_pin)?;
        res.set_temp_embedded_post_pin(temp_embedded_post_pin)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// TEMP_MOTOR_POST_RL_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_motor_post_rl_pin(&self) -> u16 {
        self.temp_motor_post_rl_pin_raw()
    }
    
    /// Get raw value of TEMP_MOTOR_POST_RL_PIN
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_motor_post_rl_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_MOTOR_POST_RL_PIN
    #[inline(always)]
    pub fn set_temp_motor_post_rl_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 257 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// TEMP_INVERTER_POST_L_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_inverter_post_l_pin(&self) -> u16 {
        self.temp_inverter_post_l_pin_raw()
    }
    
    /// Get raw value of TEMP_INVERTER_POST_L_PIN
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_inverter_post_l_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_INVERTER_POST_L_PIN
    #[inline(always)]
    pub fn set_temp_inverter_post_l_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 257 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// TEMP_EMBEDDED_POST_PIN
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_embedded_post_pin(&self) -> u16 {
        self.temp_embedded_post_pin_raw()
    }
    
    /// Get raw value of TEMP_EMBEDDED_POST_PIN
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_embedded_post_pin_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of TEMP_EMBEDDED_POST_PIN
    #[inline(always)]
    pub fn set_temp_embedded_post_pin(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 257 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Temp2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Temp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Temp2")
                .field("temp_motor_post_rl_pin", &self.temp_motor_post_rl_pin())
                .field("temp_inverter_post_l_pin", &self.temp_inverter_post_l_pin())
                .field("temp_embedded_post_pin", &self.temp_embedded_post_pin())
            .finish()
        } else {
            f.debug_tuple("Temp2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Temp2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let temp_motor_post_rl_pin = u.int_in_range(0..=0)?;
        let temp_inverter_post_l_pin = u.int_in_range(0..=0)?;
        let temp_embedded_post_pin = u.int_in_range(0..=0)?;
        Temp2::new(temp_motor_post_rl_pin,temp_inverter_post_l_pin,temp_embedded_post_pin).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// SuspRear
///
/// - ID: 258 (0x102)
/// - Size: 3 bytes
/// - Transmitter: SMUR
#[derive(Clone, Copy)]
pub struct SuspRear {
    raw: [u8; 3],
}

impl SuspRear {
    pub const MESSAGE_ID: u32 = 258;
    
    pub const SUSP_RR_MIN: f32 = 0_f32;
    pub const SUSP_RR_MAX: f32 = 0_f32;
    pub const SUSP_RL_MIN: f32 = 0_f32;
    pub const SUSP_RL_MAX: f32 = 0_f32;
    
    /// Construct new SuspRear from values
    pub fn new(susp_rr: f32, susp_rl: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_susp_rr(susp_rr)?;
        res.set_susp_rl(susp_rl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// susp_rr
    ///
    /// RR suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_rr(&self) -> f32 {
        self.susp_rr_raw()
    }
    
    /// Get raw value of susp_rr
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_rr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_rr
    #[inline(always)]
    pub fn set_susp_rr(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 258 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// susp_rl
    ///
    /// RL suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_rl(&self) -> f32 {
        self.susp_rl_raw()
    }
    
    /// Get raw value of susp_rl
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_rl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_rl
    #[inline(always)]
    pub fn set_susp_rl(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 258 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SuspRear {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for SuspRear {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("SuspRear")
                .field("susp_rr", &self.susp_rr())
                .field("susp_rl", &self.susp_rl())
            .finish()
        } else {
            f.debug_tuple("SuspRear").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for SuspRear {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let susp_rr = u.float_in_range(0_f32..=0_f32)?;
        let susp_rl = u.float_in_range(0_f32..=0_f32)?;
        SuspRear::new(susp_rr,susp_rl).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// RESERVED2
///
/// - ID: 259 (0x103)
/// - Size: 0 bytes
///
/// RESERVER FOR SMU mask - DO NOT USE
#[derive(Clone, Copy)]
pub struct Reserved2 {
    raw: [u8; 0],
}

impl Reserved2 {
    pub const MESSAGE_ID: u32 = 259;
    
    
    /// Construct new RESERVED2 from values
    pub fn new() -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 0] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 0] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Reserved2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 0 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 0];
        raw.copy_from_slice(&payload[..0]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Reserved2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Reserved2")
            .finish()
        } else {
            f.debug_tuple("Reserved2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Reserved2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        Reserved2::new().map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// SuspFront
///
/// - ID: 260 (0x104)
/// - Size: 3 bytes
/// - Transmitter: SMUF
#[derive(Clone, Copy)]
pub struct SuspFront {
    raw: [u8; 3],
}

impl SuspFront {
    pub const MESSAGE_ID: u32 = 260;
    
    pub const SUSP_FR_MIN: f32 = 0_f32;
    pub const SUSP_FR_MAX: f32 = 0_f32;
    pub const SUSP_FL_MIN: f32 = 0_f32;
    pub const SUSP_FL_MAX: f32 = 0_f32;
    
    /// Construct new SuspFront from values
    pub fn new(susp_fr: f32, susp_fl: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_susp_fr(susp_fr)?;
        res.set_susp_fl(susp_fl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// susp_fr
    ///
    /// FR suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_fr(&self) -> f32 {
        self.susp_fr_raw()
    }
    
    /// Get raw value of susp_fr
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_fr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_fr
    #[inline(always)]
    pub fn set_susp_fr(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 260 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// susp_fl
    ///
    /// FL suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_fl(&self) -> f32 {
        self.susp_fl_raw()
    }
    
    /// Get raw value of susp_fl
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_fl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_fl
    #[inline(always)]
    pub fn set_susp_fl(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 260 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SuspFront {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for SuspFront {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("SuspFront")
                .field("susp_fr", &self.susp_fr())
                .field("susp_fl", &self.susp_fl())
            .finish()
        } else {
            f.debug_tuple("SuspFront").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for SuspFront {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let susp_fr = u.float_in_range(0_f32..=0_f32)?;
        let susp_fl = u.float_in_range(0_f32..=0_f32)?;
        SuspFront::new(susp_fr,susp_fl).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// TempFrontR
///
/// - ID: 261 (0x105)
/// - Size: 3 bytes
/// - Transmitter: SMUF
#[derive(Clone, Copy)]
pub struct TempFrontR {
    raw: [u8; 3],
}

impl TempFrontR {
    pub const MESSAGE_ID: u32 = 261;
    
    pub const TEMP_MOT_POT_FR_MIN: u16 = 0_u16;
    pub const TEMP_MOT_POT_FR_MAX: u16 = 0_u16;
    pub const TEMP_MOT_PRE_FR_MIN: u16 = 0_u16;
    pub const TEMP_MOT_PRE_FR_MAX: u16 = 0_u16;
    
    /// Construct new TempFrontR from values
    pub fn new(temp_mot_pot_fr: u16, temp_mot_pre_fr: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_temp_mot_pot_fr(temp_mot_pot_fr)?;
        res.set_temp_mot_pre_fr(temp_mot_pre_fr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// temp_mot_pot_FR
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn temp_mot_pot_fr(&self) -> u16 {
        self.temp_mot_pot_fr_raw()
    }
    
    /// Get raw value of temp_mot_pot_FR
    ///
    /// - Start bit: 0
    /// - Signal size: 10 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_mot_pot_fr_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..10].load_le::<u16>();
        
        signal
    }
    
    /// Set value of temp_mot_pot_FR
    #[inline(always)]
    pub fn set_temp_mot_pot_fr(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 261 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..10].store_le(value);
        Ok(())
    }
    
    /// temp_mot_pre_FR
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn temp_mot_pre_fr(&self) -> u16 {
        self.temp_mot_pre_fr_raw()
    }
    
    /// Get raw value of temp_mot_pre_FR
    ///
    /// - Start bit: 10
    /// - Signal size: 10 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_mot_pre_fr_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[10..20].load_le::<u16>();
        
        signal
    }
    
    /// Set value of temp_mot_pre_FR
    #[inline(always)]
    pub fn set_temp_mot_pre_fr(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 261 });
        }
        self.raw.view_bits_mut::<Lsb0>()[10..20].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TempFrontR {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for TempFrontR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("TempFrontR")
                .field("temp_mot_pot_fr", &self.temp_mot_pot_fr())
                .field("temp_mot_pre_fr", &self.temp_mot_pre_fr())
            .finish()
        } else {
            f.debug_tuple("TempFrontR").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for TempFrontR {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let temp_mot_pot_fr = u.int_in_range(0..=0)?;
        let temp_mot_pre_fr = u.int_in_range(0..=0)?;
        TempFrontR::new(temp_mot_pot_fr,temp_mot_pre_fr).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// HydraulicPressure
///
/// - ID: 264 (0x108)
/// - Size: 8 bytes
/// - Transmitter: VCU
///
/// Hydraulic Brakes Pressures
#[derive(Clone, Copy)]
pub struct HydraulicPressure {
    raw: [u8; 8],
}

impl HydraulicPressure {
    pub const MESSAGE_ID: u32 = 264;
    
    pub const PRESS_FRONT_MIN: u32 = 0_u32;
    pub const PRESS_FRONT_MAX: u32 = 0_u32;
    pub const PRESS_REAR_MIN: u32 = 0_u32;
    pub const PRESS_REAR_MAX: u32 = 0_u32;
    
    /// Construct new HydraulicPressure from values
    pub fn new(press_front: u32, press_rear: u32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_press_front(press_front)?;
        res.set_press_rear(press_rear)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// PressFront
    ///
    /// 0 means fault
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: EBS
    #[inline(always)]
    pub fn press_front(&self) -> u32 {
        self.press_front_raw()
    }
    
    /// Get raw value of PressFront
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_front_raw(&self) -> u32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        signal
    }
    
    /// Set value of PressFront
    #[inline(always)]
    pub fn set_press_front(&mut self, value: u32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u32 || 0_u32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 264 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// PressRear
    ///
    /// 0 means fault
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: EBS
    #[inline(always)]
    pub fn press_rear(&self) -> u32 {
        self.press_rear_raw()
    }
    
    /// Get raw value of PressRear
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_rear_raw(&self) -> u32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        signal
    }
    
    /// Set value of PressRear
    #[inline(always)]
    pub fn set_press_rear(&mut self, value: u32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u32 || 0_u32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 264 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for HydraulicPressure {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for HydraulicPressure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("HydraulicPressure")
                .field("press_front", &self.press_front())
                .field("press_rear", &self.press_rear())
            .finish()
        } else {
            f.debug_tuple("HydraulicPressure").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for HydraulicPressure {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let press_front = u.int_in_range(0..=0)?;
        let press_rear = u.int_in_range(0..=0)?;
        HydraulicPressure::new(press_front,press_rear).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// TempImu
///
/// - ID: 277 (0x115)
/// - Size: 4 bytes
/// - Transmitter: IMU
#[derive(Clone, Copy)]
pub struct TempImu {
    raw: [u8; 4],
}

impl TempImu {
    pub const MESSAGE_ID: u32 = 277;
    
    pub const TEMP_PUMP_RIGHT_MIN: u16 = 0_u16;
    pub const TEMP_PUMP_RIGHT_MAX: u16 = 0_u16;
    pub const TEMP_PUMP_LEFT_MIN: u16 = 0_u16;
    pub const TEMP_PUMP_LEFT_MAX: u16 = 0_u16;
    
    /// Construct new TempImu from values
    pub fn new(temp_pump_right: u16, temp_pump_left: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_temp_pump_right(temp_pump_right)?;
        res.set_temp_pump_left(temp_pump_left)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// temp_pump_right
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_pump_right(&self) -> u16 {
        self.temp_pump_right_raw()
    }
    
    /// Get raw value of temp_pump_right
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_pump_right_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of temp_pump_right
    #[inline(always)]
    pub fn set_temp_pump_right(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 277 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// temp_pump_left
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_pump_left(&self) -> u16 {
        self.temp_pump_left_raw()
    }
    
    /// Get raw value of temp_pump_left
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_pump_left_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of temp_pump_left
    #[inline(always)]
    pub fn set_temp_pump_left(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 277 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TempImu {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for TempImu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("TempImu")
                .field("temp_pump_right", &self.temp_pump_right())
                .field("temp_pump_left", &self.temp_pump_left())
            .finish()
        } else {
            f.debug_tuple("TempImu").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for TempImu {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let temp_pump_right = u.int_in_range(0..=0)?;
        let temp_pump_left = u.int_in_range(0..=0)?;
        TempImu::new(temp_pump_right,temp_pump_left).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// InvVolt
///
/// - ID: 288 (0x120)
/// - Size: 2 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct InvVolt {
    raw: [u8; 2],
}

impl InvVolt {
    pub const MESSAGE_ID: u32 = 288;
    
    pub const CAR_VOLTAGE_MIN: u16 = 0_u16;
    pub const CAR_VOLTAGE_MAX: u16 = 600_u16;
    
    /// Construct new InvVolt from values
    pub fn new(car_voltage: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_car_voltage(car_voltage)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// car_voltage
    ///
    /// Voltage seen from car side (inverter) in volts
    ///
    /// - Min: 0
    /// - Max: 600
    /// - Unit: "V"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn car_voltage(&self) -> u16 {
        self.car_voltage_raw()
    }
    
    /// Get raw value of car_voltage
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn car_voltage_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of car_voltage
    #[inline(always)]
    pub fn set_car_voltage(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 600_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 288 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for InvVolt {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for InvVolt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("InvVolt")
                .field("car_voltage", &self.car_voltage())
            .finish()
        } else {
            f.debug_tuple("InvVolt").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for InvVolt {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let car_voltage = u.int_in_range(0..=600)?;
        InvVolt::new(car_voltage).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Pcu
///
/// - ID: 304 (0x130)
/// - Size: 7 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct Pcu {
    raw: [u8; 7],
}

impl Pcu {
    pub const MESSAGE_ID: u32 = 304;
    
    pub const MODE_MIN: u8 = 0_u8;
    pub const MODE_MAX: u8 = 2_u8;
    
    /// Construct new Pcu from values
    pub fn new(mode: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_mode(mode)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// Get raw value of mode
    ///
    /// - Start bit: 0
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..2].load_le::<u8>();
        
        signal
    }
    
    pub fn mode(&mut self) -> Result<PcuMode, CanError> {
        match self.mode_raw() {
            1 => Ok(PcuMode::M1(PcuModeM1{ raw: self.raw })),
            2 => Ok(PcuMode::M2(PcuModeM2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 304, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of mode
    #[inline(always)]
    fn set_mode(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 2_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 304 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..2].store_le(value);
        Ok(())
    }
    
    /// Set value of mode
    #[inline(always)]
    pub fn set_m1(&mut self, value: PcuModeM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(1)?;
        Ok(())
    }
    
    /// Set value of mode
    #[inline(always)]
    pub fn set_m2(&mut self, value: PcuModeM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Pcu {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Pcu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Pcu")
            .finish()
        } else {
            f.debug_tuple("Pcu").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Pcu {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let mode = u.int_in_range(0..=2)?;
        Pcu::new(mode).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal Pcu
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PcuMode {
    M1(PcuModeM1),
    M2(PcuModeM2),
}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct PcuModeM1 { raw: [u8; 7] }

impl PcuModeM1 {
pub fn new() -> Self { Self { raw: [0u8; 7] } }
/// rf
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn rf(&self) -> bool {
    self.rf_raw()
}

/// Get raw value of rf
///
/// - Start bit: 2
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn rf_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
    
    signal == 1
}

/// Set value of rf
#[inline(always)]
pub fn set_rf(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
    Ok(())
}

}

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct PcuModeM2 { raw: [u8; 7] }

impl PcuModeM2 {
pub fn new() -> Self { Self { raw: [0u8; 7] } }
/// enable_dv
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn enable_dv(&self) -> bool {
    self.enable_dv_raw()
}

/// Get raw value of enable_dv
///
/// - Start bit: 2
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn enable_dv_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
    
    signal == 1
}

/// Set value of enable_dv
#[inline(always)]
pub fn set_enable_dv(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
    Ok(())
}

/// enable_embedded
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn enable_embedded(&self) -> bool {
    self.enable_embedded_raw()
}

/// Get raw value of enable_embedded
///
/// - Start bit: 3
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn enable_embedded_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
    
    signal == 1
}

/// Set value of enable_embedded
#[inline(always)]
pub fn set_enable_embedded(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
    Ok(())
}

}


/// Calib
///
/// - ID: 305 (0x131)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct Calib {
    raw: [u8; 1],
}

impl Calib {
    pub const MESSAGE_ID: u32 = 305;
    
    pub const POSITION_MIN: u8 = 0_u8;
    pub const POSITION_MAX: u8 = 1_u8;
    
    /// Construct new Calib from values
    pub fn new(position: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_position(position)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Get raw value of position
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn position_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    pub fn position(&mut self) -> Result<CalibPosition, CanError> {
        match self.position_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 305, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of position
    #[inline(always)]
    fn set_position(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 305 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Calib {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Calib {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Calib")
            .finish()
        } else {
            f.debug_tuple("Calib").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Calib {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let position = u.int_in_range(0..=1)?;
        Calib::new(position).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal Calib
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CalibPosition {
}


/// CalibAck
///
/// - ID: 306 (0x132)
/// - Size: 1 bytes
/// - Transmitter: SMUF
#[derive(Clone, Copy)]
pub struct CalibAck {
    raw: [u8; 1],
}

impl CalibAck {
    pub const MESSAGE_ID: u32 = 306;
    
    pub const POSITION_MIN: u8 = 0_u8;
    pub const POSITION_MAX: u8 = 1_u8;
    
    /// Construct new CalibAck from values
    pub fn new(position: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_position(position)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Get raw value of position
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn position_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    pub fn position(&mut self) -> Result<CalibAckPosition, CanError> {
        match self.position_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 306, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of position
    #[inline(always)]
    fn set_position(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 306 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CalibAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CalibAck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CalibAck")
            .finish()
        } else {
            f.debug_tuple("CalibAck").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CalibAck {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let position = u.int_in_range(0..=1)?;
        CalibAck::new(position).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal CalibAck
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CalibAckPosition {
}


/// PcuSwControl
///
/// - ID: 307 (0x133)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct PcuSwControl {
    raw: [u8; 1],
}

impl PcuSwControl {
    pub const MESSAGE_ID: u32 = 307;
    
    
    /// Construct new PcuSwControl from values
    pub fn new(pump: bool, fan: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_pump(pump)?;
        res.set_fan(fan)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// pump
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn pump(&self) -> PcuSwControlPump {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        match signal {
            1 => PcuSwControlPump::On,
            0 => PcuSwControlPump::Off,
            _ => PcuSwControlPump::_Other(self.pump_raw()),
        }
    }
    
    /// Get raw value of pump
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pump_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of pump
    #[inline(always)]
    pub fn set_pump(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// fan
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn fan(&self) -> PcuSwControlFan {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        match signal {
            1 => PcuSwControlFan::On,
            0 => PcuSwControlFan::Off,
            _ => PcuSwControlFan::_Other(self.fan_raw()),
        }
    }
    
    /// Get raw value of fan
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fan_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fan
    #[inline(always)]
    pub fn set_fan(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuSwControl {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for PcuSwControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PcuSwControl")
                .field("pump", &self.pump())
                .field("fan", &self.fan())
            .finish()
        } else {
            f.debug_tuple("PcuSwControl").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PcuSwControl {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let pump = u.int_in_range(0..=1)? == 1;
        let fan = u.int_in_range(0..=1)? == 1;
        PcuSwControl::new(pump,fan).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for pump
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PcuSwControlPump {
    On,
    Off,
    _Other(bool),
}

impl From<PcuSwControlPump> for bool {
    fn from(val: PcuSwControlPump) -> bool {
        match val {
            PcuSwControlPump::On => true,
            PcuSwControlPump::Off => false,
            PcuSwControlPump::_Other(x) => x,
        }
    }
}

/// Defined values for fan
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum PcuSwControlFan {
    On,
    Off,
    _Other(bool),
}

impl From<PcuSwControlFan> for bool {
    fn from(val: PcuSwControlFan) -> bool {
        match val {
            PcuSwControlFan::On => true,
            PcuSwControlFan::Off => false,
            PcuSwControlFan::_Other(x) => x,
        }
    }
}


/// PcuRfAck
///
/// - ID: 308 (0x134)
/// - Size: 1 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuRfAck {
    raw: [u8; 1],
}

impl PcuRfAck {
    pub const MESSAGE_ID: u32 = 308;
    
    
    /// Construct new PcuRfAck from values
    pub fn new(rf_signal_ack: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_rf_signal_ack(rf_signal_ack)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// rf_signalAck
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn rf_signal_ack(&self) -> bool {
        self.rf_signal_ack_raw()
    }
    
    /// Get raw value of rf_signalAck
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rf_signal_ack_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rf_signalAck
    #[inline(always)]
    pub fn set_rf_signal_ack(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuRfAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for PcuRfAck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PcuRfAck")
                .field("rf_signal_ack", &self.rf_signal_ack())
            .finish()
        } else {
            f.debug_tuple("PcuRfAck").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PcuRfAck {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let rf_signal_ack = u.int_in_range(0..=1)? == 1;
        PcuRfAck::new(rf_signal_ack).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// EmbeddedAliveCheck
///
/// - ID: 310 (0x136)
/// - Size: 0 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct EmbeddedAliveCheck {
    raw: [u8; 0],
}

impl EmbeddedAliveCheck {
    pub const MESSAGE_ID: u32 = 310;
    
    
    /// Construct new EmbeddedAliveCheck from values
    pub fn new() -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 0] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 0] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for EmbeddedAliveCheck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 0 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 0];
        raw.copy_from_slice(&payload[..0]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for EmbeddedAliveCheck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("EmbeddedAliveCheck")
            .finish()
        } else {
            f.debug_tuple("EmbeddedAliveCheck").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for EmbeddedAliveCheck {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        EmbeddedAliveCheck::new().map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CalibRegen
///
/// - ID: 328 (0x148)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct CalibRegen {
    raw: [u8; 1],
}

impl CalibRegen {
    pub const MESSAGE_ID: u32 = 328;
    
    pub const ARRIVE_MIN: u8 = 0_u8;
    pub const ARRIVE_MAX: u8 = 1_u8;
    
    /// Construct new CalibRegen from values
    pub fn new(arrive: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_arrive(arrive)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Get raw value of arrive
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn arrive_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    pub fn arrive(&mut self) -> Result<CalibRegenArrive, CanError> {
        match self.arrive_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 328, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of arrive
    #[inline(always)]
    fn set_arrive(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 328 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CalibRegen {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CalibRegen {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CalibRegen")
            .finish()
        } else {
            f.debug_tuple("CalibRegen").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CalibRegen {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let arrive = u.int_in_range(0..=1)?;
        CalibRegen::new(arrive).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal CalibRegen
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CalibRegenArrive {
}


/// CalibRegenAck
///
/// - ID: 329 (0x149)
/// - Size: 1 bytes
/// - Transmitter: SMUF
#[derive(Clone, Copy)]
pub struct CalibRegenAck {
    raw: [u8; 1],
}

impl CalibRegenAck {
    pub const MESSAGE_ID: u32 = 329;
    
    pub const ARRIVE_MIN: u8 = 0_u8;
    pub const ARRIVE_MAX: u8 = 1_u8;
    
    /// Construct new CalibRegenAck from values
    pub fn new(arrive: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_arrive(arrive)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Get raw value of arrive
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn arrive_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    pub fn arrive(&mut self) -> Result<CalibRegenAckArrive, CanError> {
        match self.arrive_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: 329, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of arrive
    #[inline(always)]
    fn set_arrive(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 329 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CalibRegenAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CalibRegenAck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CalibRegenAck")
            .finish()
        } else {
            f.debug_tuple("CalibRegenAck").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CalibRegenAck {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let arrive = u.int_in_range(0..=1)?;
        CalibRegenAck::new(arrive).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for multiplexed signal CalibRegenAck
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CalibRegenAckArrive {
}


/// PcuAdc1
///
/// - ID: 331 (0x14b)
/// - Size: 6 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuAdc1 {
    raw: [u8; 6],
}

impl PcuAdc1 {
    pub const MESSAGE_ID: u32 = 331;
    
    pub const ADC_24V_MIN: f32 = 0_f32;
    pub const ADC_24V_MAX: f32 = 30_f32;
    pub const ADC_PUMPL_MIN: f32 = 0_f32;
    pub const ADC_PUMPL_MAX: f32 = 30_f32;
    pub const ADC_PUMPR_MIN: f32 = 0_f32;
    pub const ADC_PUMPR_MAX: f32 = 30_f32;
    
    /// Construct new PcuAdc1 from values
    pub fn new(adc_24v: f32, adc_pumpl: f32, adc_pumpr: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 6] };
        res.set_adc_24v(adc_24v)?;
        res.set_adc_pumpl(adc_pumpl)?;
        res.set_adc_pumpr(adc_pumpr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 6] {
        &self.raw
    }
    
    /// adc_24v
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_24v(&self) -> f32 {
        self.adc_24v_raw()
    }
    
    /// Get raw value of adc_24v
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_24v_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_24v
    #[inline(always)]
    pub fn set_adc_24v(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 331 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// adc_pumpl
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_pumpl(&self) -> f32 {
        self.adc_pumpl_raw()
    }
    
    /// Get raw value of adc_pumpl
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_pumpl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_pumpl
    #[inline(always)]
    pub fn set_adc_pumpl(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 331 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// adc_pumpr
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_pumpr(&self) -> f32 {
        self.adc_pumpr_raw()
    }
    
    /// Get raw value of adc_pumpr
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_pumpr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_pumpr
    #[inline(always)]
    pub fn set_adc_pumpr(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 331 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuAdc1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 6 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 6];
        raw.copy_from_slice(&payload[..6]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for PcuAdc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PcuAdc1")
                .field("adc_24v", &self.adc_24v())
                .field("adc_pumpl", &self.adc_pumpl())
                .field("adc_pumpr", &self.adc_pumpr())
            .finish()
        } else {
            f.debug_tuple("PcuAdc1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PcuAdc1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let adc_24v = u.float_in_range(0_f32..=30_f32)?;
        let adc_pumpl = u.float_in_range(0_f32..=30_f32)?;
        let adc_pumpr = u.float_in_range(0_f32..=30_f32)?;
        PcuAdc1::new(adc_24v,adc_pumpl,adc_pumpr).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// PcuAdc2
///
/// - ID: 332 (0x14c)
/// - Size: 8 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuAdc2 {
    raw: [u8; 8],
}

impl PcuAdc2 {
    pub const MESSAGE_ID: u32 = 332;
    
    pub const ADC_FANBATTL_MIN: f32 = 0_f32;
    pub const ADC_FANBATTL_MAX: f32 = 30_f32;
    pub const ADC_FANBATTR_MIN: f32 = 0_f32;
    pub const ADC_FANBATTR_MAX: f32 = 30_f32;
    pub const ADC_FANRADL_MIN: f32 = 0_f32;
    pub const ADC_FANRADL_MAX: f32 = 30_f32;
    pub const ADC_FANRADR_MIN: f32 = 0_f32;
    pub const ADC_FANRADR_MAX: f32 = 30_f32;
    
    /// Construct new PcuAdc2 from values
    pub fn new(adc_fanbattl: f32, adc_fanbattr: f32, adc_fanradl: f32, adc_fanradr: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_adc_fanbattl(adc_fanbattl)?;
        res.set_adc_fanbattr(adc_fanbattr)?;
        res.set_adc_fanradl(adc_fanradl)?;
        res.set_adc_fanradr(adc_fanradr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// adc_fanbattl
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_fanbattl(&self) -> f32 {
        self.adc_fanbattl_raw()
    }
    
    /// Get raw value of adc_fanbattl
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_fanbattl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_fanbattl
    #[inline(always)]
    pub fn set_adc_fanbattl(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 332 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// adc_fanbattr
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_fanbattr(&self) -> f32 {
        self.adc_fanbattr_raw()
    }
    
    /// Get raw value of adc_fanbattr
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_fanbattr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_fanbattr
    #[inline(always)]
    pub fn set_adc_fanbattr(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 332 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// adc_fanradl
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_fanradl(&self) -> f32 {
        self.adc_fanradl_raw()
    }
    
    /// Get raw value of adc_fanradl
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_fanradl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_fanradl
    #[inline(always)]
    pub fn set_adc_fanradl(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 332 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// adc_fanradr
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_fanradr(&self) -> f32 {
        self.adc_fanradr_raw()
    }
    
    /// Get raw value of adc_fanradr
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_fanradr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_fanradr
    #[inline(always)]
    pub fn set_adc_fanradr(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 332 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuAdc2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for PcuAdc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PcuAdc2")
                .field("adc_fanbattl", &self.adc_fanbattl())
                .field("adc_fanbattr", &self.adc_fanbattr())
                .field("adc_fanradl", &self.adc_fanradl())
                .field("adc_fanradr", &self.adc_fanradr())
            .finish()
        } else {
            f.debug_tuple("PcuAdc2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PcuAdc2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let adc_fanbattl = u.float_in_range(0_f32..=30_f32)?;
        let adc_fanbattr = u.float_in_range(0_f32..=30_f32)?;
        let adc_fanradl = u.float_in_range(0_f32..=30_f32)?;
        let adc_fanradr = u.float_in_range(0_f32..=30_f32)?;
        PcuAdc2::new(adc_fanbattl,adc_fanbattr,adc_fanradl,adc_fanradr).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// PcuAdc3
///
/// - ID: 333 (0x14d)
/// - Size: 6 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuAdc3 {
    raw: [u8; 6],
}

impl PcuAdc3 {
    pub const MESSAGE_ID: u32 = 333;
    
    pub const ADC_DV_MIN: f32 = 0_f32;
    pub const ADC_DV_MAX: f32 = 30_f32;
    pub const ADC_EMB_MIN: f32 = 0_f32;
    pub const ADC_EMB_MAX: f32 = 30_f32;
    pub const ADC_STEERACT_MIN: f32 = 0_f32;
    pub const ADC_STEERACT_MAX: f32 = 30_f32;
    
    /// Construct new PcuAdc3 from values
    pub fn new(adc_dv: f32, adc_emb: f32, adc_steeract: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 6] };
        res.set_adc_dv(adc_dv)?;
        res.set_adc_emb(adc_emb)?;
        res.set_adc_steeract(adc_steeract)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 6] {
        &self.raw
    }
    
    /// adc_dv
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_dv(&self) -> f32 {
        self.adc_dv_raw()
    }
    
    /// Get raw value of adc_dv
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_dv_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_dv
    #[inline(always)]
    pub fn set_adc_dv(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 333 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// adc_emb
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_emb(&self) -> f32 {
        self.adc_emb_raw()
    }
    
    /// Get raw value of adc_emb
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_emb_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_emb
    #[inline(always)]
    pub fn set_adc_emb(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 333 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// adc_steeract
    ///
    /// - Min: 0
    /// - Max: 30
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn adc_steeract(&self) -> f32 {
        self.adc_steeract_raw()
    }
    
    /// Get raw value of adc_steeract
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn adc_steeract_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of adc_steeract
    #[inline(always)]
    pub fn set_adc_steeract(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 30_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 333 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuAdc3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 6 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 6];
        raw.copy_from_slice(&payload[..6]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for PcuAdc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("PcuAdc3")
                .field("adc_dv", &self.adc_dv())
                .field("adc_emb", &self.adc_emb())
                .field("adc_steeract", &self.adc_steeract())
            .finish()
        } else {
            f.debug_tuple("PcuAdc3").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for PcuAdc3 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let adc_dv = u.float_in_range(0_f32..=30_f32)?;
        let adc_emb = u.float_in_range(0_f32..=30_f32)?;
        let adc_steeract = u.float_in_range(0_f32..=30_f32)?;
        PcuAdc3::new(adc_dv,adc_emb,adc_steeract).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// coolingControl
///
/// - ID: 334 (0x14e)
/// - Size: 8 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct CoolingControl {
    raw: [u8; 8],
}

impl CoolingControl {
    pub const MESSAGE_ID: u32 = 334;
    
    pub const PWM_FANBATT_MIN: u16 = 0_u16;
    pub const PWM_FANBATT_MAX: u16 = 65535_u16;
    pub const PWM_FANRAD_MIN: u16 = 0_u16;
    pub const PWM_FANRAD_MAX: u16 = 65535_u16;
    pub const PWM_PUMPL_MIN: u16 = 0_u16;
    pub const PWM_PUMPL_MAX: u16 = 65535_u16;
    pub const PWM_PUMPR_MIN: u16 = 0_u16;
    pub const PWM_PUMPR_MAX: u16 = 65535_u16;
    
    /// Construct new coolingControl from values
    pub fn new(pwm_fanbatt: u16, pwm_fanrad: u16, pwm_pumpl: u16, pwm_pumpr: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_pwm_fanbatt(pwm_fanbatt)?;
        res.set_pwm_fanrad(pwm_fanrad)?;
        res.set_pwm_pumpl(pwm_pumpl)?;
        res.set_pwm_pumpr(pwm_pumpr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// pwm_fanbatt
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwm_fanbatt(&self) -> u16 {
        self.pwm_fanbatt_raw()
    }
    
    /// Get raw value of pwm_fanbatt
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwm_fanbatt_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of pwm_fanbatt
    #[inline(always)]
    pub fn set_pwm_fanbatt(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 334 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// pwm_fanrad
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwm_fanrad(&self) -> u16 {
        self.pwm_fanrad_raw()
    }
    
    /// Get raw value of pwm_fanrad
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwm_fanrad_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of pwm_fanrad
    #[inline(always)]
    pub fn set_pwm_fanrad(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 334 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// pwm_pumpl
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwm_pumpl(&self) -> u16 {
        self.pwm_pumpl_raw()
    }
    
    /// Get raw value of pwm_pumpl
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwm_pumpl_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        signal
    }
    
    /// Set value of pwm_pumpl
    #[inline(always)]
    pub fn set_pwm_pumpl(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 334 });
        }
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// pwm_pumpr
    ///
    /// - Min: 0
    /// - Max: 65535
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwm_pumpr(&self) -> u16 {
        self.pwm_pumpr_raw()
    }
    
    /// Get raw value of pwm_pumpr
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwm_pumpr_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        signal
    }
    
    /// Set value of pwm_pumpr
    #[inline(always)]
    pub fn set_pwm_pumpr(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 65535_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 334 });
        }
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CoolingControl {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CoolingControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CoolingControl")
                .field("pwm_fanbatt", &self.pwm_fanbatt())
                .field("pwm_fanrad", &self.pwm_fanrad())
                .field("pwm_pumpl", &self.pwm_pumpl())
                .field("pwm_pumpr", &self.pwm_pumpr())
            .finish()
        } else {
            f.debug_tuple("CoolingControl").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CoolingControl {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let pwm_fanbatt = u.int_in_range(0..=65535)?;
        let pwm_fanrad = u.int_in_range(0..=65535)?;
        let pwm_pumpl = u.int_in_range(0..=65535)?;
        let pwm_pumpr = u.int_in_range(0..=65535)?;
        CoolingControl::new(pwm_fanbatt,pwm_fanrad,pwm_pumpl,pwm_pumpr).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Balancing
///
/// - ID: 420 (0x1a4)
/// - Size: 1 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct Balancing {
    raw: [u8; 1],
}

impl Balancing {
    pub const MESSAGE_ID: u32 = 420;
    
    pub const BALANCING_LV_MIN: i8 = 0_i8;
    pub const BALANCING_LV_MAX: i8 = 1_i8;
    
    /// Construct new Balancing from values
    pub fn new(balancing_lv: i8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_balancing_lv(balancing_lv)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// balancing_lv
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn balancing_lv(&self) -> BalancingBalancingLv {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        match signal {
            1 => BalancingBalancingLv::On,
            0 => BalancingBalancingLv::Off,
            _ => BalancingBalancingLv::_Other(self.balancing_lv_raw()),
        }
    }
    
    /// Get raw value of balancing_lv
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn balancing_lv_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let signal  = i8::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of balancing_lv
    #[inline(always)]
    pub fn set_balancing_lv(&mut self, value: i8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i8 || 1_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 420 });
        }
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Balancing {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Balancing {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Balancing")
                .field("balancing_lv", &self.balancing_lv())
            .finish()
        } else {
            f.debug_tuple("Balancing").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Balancing {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let balancing_lv = u.int_in_range(0..=1)?;
        Balancing::new(balancing_lv).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}
/// Defined values for balancing_lv
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum BalancingBalancingLv {
    On,
    Off,
    _Other(i8),
}

impl From<BalancingBalancingLv> for i8 {
    fn from(val: BalancingBalancingLv) -> i8 {
        match val {
            BalancingBalancingLv::On => 1,
            BalancingBalancingLv::Off => 0,
            BalancingBalancingLv::_Other(x) => x,
        }
    }
}


/// DisplayACK
///
/// - ID: 600 (0x258)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct DisplayAck {
    raw: [u8; 1],
}

impl DisplayAck {
    pub const MESSAGE_ID: u32 = 600;
    
    
    /// Construct new DisplayACK from values
    pub fn new(uart_fault: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_uart_fault(uart_fault)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// UART_fault
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn uart_fault(&self) -> bool {
        self.uart_fault_raw()
    }
    
    /// Get raw value of UART_fault
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn uart_fault_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of UART_fault
    #[inline(always)]
    pub fn set_uart_fault(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for DisplayAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for DisplayAck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("DisplayAck")
                .field("uart_fault", &self.uart_fault())
            .finish()
        } else {
            f.debug_tuple("DisplayAck").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for DisplayAck {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let uart_fault = u.int_in_range(0..=1)? == 1;
        DisplayAck::new(uart_fault).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MapAck
///
/// - ID: 700 (0x2bc)
/// - Size: 2 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct MapAck {
    raw: [u8; 2],
}

impl MapAck {
    pub const MESSAGE_ID: u32 = 700;
    
    pub const POWER_MIN: u8 = 1_u8;
    pub const POWER_MAX: u8 = 12_u8;
    pub const REGEN_MIN: u8 = 1_u8;
    pub const REGEN_MAX: u8 = 12_u8;
    pub const TORQUE_REP_MIN: u8 = 0_u8;
    pub const TORQUE_REP_MAX: u8 = 12_u8;
    
    /// Construct new MapAck from values
    pub fn new(power: u8, regen: u8, torque_rep: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_power(power)?;
        res.set_regen(regen)?;
        res.set_torque_rep(torque_rep)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// power
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn power(&self) -> u8 {
        self.power_raw()
    }
    
    /// Get raw value of power
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn power_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        signal
    }
    
    /// Set value of power
    #[inline(always)]
    pub fn set_power(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 700 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// regen
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn regen(&self) -> u8 {
        self.regen_raw()
    }
    
    /// Get raw value of regen
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn regen_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of regen
    #[inline(always)]
    pub fn set_regen(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 700 });
        }
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    
    /// torque_rep
    ///
    /// - Min: 0
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_rep(&self) -> u8 {
        self.torque_rep_raw()
    }
    
    /// Get raw value of torque_rep
    ///
    /// - Start bit: 8
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_rep_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
        
        signal
    }
    
    /// Set value of torque_rep
    #[inline(always)]
    pub fn set_torque_rep(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 700 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for MapAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for MapAck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("MapAck")
                .field("power", &self.power())
                .field("regen", &self.regen())
                .field("torque_rep", &self.torque_rep())
            .finish()
        } else {
            f.debug_tuple("MapAck").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for MapAck {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let power = u.int_in_range(1..=12)?;
        let regen = u.int_in_range(1..=12)?;
        let torque_rep = u.int_in_range(0..=12)?;
        MapAck::new(power,regen,torque_rep).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// VcuErrTrace
///
/// - ID: 800 (0x320)
/// - Size: 3 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct VcuErrTrace {
    raw: [u8; 3],
}

impl VcuErrTrace {
    pub const MESSAGE_ID: u32 = 800;
    
    pub const CORE_0_ERR_MIN: u8 = 0_u8;
    pub const CORE_0_ERR_MAX: u8 = 255_u8;
    pub const CORE_1_ERR_MIN: u8 = 0_u8;
    pub const CORE_1_ERR_MAX: u8 = 255_u8;
    pub const CORE_2_ERR_MIN: u8 = 0_u8;
    pub const CORE_2_ERR_MAX: u8 = 255_u8;
    
    /// Construct new VcuErrTrace from values
    pub fn new(core_0_err: u8, core_1_err: u8, core_2_err: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_core_0_err(core_0_err)?;
        res.set_core_1_err(core_1_err)?;
        res.set_core_2_err(core_2_err)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// core_0_err
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn core_0_err(&self) -> u8 {
        self.core_0_err_raw()
    }
    
    /// Get raw value of core_0_err
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn core_0_err_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        signal
    }
    
    /// Set value of core_0_err
    #[inline(always)]
    pub fn set_core_0_err(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 800 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// core_1_err
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn core_1_err(&self) -> u8 {
        self.core_1_err_raw()
    }
    
    /// Get raw value of core_1_err
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn core_1_err_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        signal
    }
    
    /// Set value of core_1_err
    #[inline(always)]
    pub fn set_core_1_err(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 800 });
        }
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// core_2_err
    ///
    /// - Min: 0
    /// - Max: 255
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn core_2_err(&self) -> u8 {
        self.core_2_err_raw()
    }
    
    /// Get raw value of core_2_err
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn core_2_err_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        signal
    }
    
    /// Set value of core_2_err
    #[inline(always)]
    pub fn set_core_2_err(&mut self, value: u8) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u8 || 255_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 800 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for VcuErrTrace {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for VcuErrTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("VcuErrTrace")
                .field("core_0_err", &self.core_0_err())
                .field("core_1_err", &self.core_1_err())
                .field("core_2_err", &self.core_2_err())
            .finish()
        } else {
            f.debug_tuple("VcuErrTrace").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for VcuErrTrace {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let core_0_err = u.int_in_range(0..=255)?;
        let core_1_err = u.int_in_range(0..=255)?;
        let core_2_err = u.int_in_range(0..=255)?;
        VcuErrTrace::new(core_0_err,core_1_err,core_2_err).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CsLog_1
///
/// - ID: 820 (0x334)
/// - Size: 8 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CsLog1 {
    raw: [u8; 8],
}

impl CsLog1 {
    pub const MESSAGE_ID: u32 = 820;
    
    pub const YR_REF_MIN: i32 = -5_i32;
    pub const YR_REF_MAX: i32 = 5_i32;
    pub const YAW_MOMENT_MIN: i32 = -4000_i32;
    pub const YAW_MOMENT_MAX: i32 = 4000_i32;
    
    /// Construct new CsLog_1 from values
    pub fn new(yr_ref: i32, yaw_moment: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_yr_ref(yr_ref)?;
        res.set_yaw_moment(yaw_moment)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// yr_ref
    ///
    /// - Min: -5
    /// - Max: 5
    /// - Unit: " rad/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn yr_ref(&self) -> i32 {
        self.yr_ref_raw()
    }
    
    /// Get raw value of yr_ref
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn yr_ref_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of yr_ref
    #[inline(always)]
    pub fn set_yr_ref(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -5_i32 || 5_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 820 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// yaw_moment
    ///
    /// - Min: -4000
    /// - Max: 4000
    /// - Unit: " Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn yaw_moment(&self) -> i32 {
        self.yaw_moment_raw()
    }
    
    /// Get raw value of yaw_moment
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn yaw_moment_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let signal  = i32::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of yaw_moment
    #[inline(always)]
    pub fn set_yaw_moment(&mut self, value: i32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < -4000_i32 || 4000_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 820 });
        }
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CsLog1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CsLog1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CsLog1")
                .field("yr_ref", &self.yr_ref())
                .field("yaw_moment", &self.yaw_moment())
            .finish()
        } else {
            f.debug_tuple("CsLog1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CsLog1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let yr_ref = u.int_in_range(-5..=5)?;
        let yaw_moment = u.int_in_range(-4000..=4000)?;
        CsLog1::new(yr_ref,yaw_moment).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CsLog_2
///
/// - ID: 821 (0x335)
/// - Size: 8 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CsLog2 {
    raw: [u8; 8],
}

impl CsLog2 {
    pub const MESSAGE_ID: u32 = 821;
    
    pub const DELTA_TORQUE_FL_MIN: i16 = 0_i16;
    pub const DELTA_TORQUE_FL_MAX: i16 = 0_i16;
    pub const DELTA_TORQUE_FR_MIN: i16 = 0_i16;
    pub const DELTA_TORQUE_FR_MAX: i16 = 0_i16;
    pub const DELTA_TORQUE_RL_MIN: i16 = 0_i16;
    pub const DELTA_TORQUE_RL_MAX: i16 = 0_i16;
    pub const DELTA_TORQUE_RR_MIN: i16 = 0_i16;
    pub const DELTA_TORQUE_RR_MAX: i16 = 0_i16;
    
    /// Construct new CsLog_2 from values
    pub fn new(delta_torque_fl: i16, delta_torque_fr: i16, delta_torque_rl: i16, delta_torque_rr: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_delta_torque_fl(delta_torque_fl)?;
        res.set_delta_torque_fr(delta_torque_fr)?;
        res.set_delta_torque_rl(delta_torque_rl)?;
        res.set_delta_torque_rr(delta_torque_rr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// delta_torque_fl
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Nmm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn delta_torque_fl(&self) -> i16 {
        self.delta_torque_fl_raw()
    }
    
    /// Get raw value of delta_torque_fl
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn delta_torque_fl_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of delta_torque_fl
    #[inline(always)]
    pub fn set_delta_torque_fl(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 821 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// delta_torque_fr
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Nmm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn delta_torque_fr(&self) -> i16 {
        self.delta_torque_fr_raw()
    }
    
    /// Get raw value of delta_torque_fr
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn delta_torque_fr_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of delta_torque_fr
    #[inline(always)]
    pub fn set_delta_torque_fr(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 821 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// delta_torque_rl
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Nmm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn delta_torque_rl(&self) -> i16 {
        self.delta_torque_rl_raw()
    }
    
    /// Get raw value of delta_torque_rl
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn delta_torque_rl_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of delta_torque_rl
    #[inline(always)]
    pub fn set_delta_torque_rl(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 821 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// delta_torque_rr
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: " Nmm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn delta_torque_rr(&self) -> i16 {
        self.delta_torque_rr_raw()
    }
    
    /// Get raw value of delta_torque_rr
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn delta_torque_rr_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let signal  = i16::from_ne_bytes(signal.to_ne_bytes());
        signal
    }
    
    /// Set value of delta_torque_rr
    #[inline(always)]
    pub fn set_delta_torque_rr(&mut self, value: i16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_i16 || 0_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 821 });
        }
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CsLog2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CsLog2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CsLog2")
                .field("delta_torque_fl", &self.delta_torque_fl())
                .field("delta_torque_fr", &self.delta_torque_fr())
                .field("delta_torque_rl", &self.delta_torque_rl())
                .field("delta_torque_rr", &self.delta_torque_rr())
            .finish()
        } else {
            f.debug_tuple("CsLog2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CsLog2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let delta_torque_fl = u.int_in_range(0..=0)?;
        let delta_torque_fr = u.int_in_range(0..=0)?;
        let delta_torque_rl = u.int_in_range(0..=0)?;
        let delta_torque_rr = u.int_in_range(0..=0)?;
        CsLog2::new(delta_torque_fl,delta_torque_fr,delta_torque_rl,delta_torque_rr).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// CsLog_3
///
/// - ID: 822 (0x336)
/// - Size: 8 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CsLog3 {
    raw: [u8; 8],
}

impl CsLog3 {
    pub const MESSAGE_ID: u32 = 822;
    
    pub const VX_MIN: f32 = 0_f32;
    pub const VX_MAX: f32 = 50_f32;
    
    /// Construct new CsLog_3 from values
    pub fn new(vx: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_vx(vx)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// vx
    ///
    /// - Min: 0
    /// - Max: 50
    /// - Unit: " m/s"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn vx(&self) -> f32 {
        self.vx_raw()
    }
    
    /// Get raw value of vx
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn vx_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of vx
    #[inline(always)]
    pub fn set_vx(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 50_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 822 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CsLog3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for CsLog3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("CsLog3")
                .field("vx", &self.vx())
            .finish()
        } else {
            f.debug_tuple("CsLog3").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for CsLog3 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let vx = u.float_in_range(0_f32..=50_f32)?;
        CsLog3::new(vx).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLvCellControl
///
/// - ID: 869 (0x365)
/// - Size: 1 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLvCellControl {
    raw: [u8; 1],
}

impl BmsLvCellControl {
    pub const MESSAGE_ID: u32 = 869;
    
    
    /// Construct new BmsLvCellControl from values
    pub fn new(enable: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_enable(enable)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// enable
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn enable(&self) -> bool {
        self.enable_raw()
    }
    
    /// Get raw value of enable
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn enable_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of enable
    #[inline(always)]
    pub fn set_enable(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLvCellControl {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLvCellControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLvCellControl")
                .field("enable", &self.enable())
            .finish()
        } else {
            f.debug_tuple("BmsLvCellControl").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLvCellControl {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let enable = u.int_in_range(0..=1)? == 1;
        BmsLvCellControl::new(enable).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLvCell_1
///
/// - ID: 870 (0x366)
/// - Size: 8 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLvCell1 {
    raw: [u8; 8],
}

impl BmsLvCell1 {
    pub const MESSAGE_ID: u32 = 870;
    
    pub const CELL_0_MIN: f32 = 0_f32;
    pub const CELL_0_MAX: f32 = 0_f32;
    pub const CELL_1_MIN: f32 = 0_f32;
    pub const CELL_1_MAX: f32 = 0_f32;
    pub const CELL_2_MIN: f32 = 0_f32;
    pub const CELL_2_MAX: f32 = 0_f32;
    pub const CELL_3_MIN: f32 = 0_f32;
    pub const CELL_3_MAX: f32 = 0_f32;
    
    /// Construct new BmsLvCell_1 from values
    pub fn new(cell_0: f32, cell_1: f32, cell_2: f32, cell_3: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_cell_0(cell_0)?;
        res.set_cell_1(cell_1)?;
        res.set_cell_2(cell_2)?;
        res.set_cell_3(cell_3)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// cell_0
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_0(&self) -> f32 {
        self.cell_0_raw()
    }
    
    /// Get raw value of cell_0
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_0_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_0
    #[inline(always)]
    pub fn set_cell_0(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 870 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// cell_1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_1(&self) -> f32 {
        self.cell_1_raw()
    }
    
    /// Get raw value of cell_1
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_1
    #[inline(always)]
    pub fn set_cell_1(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 870 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// cell_2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_2(&self) -> f32 {
        self.cell_2_raw()
    }
    
    /// Get raw value of cell_2
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_2
    #[inline(always)]
    pub fn set_cell_2(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 870 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// cell_3
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_3(&self) -> f32 {
        self.cell_3_raw()
    }
    
    /// Get raw value of cell_3
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_3_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_3
    #[inline(always)]
    pub fn set_cell_3(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 870 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLvCell1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLvCell1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLvCell1")
                .field("cell_0", &self.cell_0())
                .field("cell_1", &self.cell_1())
                .field("cell_2", &self.cell_2())
                .field("cell_3", &self.cell_3())
            .finish()
        } else {
            f.debug_tuple("BmsLvCell1").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLvCell1 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let cell_0 = u.float_in_range(0_f32..=0_f32)?;
        let cell_1 = u.float_in_range(0_f32..=0_f32)?;
        let cell_2 = u.float_in_range(0_f32..=0_f32)?;
        let cell_3 = u.float_in_range(0_f32..=0_f32)?;
        BmsLvCell1::new(cell_0,cell_1,cell_2,cell_3).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLvCell_2
///
/// - ID: 871 (0x367)
/// - Size: 8 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLvCell2 {
    raw: [u8; 8],
}

impl BmsLvCell2 {
    pub const MESSAGE_ID: u32 = 871;
    
    pub const CELL_4_MIN: f32 = 0_f32;
    pub const CELL_4_MAX: f32 = 0_f32;
    pub const CELL_5_MIN: f32 = 0_f32;
    pub const CELL_5_MAX: f32 = 0_f32;
    pub const CELL_6_MIN: f32 = 0_f32;
    pub const CELL_6_MAX: f32 = 0_f32;
    pub const CELL_7_MIN: f32 = 0_f32;
    pub const CELL_7_MAX: f32 = 0_f32;
    
    /// Construct new BmsLvCell_2 from values
    pub fn new(cell_4: f32, cell_5: f32, cell_6: f32, cell_7: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_cell_4(cell_4)?;
        res.set_cell_5(cell_5)?;
        res.set_cell_6(cell_6)?;
        res.set_cell_7(cell_7)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// cell_4
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_4(&self) -> f32 {
        self.cell_4_raw()
    }
    
    /// Get raw value of cell_4
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_4_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_4
    #[inline(always)]
    pub fn set_cell_4(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 871 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// cell_5
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_5(&self) -> f32 {
        self.cell_5_raw()
    }
    
    /// Get raw value of cell_5
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_5_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_5
    #[inline(always)]
    pub fn set_cell_5(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 871 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// cell_6
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_6(&self) -> f32 {
        self.cell_6_raw()
    }
    
    /// Get raw value of cell_6
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_6_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_6
    #[inline(always)]
    pub fn set_cell_6(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 871 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// cell_7
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_7(&self) -> f32 {
        self.cell_7_raw()
    }
    
    /// Get raw value of cell_7
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_7_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_7
    #[inline(always)]
    pub fn set_cell_7(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 871 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLvCell2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLvCell2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLvCell2")
                .field("cell_4", &self.cell_4())
                .field("cell_5", &self.cell_5())
                .field("cell_6", &self.cell_6())
                .field("cell_7", &self.cell_7())
            .finish()
        } else {
            f.debug_tuple("BmsLvCell2").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLvCell2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let cell_4 = u.float_in_range(0_f32..=0_f32)?;
        let cell_5 = u.float_in_range(0_f32..=0_f32)?;
        let cell_6 = u.float_in_range(0_f32..=0_f32)?;
        let cell_7 = u.float_in_range(0_f32..=0_f32)?;
        BmsLvCell2::new(cell_4,cell_5,cell_6,cell_7).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLvCell_3
///
/// - ID: 872 (0x368)
/// - Size: 8 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLvCell3 {
    raw: [u8; 8],
}

impl BmsLvCell3 {
    pub const MESSAGE_ID: u32 = 872;
    
    pub const CELL_8_MIN: f32 = 0_f32;
    pub const CELL_8_MAX: f32 = 0_f32;
    pub const CELL_9_MIN: f32 = 0_f32;
    pub const CELL_9_MAX: f32 = 0_f32;
    pub const CELL_10_MIN: f32 = 0_f32;
    pub const CELL_10_MAX: f32 = 0_f32;
    pub const CELL_12_MIN: f32 = 0_f32;
    pub const CELL_12_MAX: f32 = 0_f32;
    
    /// Construct new BmsLvCell_3 from values
    pub fn new(cell_8: f32, cell_9: f32, cell_10: f32, cell_12: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_cell_8(cell_8)?;
        res.set_cell_9(cell_9)?;
        res.set_cell_10(cell_10)?;
        res.set_cell_12(cell_12)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// cell_8
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_8(&self) -> f32 {
        self.cell_8_raw()
    }
    
    /// Get raw value of cell_8
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_8_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_8
    #[inline(always)]
    pub fn set_cell_8(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 872 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// cell_9
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_9(&self) -> f32 {
        self.cell_9_raw()
    }
    
    /// Get raw value of cell_9
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_9_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_9
    #[inline(always)]
    pub fn set_cell_9(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 872 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// cell_10
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_10(&self) -> f32 {
        self.cell_10_raw()
    }
    
    /// Get raw value of cell_10
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_10_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_10
    #[inline(always)]
    pub fn set_cell_10(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 872 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// cell_12
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn cell_12(&self) -> f32 {
        self.cell_12_raw()
    }
    
    /// Get raw value of cell_12
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn cell_12_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of cell_12
    #[inline(always)]
    pub fn set_cell_12(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 872 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLvCell3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLvCell3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLvCell3")
                .field("cell_8", &self.cell_8())
                .field("cell_9", &self.cell_9())
                .field("cell_10", &self.cell_10())
                .field("cell_12", &self.cell_12())
            .finish()
        } else {
            f.debug_tuple("BmsLvCell3").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLvCell3 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let cell_8 = u.float_in_range(0_f32..=0_f32)?;
        let cell_9 = u.float_in_range(0_f32..=0_f32)?;
        let cell_10 = u.float_in_range(0_f32..=0_f32)?;
        let cell_12 = u.float_in_range(0_f32..=0_f32)?;
        BmsLvCell3::new(cell_8,cell_9,cell_10,cell_12).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// BmsLvTemps
///
/// - ID: 873 (0x369)
/// - Size: 8 bytes
/// - Transmitter: BMSLV
#[derive(Clone, Copy)]
pub struct BmsLvTemps {
    raw: [u8; 8],
}

impl BmsLvTemps {
    pub const MESSAGE_ID: u32 = 873;
    
    pub const TEMP_0_MIN: f32 = 0_f32;
    pub const TEMP_0_MAX: f32 = 0_f32;
    pub const TEMP_1_MIN: f32 = 0_f32;
    pub const TEMP_1_MAX: f32 = 0_f32;
    pub const TEMP_2_MIN: f32 = 0_f32;
    pub const TEMP_2_MAX: f32 = 0_f32;
    pub const TEMP_3_MIN: f32 = 0_f32;
    pub const TEMP_3_MAX: f32 = 0_f32;
    
    /// Construct new BmsLvTemps from values
    pub fn new(temp_0: f32, temp_1: f32, temp_2: f32, temp_3: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_temp_0(temp_0)?;
        res.set_temp_1(temp_1)?;
        res.set_temp_2(temp_2)?;
        res.set_temp_3(temp_3)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// temp_0
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn temp_0(&self) -> f32 {
        self.temp_0_raw()
    }
    
    /// Get raw value of temp_0
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_0_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of temp_0
    #[inline(always)]
    pub fn set_temp_0(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 873 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// temp_1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn temp_1(&self) -> f32 {
        self.temp_1_raw()
    }
    
    /// Get raw value of temp_1
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_1_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of temp_1
    #[inline(always)]
    pub fn set_temp_1(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 873 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// temp_2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn temp_2(&self) -> f32 {
        self.temp_2_raw()
    }
    
    /// Get raw value of temp_2
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_2_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of temp_2
    #[inline(always)]
    pub fn set_temp_2(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 873 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// temp_3
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: BMSLV
    #[inline(always)]
    pub fn temp_3(&self) -> f32 {
        self.temp_3_raw()
    }
    
    /// Get raw value of temp_3
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_3_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of temp_3
    #[inline(always)]
    pub fn set_temp_3(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 873 });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLvTemps {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for BmsLvTemps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("BmsLvTemps")
                .field("temp_0", &self.temp_0())
                .field("temp_1", &self.temp_1())
                .field("temp_2", &self.temp_2())
                .field("temp_3", &self.temp_3())
            .finish()
        } else {
            f.debug_tuple("BmsLvTemps").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for BmsLvTemps {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let temp_0 = u.float_in_range(0_f32..=0_f32)?;
        let temp_1 = u.float_in_range(0_f32..=0_f32)?;
        let temp_2 = u.float_in_range(0_f32..=0_f32)?;
        let temp_3 = u.float_in_range(0_f32..=0_f32)?;
        BmsLvTemps::new(temp_0,temp_1,temp_2,temp_3).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// Lem
///
/// - ID: 962 (0x3c2)
/// - Size: 8 bytes
/// - Transmitter: LEM
#[derive(Clone, Copy)]
pub struct Lem {
    raw: [u8; 8],
}

impl Lem {
    pub const MESSAGE_ID: u32 = 962;
    
    pub const CURRENT_MIN: f32 = 0_f32;
    pub const CURRENT_MAX: f32 = 0_f32;
    
    /// Construct new Lem from values
    pub fn new(current: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_current(current)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// current
    ///
    /// Current seen from LEM on car side (PDBe
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mA"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn current(&self) -> f32 {
        self.current_raw()
    }
    
    /// Get raw value of current
    ///
    /// - Start bit: 7
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: -2147483648
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..32].load_be::<u32>();
        
        let factor = 1_f32;
        let offset = -2147483648_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of current
    #[inline(always)]
    pub fn set_current(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 962 });
        }
        let factor = 1_f32;
        let offset = -2147483648_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Msb0>()[0..32].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Lem {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Lem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Lem")
                .field("current", &self.current())
            .finish()
        } else {
            f.debug_tuple("Lem").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Lem {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let current = u.float_in_range(0_f32..=0_f32)?;
        Lem::new(current).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}


/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", feature = "std"), derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: u32,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for CanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}

