#[doc = "Register `IC_CON` reader"]
pub type R = crate::R<IC_CON_SPEC>;
#[doc = "Register `IC_CON` writer"]
pub type W = crate::W<IC_CON_SPEC>;
#[doc = "This bit controls whether the DW_apb_i2c master is enabled.  

 NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_MODE_A {
    #[doc = "0: Master mode is disabled"]
    DISABLED = 0,
    #[doc = "1: Master mode is enabled"]
    ENABLED = 1,
}
impl From<MASTER_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER_MODE` reader - This bit controls whether the DW_apb_i2c master is enabled.  

 NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
pub type MASTER_MODE_R = crate::BitReader<MASTER_MODE_A>;
impl MASTER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASTER_MODE_A {
        match self.bits {
            false => MASTER_MODE_A::DISABLED,
            true => MASTER_MODE_A::ENABLED,
        }
    }
    #[doc = "Master mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASTER_MODE_A::DISABLED
    }
    #[doc = "Master mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASTER_MODE_A::ENABLED
    }
}
#[doc = "Field `MASTER_MODE` writer - This bit controls whether the DW_apb_i2c master is enabled.  

 NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
pub type MASTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG, MASTER_MODE_A>;
impl<'a, REG> MASTER_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER_MODE_A::DISABLED)
    }
    #[doc = "Master mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER_MODE_A::ENABLED)
    }
}
#[doc = "These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.  

 This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.  

 1: standard mode (100 kbit/s)  

 2: fast mode (&lt;=400 kbit/s) or fast mode plus (&lt;=1000Kbit/s)  

 3: high speed mode (3.4 Mbit/s)  

 Note: This field is not applicable when IC_ULTRA_FAST_MODE=1  

Value on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "1: Standard Speed mode of operation"]
    STANDARD = 1,
    #[doc = "2: Fast or Fast Plus mode of operation"]
    FAST = 2,
    #[doc = "3: High Speed mode of operation"]
    HIGH = 3,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEED_A {
    type Ux = u8;
}
impl crate::IsEnum for SPEED_A {}
#[doc = "Field `SPEED` reader - These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.  

 This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.  

 1: standard mode (100 kbit/s)  

 2: fast mode (&lt;=400 kbit/s) or fast mode plus (&lt;=1000Kbit/s)  

 3: high speed mode (3.4 Mbit/s)  

 Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
pub type SPEED_R = crate::FieldReader<SPEED_A>;
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            1 => Some(SPEED_A::STANDARD),
            2 => Some(SPEED_A::FAST),
            3 => Some(SPEED_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Standard Speed mode of operation"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SPEED_A::STANDARD
    }
    #[doc = "Fast or Fast Plus mode of operation"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SPEED_A::FAST
    }
    #[doc = "High Speed mode of operation"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEED_A::HIGH
    }
}
#[doc = "Field `SPEED` writer - These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.  

 This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.  

 1: standard mode (100 kbit/s)  

 2: fast mode (&lt;=400 kbit/s) or fast mode plus (&lt;=1000Kbit/s)  

 3: high speed mode (3.4 Mbit/s)  

 Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPEED_A>;
impl<'a, REG> SPEED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard Speed mode of operation"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SPEED_A::STANDARD)
    }
    #[doc = "Fast or Fast Plus mode of operation"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(SPEED_A::FAST)
    }
    #[doc = "High Speed mode of operation"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SPEED_A::HIGH)
    }
}
#[doc = "When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_10BITADDR_SLAVE_A {
    #[doc = "0: Slave 7Bit addressing"]
    ADDR_7BITS = 0,
    #[doc = "1: Slave 10Bit addressing"]
    ADDR_10BITS = 1,
}
impl From<IC_10BITADDR_SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: IC_10BITADDR_SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_10BITADDR_SLAVE` reader - When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
pub type IC_10BITADDR_SLAVE_R = crate::BitReader<IC_10BITADDR_SLAVE_A>;
impl IC_10BITADDR_SLAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC_10BITADDR_SLAVE_A {
        match self.bits {
            false => IC_10BITADDR_SLAVE_A::ADDR_7BITS,
            true => IC_10BITADDR_SLAVE_A::ADDR_10BITS,
        }
    }
    #[doc = "Slave 7Bit addressing"]
    #[inline(always)]
    pub fn is_addr_7bits(&self) -> bool {
        *self == IC_10BITADDR_SLAVE_A::ADDR_7BITS
    }
    #[doc = "Slave 10Bit addressing"]
    #[inline(always)]
    pub fn is_addr_10bits(&self) -> bool {
        *self == IC_10BITADDR_SLAVE_A::ADDR_10BITS
    }
}
#[doc = "Field `IC_10BITADDR_SLAVE` writer - When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
pub type IC_10BITADDR_SLAVE_W<'a, REG> = crate::BitWriter<'a, REG, IC_10BITADDR_SLAVE_A>;
impl<'a, REG> IC_10BITADDR_SLAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave 7Bit addressing"]
    #[inline(always)]
    pub fn addr_7bits(self) -> &'a mut crate::W<REG> {
        self.variant(IC_10BITADDR_SLAVE_A::ADDR_7BITS)
    }
    #[doc = "Slave 10Bit addressing"]
    #[inline(always)]
    pub fn addr_10bits(self) -> &'a mut crate::W<REG> {
        self.variant(IC_10BITADDR_SLAVE_A::ADDR_10BITS)
    }
}
#[doc = "Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_10BITADDR_MASTER_A {
    #[doc = "0: Master 7Bit addressing mode"]
    ADDR_7BITS = 0,
    #[doc = "1: Master 10Bit addressing mode"]
    ADDR_10BITS = 1,
}
impl From<IC_10BITADDR_MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: IC_10BITADDR_MASTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_10BITADDR_MASTER` reader - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
pub type IC_10BITADDR_MASTER_R = crate::BitReader<IC_10BITADDR_MASTER_A>;
impl IC_10BITADDR_MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC_10BITADDR_MASTER_A {
        match self.bits {
            false => IC_10BITADDR_MASTER_A::ADDR_7BITS,
            true => IC_10BITADDR_MASTER_A::ADDR_10BITS,
        }
    }
    #[doc = "Master 7Bit addressing mode"]
    #[inline(always)]
    pub fn is_addr_7bits(&self) -> bool {
        *self == IC_10BITADDR_MASTER_A::ADDR_7BITS
    }
    #[doc = "Master 10Bit addressing mode"]
    #[inline(always)]
    pub fn is_addr_10bits(&self) -> bool {
        *self == IC_10BITADDR_MASTER_A::ADDR_10BITS
    }
}
#[doc = "Field `IC_10BITADDR_MASTER` writer - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
pub type IC_10BITADDR_MASTER_W<'a, REG> = crate::BitWriter<'a, REG, IC_10BITADDR_MASTER_A>;
impl<'a, REG> IC_10BITADDR_MASTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master 7Bit addressing mode"]
    #[inline(always)]
    pub fn addr_7bits(self) -> &'a mut crate::W<REG> {
        self.variant(IC_10BITADDR_MASTER_A::ADDR_7BITS)
    }
    #[doc = "Master 10Bit addressing mode"]
    #[inline(always)]
    pub fn addr_10bits(self) -> &'a mut crate::W<REG> {
        self.variant(IC_10BITADDR_MASTER_A::ADDR_10BITS)
    }
}
#[doc = "Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.  

 Reset value: ENABLED  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_RESTART_EN_A {
    #[doc = "0: Master restart disabled"]
    DISABLED = 0,
    #[doc = "1: Master restart enabled"]
    ENABLED = 1,
}
impl From<IC_RESTART_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IC_RESTART_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_RESTART_EN` reader - Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.  

 Reset value: ENABLED"]
