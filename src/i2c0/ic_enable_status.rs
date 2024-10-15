#[doc = "Register `IC_ENABLE_STATUS` reader"]
pub type R = crate::R<IC_ENABLE_STATUS_SPEC>;
#[doc = "ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_EN_A {
    #[doc = "0: I2C disabled"]
    DISABLED = 0,
    #[doc = "1: I2C enabled"]
    ENABLED = 1,
}
impl From<IC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_EN` reader - ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).  

 Reset value: 0x0"]
pub type IC_EN_R = crate::BitReader<IC_EN_A>;
impl IC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC_EN_A {
        match self.bits {
            false => IC_EN_A::DISABLED,
            true => IC_EN_A::ENABLED,
        }
    }
    #[doc = "I2C disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IC_EN_A::DISABLED
    }
    #[doc = "I2C enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IC_EN_A::ENABLED
    }
}
#[doc = "Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:  

 (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;  

 OR,  

 (b) address and data bytes of the Slave-Receiver operation from a remote master.  

 When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.  

 Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE\\[0\\]
has been set to 0, then this bit will also be set to 1.  

 When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.  

 Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLV_DISABLED_WHILE_BUSY_A {
    #[doc = "0: Slave is disabled when it is idle"]
    INACTIVE = 0,
    #[doc = "1: Slave is disabled when it is active"]
    ACTIVE = 1,
}
impl From<SLV_DISABLED_WHILE_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SLV_DISABLED_WHILE_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV_DISABLED_WHILE_BUSY` reader - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:  

 (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;  

 OR,  

 (b) address and data bytes of the Slave-Receiver operation from a remote master.  

 When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.  

 Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE\\[0\\]
has been set to 0, then this bit will also be set to 1.  

 When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.  

 Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.  

 Reset value: 0x0"]
pub type SLV_DISABLED_WHILE_BUSY_R = crate::BitReader<SLV_DISABLED_WHILE_BUSY_A>;
impl SLV_DISABLED_WHILE_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLV_DISABLED_WHILE_BUSY_A {
        match self.bits {
            false => SLV_DISABLED_WHILE_BUSY_A::INACTIVE,
            true => SLV_DISABLED_WHILE_BUSY_A::ACTIVE,
        }
    }
    #[doc = "Slave is disabled when it is idle"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SLV_DISABLED_WHILE_BUSY_A::INACTIVE
    }
    #[doc = "Slave is disabled when it is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLV_DISABLED_WHILE_BUSY_A::ACTIVE
    }
}
#[doc = "Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.  

 Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE\\[0\\]
has been set to 0, then this bit is also set to 1.  

 When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.  

 Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLV_RX_DATA_LOST_A {
    #[doc = "0: Slave RX Data is not lost"]
    INACTIVE = 0,
    #[doc = "1: Slave RX Data is lost"]
    ACTIVE = 1,
}
impl From<SLV_RX_DATA_LOST_A> for bool {
    #[inline(always)]
    fn from(variant: SLV_RX_DATA_LOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV_RX_DATA_LOST` reader - Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.  

 Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE\\[0\\]
has been set to 0, then this bit is also set to 1.  

 When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.  

 Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.  

 Reset value: 0x0"]
pub type SLV_RX_DATA_LOST_R = crate::BitReader<SLV_RX_DATA_LOST_A>;
impl SLV_RX_DATA_LOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLV_RX_DATA_LOST_A {
        match self.bits {
            false => SLV_RX_DATA_LOST_A::INACTIVE,
            true => SLV_RX_DATA_LOST_A::ACTIVE,
        }
    }
    #[doc = "Slave RX Data is not lost"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SLV_RX_DATA_LOST_A::INACTIVE
    }
    #[doc = "Slave RX Data is lost"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLV_RX_DATA_LOST_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1).  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn ic_en(&self) -> IC_EN_R {
        IC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while:  

 (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master;  

 OR,  

 (b) address and data bytes of the Slave-Receiver operation from a remote master.  

 When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect.  

 Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE\\[0\\]
has been set to 0, then this bit will also be set to 1.  

 When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle.  

 Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn slv_disabled_while_busy(&self) -> SLV_DISABLED_WHILE_BUSY_R {
        SLV_DISABLED_WHILE_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK.  

 Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE\\[0\\]
has been set to 0, then this bit is also set to 1.  

 When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer.  

 Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn slv_rx_data_lost(&self) -> SLV_RX_DATA_LOST_R {
        SLV_RX_DATA_LOST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "I2C Enable Status Register  

 The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE\\[0\\]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled.  

 If IC_ENABLE\\[0\\]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1.  

 If IC_ENABLE\\[0\\]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'.  

 Note: When IC_ENABLE\\[0\\]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities.  

You can [`read`](crate::Reg::read) this register and get [`ic_enable_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_ENABLE_STATUS_SPEC;
impl crate::RegisterSpec for IC_ENABLE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_enable_status::R`](R) reader structure"]
impl crate::Readable for IC_ENABLE_STATUS_SPEC {}
#[doc = "`reset()` method sets IC_ENABLE_STATUS to value 0"]
impl crate::Resettable for IC_ENABLE_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
