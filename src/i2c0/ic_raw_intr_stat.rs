#[doc = "Register `IC_RAW_INTR_STAT` reader"]
pub struct R(crate::R<IC_RAW_INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_RAW_INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_RAW_INTR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_RAW_INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.  

 Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTART_DET_A {
    #[doc = "0: RESTART_DET interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RESTART_DET interrupt is active"]
    ACTIVE = 1,
}
impl From<RESTART_DET_A> for bool {
    #[inline(always)]
    fn from(variant: RESTART_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESTART_DET` reader - Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.  

 Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.  

 Reset value: 0x0"]
pub struct RESTART_DET_R(crate::FieldReader<bool, RESTART_DET_A>);
impl RESTART_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESTART_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTART_DET_A {
        match self.bits {
            false => RESTART_DET_A::INACTIVE,
            true => RESTART_DET_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == RESTART_DET_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == RESTART_DET_A::ACTIVE
    }
}
impl core::ops::Deref for RESTART_DET_R {
    type Target = crate::FieldReader<bool, RESTART_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEN_CALL_A {
    #[doc = "0: GEN_CALL interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: GEN_CALL interrupt is active"]
    ACTIVE = 1,
}
impl From<GEN_CALL_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_CALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEN_CALL` reader - Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.  

 Reset value: 0x0"]
pub struct GEN_CALL_R(crate::FieldReader<bool, GEN_CALL_A>);
impl GEN_CALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GEN_CALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEN_CALL_A {
        match self.bits {
            false => GEN_CALL_A::INACTIVE,
            true => GEN_CALL_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == GEN_CALL_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == GEN_CALL_A::ACTIVE
    }
}
impl core::ops::Deref for GEN_CALL_R {
    type Target = crate::FieldReader<bool, GEN_CALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_DET_A {
    #[doc = "0: START_DET interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: START_DET interrupt is active"]
    ACTIVE = 1,
}
impl From<START_DET_A> for bool {
    #[inline(always)]
    fn from(variant: START_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_DET` reader - Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.  

 Reset value: 0x0"]
pub struct START_DET_R(crate::FieldReader<bool, START_DET_A>);
impl START_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_DET_A {
        match self.bits {
            false => START_DET_A::INACTIVE,
            true => START_DET_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == START_DET_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == START_DET_A::ACTIVE
    }
}
impl core::ops::Deref for START_DET_R {
    type Target = crate::FieldReader<bool, START_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.  

 In Slave Mode: - If IC_CON\\[7\\]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON\\[7\\]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON\\[10\\]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON\\[10\\]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_DET_A {
    #[doc = "0: STOP_DET interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: STOP_DET interrupt is active"]
    ACTIVE = 1,
}
impl From<STOP_DET_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_DET` reader - Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.  

 In Slave Mode: - If IC_CON\\[7\\]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON\\[7\\]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON\\[10\\]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON\\[10\\]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0"]
pub struct STOP_DET_R(crate::FieldReader<bool, STOP_DET_A>);
impl STOP_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_DET_A {
        match self.bits {
            false => STOP_DET_A::INACTIVE,
            true => STOP_DET_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == STOP_DET_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == STOP_DET_A::ACTIVE
    }
}
impl core::ops::Deref for STOP_DET_R {
    type Target = crate::FieldReader<bool, STOP_DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVITY_A {
    #[doc = "0: RAW_INTR_ACTIVITY interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RAW_INTR_ACTIVITY interrupt is active"]
    ACTIVE = 1,
}
impl From<ACTIVITY_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVITY` reader - This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.  

 Reset value: 0x0"]
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
#[doc = "When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DONE_A {
    #[doc = "0: RX_DONE interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RX_DONE interrupt is active"]
    ACTIVE = 1,
}
impl From<RX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DONE` reader - When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.  

 Reset value: 0x0"]
pub struct RX_DONE_R(crate::FieldReader<bool, RX_DONE_A>);
impl RX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DONE_A {
        match self.bits {
            false => RX_DONE_A::INACTIVE,
            true => RX_DONE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == RX_DONE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == RX_DONE_A::ACTIVE
    }
}
impl core::ops::Deref for RX_DONE_R {
    type Target = crate::FieldReader<bool, RX_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.  

 Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ABRT_A {
    #[doc = "0: TX_ABRT interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: TX_ABRT interrupt is active"]
    ACTIVE = 1,
}
impl From<TX_ABRT_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ABRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_ABRT` reader - This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.  

 Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.  

 Reset value: 0x0"]
pub struct TX_ABRT_R(crate::FieldReader<bool, TX_ABRT_A>);
impl TX_ABRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_ABRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ABRT_A {
        match self.bits {
            false => TX_ABRT_A::INACTIVE,
            true => TX_ABRT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == TX_ABRT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == TX_ABRT_A::ACTIVE
    }
}
impl core::ops::Deref for TX_ABRT_R {
    type Target = crate::FieldReader<bool, TX_ABRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_REQ_A {
    #[doc = "0: RD_REQ interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RD_REQ interrupt is active"]
    ACTIVE = 1,
}
impl From<RD_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: RD_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_REQ` reader - This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.  

 Reset value: 0x0"]
pub struct RD_REQ_R(crate::FieldReader<bool, RD_REQ_A>);
impl RD_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_REQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_REQ_A {
        match self.bits {
            false => RD_REQ_A::INACTIVE,
            true => RD_REQ_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == RD_REQ_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == RD_REQ_A::ACTIVE
    }
}
impl core::ops::Deref for RD_REQ_R {
    type Target = crate::FieldReader<bool, RD_REQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE\\[0\\]
is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.  

 Reset value: 0x0.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EMPTY_A {
    #[doc = "0: TX_EMPTY interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: TX_EMPTY interrupt is active"]
    ACTIVE = 1,
}
impl From<TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EMPTY` reader - The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE\\[0\\]
is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.  

 Reset value: 0x0."]
pub struct TX_EMPTY_R(crate::FieldReader<bool, TX_EMPTY_A>);
impl TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_EMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_A {
        match self.bits {
            false => TX_EMPTY_A::INACTIVE,
            true => TX_EMPTY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == TX_EMPTY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == TX_EMPTY_A::ACTIVE
    }
}
impl core::ops::Deref for TX_EMPTY_R {
    type Target = crate::FieldReader<bool, TX_EMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_OVER_A {
    #[doc = "0: TX_OVER interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: TX_OVER interrupt is active"]
    ACTIVE = 1,
}
impl From<TX_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: TX_OVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OVER` reader - Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Reset value: 0x0"]
pub struct TX_OVER_R(crate::FieldReader<bool, TX_OVER_A>);
impl TX_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_OVER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_OVER_A {
        match self.bits {
            false => TX_OVER_A::INACTIVE,
            true => TX_OVER_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == TX_OVER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == TX_OVER_A::ACTIVE
    }
}
impl core::ops::Deref for TX_OVER_R {
    type Target = crate::FieldReader<bool, TX_OVER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FULL_A {
    #[doc = "0: RX_FULL interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RX_FULL interrupt is active"]
    ACTIVE = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FULL` reader - Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.  

 Reset value: 0x0"]
pub struct RX_FULL_R(crate::FieldReader<bool, RX_FULL_A>);
impl RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::INACTIVE,
            true => RX_FULL_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == RX_FULL_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == RX_FULL_A::ACTIVE
    }
}
impl core::ops::Deref for RX_FULL_R {
    type Target = crate::FieldReader<bool, RX_FULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OVER_A {
    #[doc = "0: RX_OVER interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RX_OVER interrupt is active"]
    ACTIVE = 1,
}
impl From<RX_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OVER` reader - Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.  

 Reset value: 0x0"]
pub struct RX_OVER_R(crate::FieldReader<bool, RX_OVER_A>);
impl RX_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OVER_A {
        match self.bits {
            false => RX_OVER_A::INACTIVE,
            true => RX_OVER_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == RX_OVER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == RX_OVER_A::ACTIVE
    }
}
impl core::ops::Deref for RX_OVER_R {
    type Target = crate::FieldReader<bool, RX_OVER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_UNDER_A {
    #[doc = "0: RX_UNDER interrupt is inactive"]
    INACTIVE = 0,
    #[doc = "1: RX_UNDER interrupt is active"]
    ACTIVE = 1,
}
impl From<RX_UNDER_A> for bool {
    #[inline(always)]
    fn from(variant: RX_UNDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_UNDER` reader - Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Reset value: 0x0"]
pub struct RX_UNDER_R(crate::FieldReader<bool, RX_UNDER_A>);
impl RX_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_UNDER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_UNDER_A {
        match self.bits {
            false => RX_UNDER_A::INACTIVE,
            true => RX_UNDER_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == RX_UNDER_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == RX_UNDER_A::ACTIVE
    }
}
impl core::ops::Deref for RX_UNDER_R {
    type Target = crate::FieldReader<bool, RX_UNDER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 12 - Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1.  

 Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn restart_det(&self) -> RESTART_DET_R {
        RESTART_DET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn gen_call(&self) -> GEN_CALL_R {
        GEN_CALL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn start_det(&self) -> START_DET_R {
        START_DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode.  

 In Slave Mode: - If IC_CON\\[7\\]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON\\[7\\]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON\\[10\\]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON\\[10\\]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0"]
    #[inline(always)]
    pub fn stop_det(&self) -> STOP_DET_R {
        STOP_DET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places.  

 Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE\\[0\\]
is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0.  

 Reset value: 0x0."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn rx_over(&self) -> RX_OVER_R {
        RX_OVER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn rx_under(&self) -> RX_UNDER_R {
        RX_UNDER_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "I2C Raw Interrupt Status Register  

 Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_raw_intr_stat](index.html) module"]
pub struct IC_RAW_INTR_STAT_SPEC;
impl crate::RegisterSpec for IC_RAW_INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_raw_intr_stat::R](R) reader structure"]
impl crate::Readable for IC_RAW_INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_RAW_INTR_STAT to value 0"]
impl crate::Resettable for IC_RAW_INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