pub type IC_RESTART_EN_R = crate::BitReader<IC_RESTART_EN_A>;
impl IC_RESTART_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC_RESTART_EN_A {
        match self.bits {
            false => IC_RESTART_EN_A::DISABLED,
            true => IC_RESTART_EN_A::ENABLED,
        }
    }
    #[doc = "Master restart disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IC_RESTART_EN_A::DISABLED
    }
    #[doc = "Master restart enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IC_RESTART_EN_A::ENABLED
    }
}
#[doc = "Field `IC_RESTART_EN` writer - Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.  

 Reset value: ENABLED"]
pub type IC_RESTART_EN_W<'a, REG> = crate::BitWriter<'a, REG, IC_RESTART_EN_A>;
impl<'a, REG> IC_RESTART_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master restart disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IC_RESTART_EN_A::DISABLED)
    }
    #[doc = "Master restart enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IC_RESTART_EN_A::ENABLED)
    }
}
#[doc = "This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.  

 If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.  

 NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_SLAVE_DISABLE_A {
    #[doc = "0: Slave mode is enabled"]
    SLAVE_ENABLED = 0,
    #[doc = "1: Slave mode is disabled"]
    SLAVE_DISABLED = 1,
}
impl From<IC_SLAVE_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: IC_SLAVE_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_SLAVE_DISABLE` reader - This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.  

 If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.  

 NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
pub type IC_SLAVE_DISABLE_R = crate::BitReader<IC_SLAVE_DISABLE_A>;
impl IC_SLAVE_DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC_SLAVE_DISABLE_A {
        match self.bits {
            false => IC_SLAVE_DISABLE_A::SLAVE_ENABLED,
            true => IC_SLAVE_DISABLE_A::SLAVE_DISABLED,
        }
    }
    #[doc = "Slave mode is enabled"]
    #[inline(always)]
    pub fn is_slave_enabled(&self) -> bool {
        *self == IC_SLAVE_DISABLE_A::SLAVE_ENABLED
    }
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn is_slave_disabled(&self) -> bool {
        *self == IC_SLAVE_DISABLE_A::SLAVE_DISABLED
    }
}
#[doc = "Field `IC_SLAVE_DISABLE` writer - This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.  

 If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.  

 NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
pub type IC_SLAVE_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG, IC_SLAVE_DISABLE_A>;
impl<'a, REG> IC_SLAVE_DISABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode is enabled"]
    #[inline(always)]
    pub fn slave_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IC_SLAVE_DISABLE_A::SLAVE_ENABLED)
    }
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn slave_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IC_SLAVE_DISABLE_A::SLAVE_DISABLED)
    }
}
#[doc = "In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0  

 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_DET_IFADDRESSED_A {
    #[doc = "0: slave issues STOP_DET intr always"]
    DISABLED = 0,
    #[doc = "1: slave issues STOP_DET intr only if addressed"]
    ENABLED = 1,
}
impl From<STOP_DET_IFADDRESSED_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_DET_IFADDRESSED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_DET_IFADDRESSED` reader - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0  

 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
