mod fault;

pub use crate::registers::{
    REG00::{IINLIM::Value as InputCurrentLimit, VINDPM::Value as InputVoltageLimit},
    REG05::WATCHDOG::Value as WatchdogTimer,
    REG08::{CHRG_STAT::Value as ChargerStatus, VBUS_STAT::Value as VBusStatus},
};

pub use fault::*;
