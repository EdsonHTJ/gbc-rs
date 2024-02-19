

/*
    0x00 - 0x00: JoyPad
    0x01 - 0x01: Serial Transfer Data
    0x02 - 0x02: Serial Transfer Control
    0x04 - 0x04: Divider Register
    0x05 - 0x05: Timer Counter
    0x06 - 0x06: Timer Modulo
    0x07 - 0x0E: Timer Control
    0x0F - 0x0F: Interrupt Flags
    0x10 - 0x10: Sound Mode 1 Sweep
    0x11 - 0x11: Sound Mode 1 Length/Wave
    0x12 - 0x12: Sound Mode 1 Volume
    0x13 - 0x13: Sound Mode 1 Period Low
    0x14 - 0x14: Sound Mode 1 Period High
    0x16 - 0x16: Sound Mode 2 Length/Wave
    0x17 - 0x17: Sound Mode 2 Volume
    0x18 - 0x18: Sound Mode 2 Period Low
    0x19 - 0x19: Sound Mode 2 Period High
    0x1A - 0x1A: Sound Mode 3 DAC Enable
    0x1B - 0x1B: Sound Mode 3 Length
    0x1C - 0x1C: Sound Mode 3 Output Level
    0x1D - 0x1D: Sound Mode 3 Period Low
    0x1E - 0x1E: Sound Mode 3 Period High
    0x20 - 0x20: Sound Mode 4 Length timer
    0x21 - 0x21: Sound Mode 4 Volume
    0x22 - 0x22: Sound Mode 4 Frequency / Randomness
    0x23 - 0x23: Sound Mode 4 Control
    0x24 - 0x24: Master Volume Control
    0x25 - 0x25: Sound Panning
    0x26 - 0x26: Sound On/Off
    0x40 - 0x40: LCD Control
    0x41 - 0x41: LCD Status
    0x42 - 0x42: Viewport Y
    0x43 - 0x43: Viewport X
    0x44 - 0x44: LDC Y-Coordinate
    0x45 - 0x45: LCD Y-Coordinate Compare
    0x46 - 0x46: DMA Transfer
    0x47 - 0x47: BG Palette Data
    0x48 - 0x48: Object Palette 0 Data
    0x49 - 0x49: Object Palette 1 Data
    0x4A - 0x4A: Window Y Position
    0x4B - 0x4C: Window X Position Plus 7
    0x4D - 0x4D: Speed Switch
    0x4F - 0x50: VRAM Bank
    0x51 - 0x51: VRAM SOURCE HIGH
    0x52 - 0x52: VRAM SOURCE LOW
    0x53 - 0x53: VRAM DESTINATION HIGH
    0x54 - 0x54: VRAM DESTINATION LOW
    0x55 - 0x55: VRAM DMA
    0x56 - 0x67: RP
    0x68 - 0x68: BCPS
    0x69 - 0x69: BCPD
    0x6A - 0x6A: OCPS
    0x6B - 0x6B: OCPD
    0x6C - 0x6F: OPRI
    0x70 - 0x75: WRAM Bank
    0x76 - 0x76: PCM12
    0x77 - 0x77: PCM34
    0xFF - 0xFF: Interrupt Enable
*/

use crate::io::IoError;

pub enum IoRegions {
    JoyPad,
    SerialTransferData,
    SerialTransferControl,
    DividerRegister,
    TimerCounter,
    TimerModulo,
    TimerControl,
    InterruptFlags,
    SoundMode1Sweep,
    SoundMode1LengthWave,
    SoundMode1Volume,
    SoundMode1PeriodLow,
    SoundMode1PeriodHigh,
    SoundMode2LengthWave,
    SoundMode2Volume,
    SoundMode2PeriodLow,
    SoundMode2PeriodHigh,
    SoundMode3DACEnable,
    SoundMode3Length,
    SoundMode3OutputLevel,
    SoundMode3PeriodLow,
    SoundMode3PeriodHigh,
    SoundMode4LengthTimer,
    SoundMode4Volume,
    SoundMode4FrequencyRandomness,
    SoundMode4Control,
    MasterVolumeControl,
    SoundPanning,
    SoundOnOff,
    LCDControl,
    LCDStatus,
    ViewportY,
    ViewportX,
    LCDYCoordinate,
    LCDYCoordinateCompare,
    DMATransfer,
    BGPaletteData,
    ObjectPalette0Data,
    ObjectPalette1Data,
    WindowYPosition,
    WindowXPositionPlus7,
    SpeedSwitch,
    VRAMBank,
    VRAMSourceHigh,
    VRAMSourceLow,
    VRAMDestinationHigh,
    VRAMDestinationLow,
    VRAMDMA,
    RP,
    BCPS,
    BCPD,
    OCPS,
    OCPD,
    OPRI,
    WRAMBank,
    PCM12,
    PCM34,
    InterruptEnable,
}

