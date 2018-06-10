//! I2C driver for the Bosch Sensortec BMP280 barometric pressure sensor.
//!
//! Technical specifications:
//!
//!   - <https://www.bosch-sensortec.com/bst/products/all_products/bmp280>
//!   - <https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BMP280-DS001-19.pdf>
#![deny(
    missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unused_import_braces, unused_qualifications
)]
// TODO: #![deny(missing_docs)]
extern crate byteorder;
extern crate i2cdev;

pub mod error;
pub mod reg;

pub const CHIP_ID: u8 = 0x58;

/// An interface to a BMP280 device that can be used to control the device over I2C.
#[derive(Debug)]
pub struct Bmp280<D> {
    device: D,
    t_fine: i32,

    calibration_data: CalibrationData,
}

#[derive(Debug)]
struct CalibrationData {
    dig_t1: u16,
    dig_t2: i16,
    dig_t3: i16,

    dig_p1: u16,
    dig_p2: i16,
    dig_p3: i16,
    dig_p4: i16,
    dig_p5: i16,
    dig_p6: i16,
    dig_p7: i16,
    dig_p8: i16,
    dig_p9: i16,
}

impl<D> Bmp280<D>
where
    D: i2cdev::core::I2CDevice,
{
    /// Create a new instance.
    ///
    /// The supplied chip ID will be cross-checked against the chip's reported ID.
    pub fn new(mut device: D, chip_id: u8) -> error::Result<Bmp280<D>, D> {
        let actual_chip_id = device
            .smbus_read_byte_data(reg::Register::ChipId as u8)
            .map_err(error::Error::I2C)?;
        if actual_chip_id != chip_id {
            return Err(error::Error::UnexpectedChipId(actual_chip_id));
        }

        let calibration_data = Bmp280::read_coefficients(&mut device)?;
        device
            .smbus_write_byte_data(reg::Register::Control as u8, 0x3f)
            .map_err(error::Error::I2C)?;

        Ok(Bmp280 {
            device,
            t_fine: 0,
            calibration_data,
        })
    }

    /// Get a temperature reading, in Celsius.
    pub fn read_temperature(&mut self) -> error::Result<f32, D> {
        let t_fine = self.read_temperature_fine()?;
        Ok(((t_fine * 5 + 128) >> 8) as f32 / 100.0)
    }

    fn read_temperature_fine(&mut self) -> error::Result<i32, D> {
        let adc_t = Bmp280::read_i24_le(&mut self.device, reg::Register::TempData)? >> 4;

        let v1 = (((adc_t >> 3) - (self.calibration_data.dig_t1 as i32) << 1)
            * (self.calibration_data.dig_t2 as i32)) >> 11;
        let v2 = (((((adc_t >> 4) - (self.calibration_data.dig_t1 as i32))
            * ((adc_t >> 4) - (self.calibration_data.dig_t1 as i32))) >> 12)
            * (self.calibration_data.dig_t3 as i32)) >> 14;

        Ok(v1 + v2)
    }

    /// Get a pressure reading, in Pascals.
    pub fn read_pressure(&mut self) -> error::Result<f32, D> {
        unimplemented!()
    }

    /// Get an altitude reading, in meters.
    pub fn read_altitude(&mut self, _sea_level_hpa: f32) -> error::Result<f32, D> {
        unimplemented!()
    }

    fn read_coefficients(device: &mut D) -> error::Result<CalibrationData, D> {
        Ok(CalibrationData {
            dig_t1: Bmp280::read_u16_le(device, reg::Register::DigT1)?,
            dig_t2: Bmp280::read_i16_le(device, reg::Register::DigT2)?,
            dig_t3: Bmp280::read_i16_le(device, reg::Register::DigT3)?,

            dig_p1: Bmp280::read_u16_le(device, reg::Register::DigP1)?,
            dig_p2: Bmp280::read_i16_le(device, reg::Register::DigP2)?,
            dig_p3: Bmp280::read_i16_le(device, reg::Register::DigP3)?,
            dig_p4: Bmp280::read_i16_le(device, reg::Register::DigP4)?,
            dig_p5: Bmp280::read_i16_le(device, reg::Register::DigP5)?,
            dig_p6: Bmp280::read_i16_le(device, reg::Register::DigP6)?,
            dig_p7: Bmp280::read_i16_le(device, reg::Register::DigP7)?,
            dig_p8: Bmp280::read_i16_le(device, reg::Register::DigP8)?,
            dig_p9: Bmp280::read_i16_le(device, reg::Register::DigP9)?,
        })
    }

    fn read_u16_le(device: &mut D, reg: reg::Register) -> error::Result<u16, D> {
        use byteorder::ByteOrder;

        let mut data = [0u8; 2];
        device
            .smbus_write_byte(reg as u8)
            .map_err(error::Error::I2C)?;
        device.read(&mut data).map_err(error::Error::I2C)?;
        Ok(byteorder::LittleEndian::read_u16(&data))
    }

    fn read_i16_le(device: &mut D, reg: reg::Register) -> error::Result<i16, D> {
        use byteorder::ByteOrder;

        let mut data = [0u8; 2];
        device
            .smbus_write_byte(reg as u8)
            .map_err(error::Error::I2C)?;
        device.read(&mut data).map_err(error::Error::I2C)?;
        Ok(byteorder::LittleEndian::read_i16(&data))
    }

    fn read_i24_le(device: &mut D, reg: reg::Register) -> error::Result<i32, D> {
        use byteorder::ByteOrder;

        let mut data = [0u8; 3];
        device
            .smbus_write_byte(reg as u8)
            .map_err(error::Error::I2C)?;
        device.read(&mut data).map_err(error::Error::I2C)?;

        let value = (data[0] as i32) << 16 | (data[1] as i32) << 8 | data[2] as i32;
        Ok(value)
    }
}
