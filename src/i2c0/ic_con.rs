#[doc = "Reader of register IC_CON"]
pub type R = crate::R<u32, super::IC_CON>;
#[doc = "Writer for register IC_CON"]
pub type W = crate::W<u32, super::IC_CON>;
#[doc = "Register IC_CON `reset()`'s with value 0x65"]
impl crate::ResetValue for super::IC_CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x65
    }
}
#[doc = "Reader of field `STOP_DET_IF_MASTER_ACTIVE`"]
pub type STOP_DET_IF_MASTER_ACTIVE_R = crate::R<bool, bool>;
#[doc = "This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.\\n\\n Reset value: 0x0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RX_FIFO_FULL_HLD_CTRL`"]
pub type RX_FIFO_FULL_HLD_CTRL_R = crate::R<bool, RX_FIFO_FULL_HLD_CTRL_A>;
impl RX_FIFO_FULL_HLD_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_FULL_HLD_CTRL_A {
        match self.bits {
            false => RX_FIFO_FULL_HLD_CTRL_A::DISABLED,
            true => RX_FIFO_FULL_HLD_CTRL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_FIFO_FULL_HLD_CTRL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_FIFO_FULL_HLD_CTRL_A::ENABLED
    }
}
#[doc = "Write proxy for field `RX_FIFO_FULL_HLD_CTRL`"]
pub struct RX_FIFO_FULL_HLD_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_FULL_HLD_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_FULL_HLD_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overflow when RX_FIFO is full"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_FIFO_FULL_HLD_CTRL_A::DISABLED)
    }
    #[doc = "Hold bus when RX_FIFO is full"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_FIFO_FULL_HLD_CTRL_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `TX_EMPTY_CTRL`"]
pub type TX_EMPTY_CTRL_R = crate::R<bool, TX_EMPTY_CTRL_A>;
impl TX_EMPTY_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_CTRL_A {
        match self.bits {
            false => TX_EMPTY_CTRL_A::DISABLED,
            true => TX_EMPTY_CTRL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TX_EMPTY_CTRL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TX_EMPTY_CTRL_A::ENABLED
    }
}
#[doc = "Write proxy for field `TX_EMPTY_CTRL`"]
pub struct TX_EMPTY_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EMPTY_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_EMPTY_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Default behaviour of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_EMPTY_CTRL_A::DISABLED)
    }
    #[doc = "Controlled generation of TX_EMPTY interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_EMPTY_CTRL_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0\\n\\n NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `STOP_DET_IFADDRESSED`"]
pub type STOP_DET_IFADDRESSED_R = crate::R<bool, STOP_DET_IFADDRESSED_A>;
impl STOP_DET_IFADDRESSED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_DET_IFADDRESSED_A {
        match self.bits {
            false => STOP_DET_IFADDRESSED_A::DISABLED,
            true => STOP_DET_IFADDRESSED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOP_DET_IFADDRESSED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOP_DET_IFADDRESSED_A::ENABLED
    }
}
#[doc = "Write proxy for field `STOP_DET_IFADDRESSED`"]
pub struct STOP_DET_IFADDRESSED_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_DET_IFADDRESSED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_DET_IFADDRESSED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "slave issues STOP_DET intr always"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOP_DET_IFADDRESSED_A::DISABLED)
    }
    #[doc = "slave issues STOP_DET intr only if addressed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOP_DET_IFADDRESSED_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.\\n\\n If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.\\n\\n NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `IC_SLAVE_DISABLE`"]
