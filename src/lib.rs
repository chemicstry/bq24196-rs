#![no_std]
#![doc = include_str!("../README.md")]

mod registers;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use registers::I2cRegister;
use tock_registers::{register_bitfields, register_bitmasks, fields::Field, RegisterLongName, LocalRegisterCopy};

const ADDR: u8 = 0x6B;

pub struct BQ24196<I2C> {
    i2c: I2C
}

pub enum ChargerFault {
    /// Input fault (VBUS OVP or VBAT < VBUS < 3.8 V)
    InputFault,
    /// Thermal shutdown
    ThermalShutdown,
    /// Charge Safety Timer Expiration
    SafetyTimer,
}

pub enum NtcFault {
    /// Battery too cold
    Cold,
    /// Battery too hot
    Hot,
}

pub struct FaultRegister {
    val: u8,
}

impl FaultRegister {
    const ADDR: u8 = 0x09;

    /// Watchdog timer expiration
    pub fn watchdog_expired(&self) -> bool {
        self.val & 0b1000_0000 != 0
    }

    /// VBUS overloaded (OCP), or VBUS OVP in boost mode
    pub fn boost_fault(&self) -> bool {
        self.val & 0b0100_0000 != 0
    }

    /// Charger fault
    pub fn charger_fault(&self) -> Option<ChargerFault> {
        match self.val & 0b0011_0000 >> 4 {
            0b01 => Some(ChargerFault::InputFault),
            0b10 => Some(ChargerFault::ThermalShutdown),
            0b11 => Some(ChargerFault::SafetyTimer),
            _ => None,
        }
    }

    /// Battery overvoltage
    pub fn battery_fault(&self) -> bool {
        self.val & 0b0000_1000 != 0
    }

    /// Battery temperature fault
    pub fn ntc_fault(&self) -> Option<NtcFault> {
        match self.val & 0b0000_0111 {
            0b101 => Some(NtcFault::Cold),
            0b110 => Some(NtcFault::Hot),
            _ => None,
        }
    }
}

pub use registers::InputSourceControl::{
    InputVoltageLimit::Value as InputVoltageLimit,
    InputCurrentLimit::Value as InputCurrentLimit,
};

impl<I2C, E> BQ24196<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    // pub fn read_faults(&mut self) -> Result<FaultRegister, E> {
    //     Ok(FaultRegister { val: self.read_reg(FaultRegister::ADDR)? })
    // }

    pub fn get_input_voltage_limit(&mut self) -> Result<InputVoltageLimit, E> {
        Ok(self.read_reg()?.read_as_enum(registers::InputSourceControl::InputVoltageLimit).unwrap())
    }

    pub fn set_input_voltage_limit(&mut self, lim: InputVoltageLimit) -> Result<(), E> {
        self.modify_reg(|reg| {
            reg.modify(registers::InputSourceControl::InputVoltageLimit.val(lim as _));
        })
    }

    pub fn get_input_current_limit(&mut self) -> Result<InputCurrentLimit, E> {
        Ok(self.read_reg()?.read_as_enum(registers::InputSourceControl::InputCurrentLimit).unwrap())
    }

    pub fn set_input_current_limit(&mut self, lim: InputCurrentLimit) -> Result<(), E> {
        self.modify_reg(|reg| {
            reg.modify(registers::InputSourceControl::InputCurrentLimit.val(lim as _));
        })
    }

    #[inline]
    fn read_reg<REG: I2cRegister>(&mut self) -> Result<LocalRegisterCopy::<u8, REG>, E> {
        let mut buf = [0u8];
        self.i2c.write_read(ADDR, &[REG::ADDR], &mut buf)?;
        Ok(LocalRegisterCopy::new(buf[0]))
    }

    #[inline]
    fn write_reg<REG: I2cRegister>(&mut self, reg: LocalRegisterCopy::<u8, REG>) -> Result<(), E> {
        self.i2c.write(ADDR, &[REG::ADDR, reg.get()])
    }

    #[inline]
    fn modify_reg<REG, F>(&mut self, f: F) -> Result<(), E>
    where
        REG: I2cRegister,
        F: FnOnce(&mut LocalRegisterCopy::<u8, REG>),
    {
        let mut r = self.read_reg()?;
        f(&mut r);
        self.write_reg(r)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
