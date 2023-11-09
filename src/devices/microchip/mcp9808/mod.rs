use embedded_registers::RegisterInterface;

pub mod address;
pub mod configuration;
pub mod device_id_revision;
pub mod manufacturer_id;
pub mod resolution;
pub mod temperature;

crate::simple_device::device!(MCP9808);
crate::simple_device::i2c!(MCP9808, self::address::Address, init_wanted);

/// All possible errors in this crate
#[derive(Debug, defmt::Format)]
pub enum InitError<BusError> {
    /// Bus error
    Bus(BusError),
    /// Invalid Device Id was encountered
    InvalidDeviceId,
    /// Invalid Manufacturer Id was encountered
    InvalidManufacturerId,
}

#[maybe_async_cfg::maybe(sync(not(feature = "async")), async(feature = "async"), keep_self)]
impl<I> MCP9808<I>
where
    I: RegisterInterface,
{
    /// Initialize the sensor by verifying its device id and manufacturer id.
    /// Not mandatory, but recommended.
    pub async fn init(&mut self) -> Result<(), InitError<I::Error>> {
        let device_id = self.read_device_id_revision().await.map_err(InitError::Bus)?;
        if device_id.read_device_id() != self::device_id_revision::DEVICE_ID_VALID {
            return Err(InitError::InvalidDeviceId);
        }

        let manufacturer_id = self.read_manufacturer_id().await.map_err(InitError::Bus)?;
        if manufacturer_id.read_manufacturer_id() != self::manufacturer_id::MANUFACTURER_ID_VALID {
            return Err(InitError::InvalidManufacturerId);
        }

        Ok(())
    }
}