pub type STOP_DET_IFADDRESSED_R = crate::BitReader<STOP_DET_IFADDRESSED_A>;
impl STOP_DET_IFADDRESSED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_DET_IFADDRESSED_A {
        match self.bits {
            false => STOP_DET_IFADDRESSED_A::DISABLED,
            true => STOP_DET_IFADDRESSED_A::ENABLED,
        }
    }
    #[doc = "slave issues STOP_DET intr always"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOP_DET_IFADDRESSED_A::DISABLED
    }
    #[doc = "slave issues STOP_DET intr only if addressed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOP_DET_IFADDRESSED_A::ENABLED
    }
}
#[doc = "Field `STOP_DET_IFADDRESSED` writer - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0  

 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
pub type STOP_DET_IFADDRESSED_W<'a, REG> = crate::BitWriter<'a, REG, STOP_DET_IFADDRESSED_A>;
impl<'a, REG> STOP_DET_IFADDRESSED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "slave issues STOP_DET intr always"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_DET_IFADDRESSED_A::DISABLED)
    }
    #[doc = "slave issues STOP_DET intr only if addressed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_DET_IFADDRESSED_A::ENABLED)
    }
}
#[doc = "This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.  

 Reset value: 0x0.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EMPTY_CTRL_A {
    #[doc = "0: Default behaviour of TX_EMPTY interrupt"]
    DISABLED = 0,
    #[doc = "1: Controlled generation of TX_EMPTY interrupt"]
    ENABLED = 1,
}
impl From<TX_EMPTY_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EMPTY_CTRL` reader - This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.  

 Reset value: 0x0."]
pub type TX_EMPTY_CTRL_R = crate::BitReader<TX_EMPTY_CTRL_A>;
impl TX_EMPTY_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_EMPTY_CTRL_A {
        match self.bits {
            false => TX_EMPTY_CTRL_A::DISABLED,
            true => TX_EMPTY_CTRL_A::ENABLED,
        }
    }
    #[doc = "Default behaviour of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_EMPTY_CTRL_A::DISABLED
    }
    #[doc = "Controlled generation of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_EMPTY_CTRL_A::ENABLED
    }
}
#[doc = "Field `TX_EMPTY_CTRL` writer - This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.  

 Reset value: 0x0."]
pub type TX_EMPTY_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, TX_EMPTY_CTRL_A>;
impl<'a, REG> TX_EMPTY_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_CTRL_A::DISABLED)
    }
    #[doc = "Controlled generation of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TX_EMPTY_CTRL_A::ENABLED)
    }
}
#[doc = "This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.  

 Reset value: 0x0.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_FULL_HLD_CTRL_A {
    #[doc = "0: Overflow when RX_FIFO is full"]
    DISABLED = 0,
    #[doc = "1: Hold bus when RX_FIFO is full"]
    ENABLED = 1,
}
impl From<RX_FIFO_FULL_HLD_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_FULL_HLD_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_FULL_HLD_CTRL` reader - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.  

 Reset value: 0x0."]
