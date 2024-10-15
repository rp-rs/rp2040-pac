#[doc = "Register `IC_STATUS` reader"]
pub type R = crate::R<IC_STATUS_SPEC>;
#[doc = "I2C Activity Status. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type ACTIVITY_R = crate::BitReader<ACTIVITY_A>;
impl ACTIVITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTIVITY_A {
        match self.bits {
            false => ACTIVITY_A::INACTIVE,
            true => ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "I2C is idle"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ACTIVITY_A::INACTIVE
    }
    #[doc = "I2C is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVITY_A::ACTIVE
    }
}
#[doc = "Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type TFNF_R = crate::BitReader<TFNF_A>;
impl TFNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFNF_A {
        match self.bits {
            false => TFNF_A::FULL,
            true => TFNF_A::NOT_FULL,
        }
    }
    #[doc = "Tx FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TFNF_A::FULL
    }
    #[doc = "Tx FIFO not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TFNF_A::NOT_FULL
    }
}
#[doc = "Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type TFE_R = crate::BitReader<TFE_A>;
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::NON_EMPTY,
            true => TFE_A::EMPTY,
        }
    }
    #[doc = "Tx FIFO not empty"]
    #[inline(always)]
    pub fn is_non_empty(&self) -> bool {
        *self == TFE_A::NON_EMPTY
    }
    #[doc = "Tx FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TFE_A::EMPTY
    }
}
#[doc = "Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type RFNE_R = crate::BitReader<RFNE_A>;
impl RFNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFNE_A {
        match self.bits {
            false => RFNE_A::EMPTY,
            true => RFNE_A::NOT_EMPTY,
        }
    }
    #[doc = "Rx FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RFNE_A::EMPTY
    }
    #[doc = "Rx FIFO not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RFNE_A::NOT_EMPTY
    }
}
#[doc = "Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type RFF_R = crate::BitReader<RFF_A>;
impl RFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::NOT_FULL,
            true => RFF_A::FULL,
        }
    }
    #[doc = "Rx FIFO not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF_A::NOT_FULL
    }
    #[doc = "Rx FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF_A::FULL
    }
}
#[doc = "Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS\\[0\\]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type MST_ACTIVITY_R = crate::BitReader<MST_ACTIVITY_A>;
impl MST_ACTIVITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MST_ACTIVITY_A {
        match self.bits {
            false => MST_ACTIVITY_A::IDLE,
            true => MST_ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "Master is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MST_ACTIVITY_A::IDLE
    }
    #[doc = "Master not idle"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MST_ACTIVITY_A::ACTIVE
    }
}
#[doc = "Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type SLV_ACTIVITY_R = crate::BitReader<SLV_ACTIVITY_A>;
impl SLV_ACTIVITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLV_ACTIVITY_A {
        match self.bits {
            false => SLV_ACTIVITY_A::IDLE,
            true => SLV_ACTIVITY_A::ACTIVE,
        }
    }
    #[doc = "Slave is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLV_ACTIVITY_A::IDLE
    }
    #[doc = "Slave not idle"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLV_ACTIVITY_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - I2C Activity Status. Reset value: 0x0"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS\\[0\\]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn mst_activity(&self) -> MST_ACTIVITY_R {
        MST_ACTIVITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0"]
    #[inline(always)]
    pub fn slv_activity(&self) -> SLV_ACTIVITY_R {
        SLV_ACTIVITY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "I2C Status Register  

 This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt.  

 When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0  

You can [`read`](crate::Reg::read) this register and get [`ic_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_STATUS_SPEC;
impl crate::RegisterSpec for IC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_status::R`](R) reader structure"]
impl crate::Readable for IC_STATUS_SPEC {}
#[doc = "`reset()` method sets IC_STATUS to value 0x06"]
impl crate::Resettable for IC_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