impl IoRegions {
    pub fn from_u8_address(address: u8) -> Result<IoRegions, IoError> {
        match address {
            0x00 => Ok(IoRegions::JoyPad),
            0x01 => Ok(IoRegions::SerialTransferData),
            0x02 => Ok(IoRegions::SerialTransferControl),
            0x04 => Ok(IoRegions::DividerRegister),
            0x05 => Ok(IoRegions::TimerCounter),
            0x06 => Ok(IoRegions::TimerModulo),
            0x07 => Ok(IoRegions::TimerControl),
            0x0F => Ok(IoRegions::InterruptFlags),
            0x10 => Ok(IoRegions::SoundMode1Sweep),
            0x11 => Ok(IoRegions::SoundMode1LengthWave),
            0x12 => Ok(IoRegions::SoundMode1Volume),
            0x13 => Ok(IoRegions::SoundMode1PeriodLow),
            0x14 => Ok(IoRegions::SoundMode1PeriodHigh),
            0x16 => Ok(IoRegions::SoundMode2LengthWave),
            0x17 => Ok(IoRegions::SoundMode2Volume),
            0x18 => Ok(IoRegions::SoundMode2PeriodLow),
            0x19 => Ok(IoRegions::SoundMode2PeriodHigh),
            0x1A => Ok(IoRegions::SoundMode3DACEnable),
            0x1B => Ok(IoRegions::SoundMode3Length),
            0x1C => Ok(IoRegions::SoundMode3OutputLevel),
            0x1D => Ok(IoRegions::SoundMode3PeriodLow),
            0x1E => Ok(IoRegions::SoundMode3PeriodHigh),
            0x20 => Ok(IoRegions::SoundMode4LengthTimer),
            0x21 => Ok(IoRegions::SoundMode4Volume),
            0x22 => Ok(IoRegions::SoundMode4FrequencyRandomness),
            0x23 => Ok(IoRegions::SoundMode4Control),
            0x24 => Ok(IoRegions::MasterVolumeControl),
            0x25 => Ok(IoRegions::SoundPanning),
            0x26 => Ok(IoRegions::SoundOnOff),
            0x40 => Ok(IoRegions::LCDControl),
            0x41 => Ok(IoRegions::LCDStatus),
            0x42 => Ok(IoRegions::ViewportY),
            0x43 => Ok(IoRegions::ViewportX),
            0x44 => Ok(IoRegions::LCDYCoordinate),
            0x45 => Ok(IoRegions::LCDYCoordinateCompare),
            0x46 => Ok(IoRegions::DMATransfer),
            0x47 => Ok(IoRegions::BGPaletteData),
            0x48 => Ok(IoRegions::ObjectPalette0Data),
            0x49 => Ok(IoRegions::ObjectPalette1Data),
            0x4A => Ok(IoRegions::WindowYPosition),
            0x4B => Ok(IoRegions::WindowXPositionPlus7),
            0x4D => Ok(IoRegions::SpeedSwitch),
            0x4F => Ok(IoRegions::VRAMBank),
            0x51 => Ok(IoRegions::VRAMSourceHigh),
            0x52 => Ok(IoRegions::VRAMSourceLow),
            0x53 => Ok(IoRegions::VRAMDestinationHigh),
            0x54 => Ok(IoRegions::VRAMDestinationLow),
            0x55 => Ok(IoRegions::VRAMDMA),
            0x56..=0x67 => Ok(IoRegions::RP),
            0x68 => Ok(IoRegions::BCPS),
            0x69 => Ok(IoRegions::BCPD),
            0x6A => Ok(IoRegions::OCPS),
            0x6B => Ok(IoRegions::OCPD),
            0x6C..=0x6F => Ok(IoRegions::OPRI),
            0x70..=0x75 => Ok(IoRegions::WRAMBank),
            0x76 => Ok(IoRegions::PCM12),
            0x77 => Ok(IoRegions::PCM34),
            0xFF => Ok(IoRegions::InterruptEnable),
            _ => Ok(IoRegions::PCM34),
        }
    }
}