#[doc = "Register `IC_STATUS` reader"]
pub struct R(crate::R<IC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLV_ACTIVITY_A {
    #[doc = "0: Slave is idle"]
    IDLE = 0,
    #[doc = "1: Slave not idle"]
    ACTIVE = 1,
}
impl From<SLV_ACTIVITY_A> for bool {
    #[inline(always)]
    fn from(variant: SLV_ACTIVITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV_ACTIVITY` reader - Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0"]
pub struct SLV_ACTIVITY_R(crate::FieldReader<bool, SLV_ACTIVITY_A>);
impl SLV_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ACTIVITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLV_ACTIVITY_A {
        match self.bits {
            false => SLV_ACTIVITY_A::IDLE,
            true => SLV_ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == SLV_ACTIVITY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == SLV_ACTIVITY_A::ACTIVE
    }
}
impl core::ops::Deref for SLV_ACTIVITY_R {
    type Target = crate::FieldReader<bool, SLV_ACTIVITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS\\[0\\]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST_ACTIVITY_A {
    #[doc = "0: Master is idle"]
    IDLE = 0,
    #[doc = "1: Master not idle"]
    ACTIVE = 1,
}
impl From<MST_ACTIVITY_A> for bool {
    #[inline(always)]
    fn from(variant: MST_ACTIVITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST_ACTIVITY` reader - Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS\\[0\\]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.  

 Reset value: 0x0"]
pub struct MST_ACTIVITY_R(crate::FieldReader<bool, MST_ACTIVITY_A>);
impl MST_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_ACTIVITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_ACTIVITY_A {
        match self.bits {
            false => MST_ACTIVITY_A::IDLE,
            true => MST_ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == MST_ACTIVITY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == MST_ACTIVITY_A::ACTIVE
    }
}
impl core::ops::Deref for MST_ACTIVITY_R {
    type Target = crate::FieldReader<bool, MST_ACTIVITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF_A {
    #[doc = "0: Rx FIFO not full"]
    NOT_FULL = 0,
    #[doc = "1: Rx FIFO is full"]
    FULL = 1,
}
impl From<RFF_A> for bool {
    #[inline(always)]
    fn from(variant: RFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0"]
pub struct RFF_R(crate::FieldReader<bool, RFF_A>);
impl RFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::NOT_FULL,
            true => RFF_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        **self == RFF_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == RFF_A::FULL
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, RFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFNE_A {
    #[doc = "0: Rx FIFO is empty"]
    EMPTY = 0,
    #[doc = "1: Rx FIFO not empty"]
    NOT_EMPTY = 1,
}
impl From<RFNE_A> for bool {
    #[inline(always)]
    fn from(variant: RFNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFNE` reader - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0"]
pub struct RFNE_R(crate::FieldReader<bool, RFNE_A>);
impl RFNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFNE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFNE_A {
        match self.bits {
            false => RFNE_A::EMPTY,
            true => RFNE_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == RFNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == RFNE_A::NOT_EMPTY
    }
}
impl core::ops::Deref for RFNE_R {
    type Target = crate::FieldReader<bool, RFNE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
    #[doc = "0: Tx FIFO not empty"]
    NON_EMPTY = 0,
    #[doc = "1: Tx FIFO is empty"]
    EMPTY = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1"]
pub struct TFE_R(crate::FieldReader<bool, TFE_A>);
impl TFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::NON_EMPTY,
            true => TFE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NON_EMPTY`"]
    #[inline(always)]
    pub fn is_non_empty(&self) -> bool {
        **self == TFE_A::NON_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == TFE_A::EMPTY
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, TFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFNF_A {
    #[doc = "0: Tx FIFO is full"]
    FULL = 0,
    #[doc = "1: Tx FIFO not full"]
    NOT_FULL = 1,
}
impl From<TFNF_A> for bool {
    #[inline(always)]
    fn from(variant: TFNF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFNF` reader - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1"]
pub struct TFNF_R(crate::FieldReader<bool, TFNF_A>);
impl TFNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFNF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFNF_A {
        match self.bits {
            false => TFNF_A::FULL,
            true => TFNF_A::NOT_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == TFNF_A::FULL
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        **self == TFNF_A::NOT_FULL
    }
}
impl core::ops::Deref for TFNF_R {
    type Target = crate::FieldReader<bool, TFNF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "I2C Activity Status. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVITY_A {
    #[doc = "0: I2C is idle"]
    INACTIVE = 0,
    #[doc = "1: I2C is active"]
    ACTIVE = 1,
}
impl From<ACTIVITY_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVITY` reader - I2C Activity Status. Reset value: 0x0"]
pub struct ACTIVITY_R(crate::FieldReader<bool, ACTIVITY_A>);
impl ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVITY_A {
        match self.bits {
            false => ACTIVITY_A::INACTIVE,
            true => ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ACTIVITY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ACTIVITY_A::ACTIVE
    }
}
impl core::ops::Deref for ACTIVITY_R {
    type Target = crate::FieldReader<bool, ACTIVITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0"]
    #[inline(always)]
    pub fn slv_activity(&self) -> SLV_ACTIVITY_R {
        SLV_ACTIVITY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS\\[0\\]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn mst_activity(&self) -> MST_ACTIVITY_R {
        MST_ACTIVITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C Activity Status. Reset value: 0x0"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_status](index.html) module"]
pub struct IC_STATUS_SPEC;
impl crate::RegisterSpec for IC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_status::R](R) reader structure"]
impl crate::Readable for IC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_STATUS to value 0x06"]
impl crate::Resettable for IC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