pub type IC_SLAVE_DISABLE_R = crate::R<bool, IC_SLAVE_DISABLE_A>;
impl IC_SLAVE_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_SLAVE_DISABLE_A {
        match self.bits {
            false => IC_SLAVE_DISABLE_A::SLAVE_ENABLED,
            true => IC_SLAVE_DISABLE_A::SLAVE_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_ENABLED`"]
    #[inline(always)]
    pub fn is_slave_enabled(&self) -> bool {
        *self == IC_SLAVE_DISABLE_A::SLAVE_ENABLED
    }
    #[doc = "Checks if the value of the field is `SLAVE_DISABLED`"]
    #[inline(always)]
    pub fn is_slave_disabled(&self) -> bool {
        *self == IC_SLAVE_DISABLE_A::SLAVE_DISABLED
    }
}
#[doc = "Write proxy for field `IC_SLAVE_DISABLE`"]
pub struct IC_SLAVE_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SLAVE_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_SLAVE_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave mode is enabled"]
    #[inline(always)]
    pub fn slave_enabled(self) -> &'a mut W {
        self.variant(IC_SLAVE_DISABLE_A::SLAVE_ENABLED)
    }
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn slave_disabled(self) -> &'a mut W {
        self.variant(IC_SLAVE_DISABLE_A::SLAVE_DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: ENABLED\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `IC_RESTART_EN`"]
pub type IC_RESTART_EN_R = crate::R<bool, IC_RESTART_EN_A>;
impl IC_RESTART_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_RESTART_EN_A {
        match self.bits {
            false => IC_RESTART_EN_A::DISABLED,
            true => IC_RESTART_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IC_RESTART_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IC_RESTART_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `IC_RESTART_EN`"]
pub struct IC_RESTART_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_RESTART_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_RESTART_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master restart disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IC_RESTART_EN_A::DISABLED)
    }
    #[doc = "Master restart enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IC_RESTART_EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `IC_10BITADDR_MASTER`"]
pub type IC_10BITADDR_MASTER_R = crate::R<bool, IC_10BITADDR_MASTER_A>;
impl IC_10BITADDR_MASTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_10BITADDR_MASTER_A {
        match self.bits {
            false => IC_10BITADDR_MASTER_A::ADDR_7BITS,
            true => IC_10BITADDR_MASTER_A::ADDR_10BITS,
        }
    }
    #[doc = "Checks if the value of the field is `ADDR_7BITS`"]
    #[inline(always)]
    pub fn is_addr_7bits(&self) -> bool {
        *self == IC_10BITADDR_MASTER_A::ADDR_7BITS
    }
    #[doc = "Checks if the value of the field is `ADDR_10BITS`"]
    #[inline(always)]
    pub fn is_addr_10bits(&self) -> bool {
        *self == IC_10BITADDR_MASTER_A::ADDR_10BITS
    }
}
#[doc = "Write proxy for field `IC_10BITADDR_MASTER`"]
pub struct IC_10BITADDR_MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_10BITADDR_MASTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_10BITADDR_MASTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master 7Bit addressing mode"]
    #[inline(always)]
    pub fn addr_7bits(self) -> &'a mut W {
        self.variant(IC_10BITADDR_MASTER_A::ADDR_7BITS)
    }
    #[doc = "Master 10Bit addressing mode"]
    #[inline(always)]
    pub fn addr_10bits(self) -> &'a mut W {
        self.variant(IC_10BITADDR_MASTER_A::ADDR_10BITS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `IC_10BITADDR_SLAVE`"]
pub type IC_10BITADDR_SLAVE_R = crate::R<bool, IC_10BITADDR_SLAVE_A>;
impl IC_10BITADDR_SLAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_10BITADDR_SLAVE_A {
        match self.bits {
            false => IC_10BITADDR_SLAVE_A::ADDR_7BITS,
            true => IC_10BITADDR_SLAVE_A::ADDR_10BITS,
        }
    }
    #[doc = "Checks if the value of the field is `ADDR_7BITS`"]
    #[inline(always)]
    pub fn is_addr_7bits(&self) -> bool {
        *self == IC_10BITADDR_SLAVE_A::ADDR_7BITS
    }
    #[doc = "Checks if the value of the field is `ADDR_10BITS`"]
    #[inline(always)]
    pub fn is_addr_10bits(&self) -> bool {
        *self == IC_10BITADDR_SLAVE_A::ADDR_10BITS
    }
}
#[doc = "Write proxy for field `IC_10BITADDR_SLAVE`"]
pub struct IC_10BITADDR_SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_10BITADDR_SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_10BITADDR_SLAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave 7Bit addressing"]
    #[inline(always)]
    pub fn addr_7bits(self) -> &'a mut W {
        self.variant(IC_10BITADDR_SLAVE_A::ADDR_7BITS)
    }
    #[doc = "Slave 10Bit addressing"]
    #[inline(always)]
    pub fn addr_10bits(self) -> &'a mut W {
        self.variant(IC_10BITADDR_SLAVE_A::ADDR_10BITS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.\\n\\n This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.\\n\\n 1: standard mode (100 kbit/s)\\n\\n 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)\\n\\n 3: high speed mode (3.4 Mbit/s)\\n\\n Note: This field is not applicable when IC_ULTRA_FAST_MODE=1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SPEED_A::STANDARD),
            2 => Val(SPEED_A::FAST),
            3 => Val(SPEED_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SPEED_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SPEED_A::FAST
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEED_A::HIGH
    }
}
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard Speed mode of operation"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SPEED_A::STANDARD)
    }
    #[doc = "Fast or Fast Plus mode of operation"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SPEED_A::FAST)
    }
    #[doc = "High Speed mode of operation"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEED_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "This bit controls whether the DW_apb_i2c master is enabled.\\n\\n NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `MASTER_MODE`"]
pub type MASTER_MODE_R = crate::R<bool, MASTER_MODE_A>;
impl MASTER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_MODE_A {
        match self.bits {
            false => MASTER_MODE_A::DISABLED,
            true => MASTER_MODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASTER_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASTER_MODE_A::ENABLED
    }
}
#[doc = "Write proxy for field `MASTER_MODE`"]
pub struct MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MASTER_MODE_A::DISABLED)
    }
    #[doc = "Master mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MASTER_MODE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - Master issues the STOP_DET interrupt irrespective of whether master is active or not"]
    #[inline(always)]
    pub fn stop_det_if_master_active(&self) -> STOP_DET_IF_MASTER_ACTIVE_R {
        STOP_DET_IF_MASTER_ACTIVE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.\\n\\n Reset value: 0x0."]
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> RX_FIFO_FULL_HLD_CTRL_R {
        RX_FIFO_FULL_HLD_CTRL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0."]
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> TX_EMPTY_CTRL_R {
        TX_EMPTY_CTRL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0\\n\\n NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> STOP_DET_IFADDRESSED_R {
        STOP_DET_IFADDRESSED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.\\n\\n If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.\\n\\n NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
    #[inline(always)]
    pub fn ic_slave_disable(&self) -> IC_SLAVE_DISABLE_R {
        IC_SLAVE_DISABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: ENABLED"]
    #[inline(always)]
    pub fn ic_restart_en(&self) -> IC_RESTART_EN_R {
        IC_RESTART_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_master(&self) -> IC_10BITADDR_MASTER_R {
        IC_10BITADDR_MASTER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&self) -> IC_10BITADDR_SLAVE_R {
        IC_10BITADDR_SLAVE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.\\n\\n This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.\\n\\n 1: standard mode (100 kbit/s)\\n\\n 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)\\n\\n 3: high speed mode (3.4 Mbit/s)\\n\\n Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled.\\n\\n NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter.\\n\\n Reset value: 0x0."]
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&mut self) -> RX_FIFO_FULL_HLD_CTRL_W {
        RX_FIFO_FULL_HLD_CTRL_W { w: self }
    }
    #[doc = "Bit 8 - This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register.\\n\\n Reset value: 0x0."]
    #[inline(always)]
    pub fn tx_empty_ctrl(&mut self) -> TX_EMPTY_CTRL_W {
        TX_EMPTY_CTRL_W { w: self }
    }
    #[doc = "Bit 7 - In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0\\n\\n NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    #[inline(always)]
    pub fn stop_det_ifaddressed(&mut self) -> STOP_DET_IFADDRESSED_W {
        STOP_DET_IFADDRESSED_W { w: self }
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled.\\n\\n If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave.\\n\\n NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
    #[inline(always)]
    pub fn ic_slave_disable(&mut self) -> IC_SLAVE_DISABLE_W {
        IC_SLAVE_DISABLE_W { w: self }
    }
    #[doc = "Bit 5 - Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register.\\n\\n Reset value: ENABLED"]
    #[inline(always)]
    pub fn ic_restart_en(&mut self) -> IC_RESTART_EN_W {
        IC_RESTART_EN_W { w: self }
    }
    #[doc = "Bit 4 - Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_master(&mut self) -> IC_10BITADDR_MASTER_W {
        IC_10BITADDR_MASTER_W { w: self }
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&mut self) -> IC_10BITADDR_SLAVE_W {
        IC_10BITADDR_SLAVE_W { w: self }
    }
    #[doc = "Bits 1:2 - These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode.\\n\\n This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE.\\n\\n 1: standard mode (100 kbit/s)\\n\\n 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s)\\n\\n 3: high speed mode (3.4 Mbit/s)\\n\\n Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 0 - This bit controls whether the DW_apb_i2c master is enabled.\\n\\n NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W { w: self }
    }
}
