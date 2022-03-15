#![no_std]
#![doc = include_str!("../README.md")]

mod registers;

use embedded_hal::blocking::i2c::{Write, WriteRead};
use registers::I2cRegister;
use tock_registers::LocalRegisterCopy;

const DEVICE_ADDR: u8 = 0x6B;

pub use registers::{
    REG00::{IINLIM::Value as InputCurrentLimit, VINDPM::Value as InputVoltageLimit},
    REG08::{CHRG_STAT::Value as ChargerStatus, VBUS_STAT::Value as VBusStatus},
};

#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    Bus(E),
    /// Device P/N did not match during initialization
    UnknownDevice(u8),
}

pub struct BQ24196<I2C> {
    i2c: I2C,
}

impl<I2C, E> BQ24196<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C) -> Result<Self, Error<E>> {
        let mut dev = Self { i2c };

        let pn = dev.read_reg()?.read(registers::REG0A::PN);
        if pn != registers::REG0A::PN::Value::DEFAULT_PN as _ {
            return Err(Error::UnknownDevice(pn));
        }

        Ok(dev)
    }

    pub fn is_input_high_z(&mut self) -> Result<bool, Error<E>> {
        Ok(self.read_reg()?.is_set(registers::REG00::EN_HIZ))
    }

    pub fn set_input_high_z(&mut self, en: bool) -> Result<(), Error<E>> {
        self.modify_reg(|reg| {
            reg.modify(registers::REG00::EN_HIZ.val(en as _));
        })
    }

    pub fn get_input_voltage_limit(&mut self) -> Result<InputVoltageLimit, Error<E>> {
        Ok(self
            .read_reg()?
            .read_as_enum(registers::REG00::VINDPM)
            .unwrap())
    }

    pub fn set_input_voltage_limit(&mut self, lim: InputVoltageLimit) -> Result<(), Error<E>> {
        self.modify_reg(|reg| {
            reg.modify(registers::REG00::VINDPM.val(lim as _));
        })
    }

    pub fn get_input_current_limit(&mut self) -> Result<InputCurrentLimit, Error<E>> {
        Ok(self
            .read_reg()?
            .read_as_enum(registers::REG00::IINLIM)
            .unwrap())
    }

    pub fn set_input_current_limit(&mut self, lim: InputCurrentLimit) -> Result<(), Error<E>> {
        self.modify_reg(|reg| {
            reg.modify(registers::REG00::IINLIM.val(lim as _));
        })
    }

    pub fn vbus_status(&mut self) -> Result<VBusStatus, Error<E>> {
        Ok(self
            .read_reg()?
            .read_as_enum(registers::REG08::VBUS_STAT)
            .unwrap())
    }

    pub fn charger_status(&mut self) -> Result<ChargerStatus, Error<E>> {
        Ok(self
            .read_reg()?
            .read_as_enum(registers::REG08::CHRG_STAT)
            .unwrap())
    }

    /// True if any of the faults are active
    pub fn has_fault(&mut self) -> Result<bool, Error<E>> {
        Ok(self.read_reg::<registers::REG09::Register>()?.get() != 0)
    }

    #[inline]
    fn read_reg<REG: I2cRegister>(&mut self) -> Result<LocalRegisterCopy<u8, REG>, Error<E>> {
        let mut buf = [0u8];
        self.i2c
            .write_read(DEVICE_ADDR, &[REG::ADDR], &mut buf)
            .map_err(|e| Error::Bus(e))?;
        Ok(LocalRegisterCopy::new(buf[0]))
    }

    #[inline]
    fn write_reg<REG: I2cRegister>(
        &mut self,
        reg: LocalRegisterCopy<u8, REG>,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(DEVICE_ADDR, &[REG::ADDR, reg.get()])
            .map_err(|e| Error::Bus(e))
    }

    #[inline]
    fn modify_reg<REG, F>(&mut self, f: F) -> Result<(), Error<E>>
    where
        REG: I2cRegister,
        F: FnOnce(&mut LocalRegisterCopy<u8, REG>),
    {
        let mut r = self.read_reg()?;
        f(&mut r);
        self.write_reg(r)?;
        Ok(())
    }
}
