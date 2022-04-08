use tock_registers::LocalRegisterCopy;

use crate::registers::REG09;

pub use crate::registers::REG09::{
    BAT_FAULT::Value as BatFault, BOOST_FAULT::Value as BoostFault,
    CHRG_FAULT::Value as ChargeFault, NTC_FAULT::Value as NtcFault,
    WATCHDOG_FAULT::Value as WatchdogFault,
};

pub struct Faults {
    pub watchdog: WatchdogFault,
    pub boost: BoostFault,
    pub charge: ChargeFault,
    pub bat: BatFault,
    pub ntc: NtcFault,
}

impl Faults {
    /// Returns true if there are any active faults
    pub fn fault_active(&self) -> bool {
        match self.watchdog {
            WatchdogFault::Normal => {}
            WatchdogFault::TimerExpired => return true,
        }

        match self.boost {
            BoostFault::Normal => {}
            BoostFault::Fault => return true,
        }

        match self.charge {
            ChargeFault::Normal => {}
            _ => return true,
        }

        match self.bat {
            BatFault::Normal => {}
            BatFault::Overvoltage => return true,
        }

        match self.ntc {
            NtcFault::Normal => {}
            _ => return true,
        }

        false
    }
}

impl TryFrom<LocalRegisterCopy<u8, REG09::Register>> for Faults {
    type Error = ();

    fn try_from(reg: LocalRegisterCopy<u8, REG09::Register>) -> Result<Self, Self::Error> {
        Ok(Faults {
            watchdog: reg.read_as_enum(REG09::WATCHDOG_FAULT).ok_or(())?,
            boost: reg.read_as_enum(REG09::BOOST_FAULT).ok_or(())?,
            charge: reg.read_as_enum(REG09::CHRG_FAULT).ok_or(())?,
            bat: reg.read_as_enum(REG09::BAT_FAULT).ok_or(())?,
            ntc: reg.read_as_enum(REG09::NTC_FAULT).ok_or(())?,
        })
    }
}

impl core::fmt::Debug for Faults {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut set = f.debug_set();

        if matches!(self.watchdog, WatchdogFault::TimerExpired) {
            set.entry(&"WatchdogTimerExpired");
        }

        if matches!(self.boost, BoostFault::Fault) {
            set.entry(&"BoostFault");
        }

        match self.charge {
            ChargeFault::Normal => {}
            ChargeFault::InputFault => {
                set.entry(&"ChargeInputFault");
            }
            ChargeFault::ThermalShutdown => {
                set.entry(&"ChargeThermalShutdown");
            }
            ChargeFault::SafetyTimer => {
                set.entry(&"ChargeSafetyTimer");
            }
        }

        if matches!(self.bat, BatFault::Overvoltage) {
            set.entry(&"BatteryOvervoltage");
        }

        match self.ntc {
            NtcFault::Normal => {}
            NtcFault::Cold => {
                set.entry(&"NtcCold");
            }
            NtcFault::Hot => {
                set.entry(&"NtcHot");
            }
        }

        set.finish()
    }
}