pub type RX_FIFO_FULL_HLD_CTRL_R = crate::BitReader<RX_FIFO_FULL_HLD_CTRL_A>;
impl RX_FIFO_FULL_HLD_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_FIFO_FULL_HLD_CTRL_A {
        match self.bits {
            false => RX_FIFO_FULL_HLD_CTRL_A::DISABLED,
            true => RX_FIFO_FULL_HLD_CTRL_A::ENABLED,
        }
    }
    #[doc = "Overflow when RX_FIFO is full"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_FIFO_FULL_HLD_CTRL_A::DISABLED
    }
    #[doc = "Hold bus when RX_FIFO is full"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_FIFO_FULL_HLD_CTRL_A::ENABLED
    }
}
#[doc = "Field `RX_FIFO_FULL_HLD_CTRL` writer - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.  

 Reset value: 0x0."]
pub type RX_FIFO_FULL_HLD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, RX_FIFO_FULL_HLD_CTRL_A>;
impl<'a, REG> RX_FIFO_FULL_HLD_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow when RX_FIFO is full"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_FULL_HLD_CTRL_A::DISABLED)
    }
    #[doc = "Hold bus when RX_FIFO is full"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FIFO_FULL_HLD_CTRL_A::ENABLED)
    }
}
#[doc = "Field `STOP_DET_IF_MASTER_ACTIVE` reader - Master issues the STOP_DET interrupt irrespective of whether master is active or not"]
pub type STOP_DET_IF_MASTER_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled.  

 NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.  

 This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.  

 1: standard mode (100 kbit/s)  

 2: fast mode (&lt;=400 kbit/s) or fast mode plus (&lt;=1000Kbit/s)  

 3: high speed mode (3.4 Mbit/s)  

 Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&self) -> IC_10BITADDR_SLAVE_R {
        IC_10BITADDR_SLAVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_master(&self) -> IC_10BITADDR_MASTER_R {
        IC_10BITADDR_MASTER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.  

 Reset value: ENABLED"]
    #[inline(always)]
    pub fn ic_restart_en(&self) -> IC_RESTART_EN_R {
        IC_RESTART_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.  

 If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.  

 NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
    #[inline(always)]
    pub fn ic_slave_disable(&self) -> IC_SLAVE_DISABLE_R {
        IC_SLAVE_DISABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0  

 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> STOP_DET_IFADDRESSED_R {
        STOP_DET_IFADDRESSED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.  

 Reset value: 0x0."]
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> TX_EMPTY_CTRL_R {
        TX_EMPTY_CTRL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.  

 Reset value: 0x0."]
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> RX_FIFO_FULL_HLD_CTRL_R {
        RX_FIFO_FULL_HLD_CTRL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master issues the STOP_DET interrupt irrespective of whether master is active or not"]
    #[inline(always)]
    pub fn stop_det_if_master_active(&self) -> STOP_DET_IF_MASTER_ACTIVE_R {
        STOP_DET_IF_MASTER_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled.  

 NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    #[inline(always)]
    #[must_use]
    pub fn master_mode(&mut self) -> MASTER_MODE_W<IC_CON_SPEC> {
        MASTER_MODE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.  

 This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.  

 1: standard mode (100 kbit/s)  

 2: fast mode (&lt;=400 kbit/s) or fast mode plus (&lt;=1000Kbit/s)  

 3: high speed mode (3.4 Mbit/s)  

 Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<IC_CON_SPEC> {
        SPEED_W::new(self, 1)
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
    #[inline(always)]
    #[must_use]
    pub fn ic_10bitaddr_slave(&mut self) -> IC_10BITADDR_SLAVE_W<IC_CON_SPEC> {
        IC_10BITADDR_SLAVE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
    #[inline(always)]
    #[must_use]
    pub fn ic_10bitaddr_master(&mut self) -> IC_10BITADDR_MASTER_W<IC_CON_SPEC> {
        IC_10BITADDR_MASTER_W::new(self, 4)
    }
    #[doc = "Bit 5 - Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.  

 Reset value: ENABLED"]
    #[inline(always)]
    #[must_use]
    pub fn ic_restart_en(&mut self) -> IC_RESTART_EN_W<IC_CON_SPEC> {
        IC_RESTART_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.  

 If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.  

 NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
    #[inline(always)]
    #[must_use]
    pub fn ic_slave_disable(&mut self) -> IC_SLAVE_DISABLE_W<IC_CON_SPEC> {
        IC_SLAVE_DISABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0  

 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_det_ifaddressed(&mut self) -> STOP_DET_IFADDRESSED_W<IC_CON_SPEC> {
        STOP_DET_IFADDRESSED_W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.  

 Reset value: 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty_ctrl(&mut self) -> TX_EMPTY_CTRL_W<IC_CON_SPEC> {
        TX_EMPTY_CTRL_W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.  

 Reset value: 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full_hld_ctrl(&mut self) -> RX_FIFO_FULL_HLD_CTRL_W<IC_CON_SPEC> {
        RX_FIFO_FULL_HLD_CTRL_W::new(self, 9)
    }
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only.  

You can [`read`](crate::Reg::read) this register and get [`ic_con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_CON_SPEC;
impl crate::RegisterSpec for IC_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_con::R`](R) reader structure"]
impl crate::Readable for IC_CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_con::W`](W) writer structure"]
impl crate::Writable for IC_CON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_CON to value 0x65"]
impl crate::Resettable for IC_CON_SPEC {
    const RESET_VALUE: u32 = 0x65;
}
