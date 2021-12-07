#[doc = "Register `IC_TX_ABRT_SOURCE` reader"]
pub struct R(crate::R<IC_TX_ABRT_SOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_TX_ABRT_SOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_TX_ABRT_SOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_TX_ABRT_SOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_FLUSH_CNT` reader - This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
pub struct TX_FLUSH_CNT_R(crate::FieldReader<u16, u16>);
impl TX_FLUSH_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_FLUSH_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FLUSH_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE\\[1\\])  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_USER_ABRT_A {
    #[doc = "0: Transfer abort detected by master- scenario not present"]
    ABRT_USER_ABRT_VOID = 0,
    #[doc = "1: Transfer abort detected by master"]
    ABRT_USER_ABRT_GENERATED = 1,
}
impl From<ABRT_USER_ABRT_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_USER_ABRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_USER_ABRT` reader - This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE\\[1\\])  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
pub struct ABRT_USER_ABRT_R(crate::FieldReader<bool, ABRT_USER_ABRT_A>);
impl ABRT_USER_ABRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_USER_ABRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_USER_ABRT_A {
        match self.bits {
            false => ABRT_USER_ABRT_A::ABRT_USER_ABRT_VOID,
            true => ABRT_USER_ABRT_A::ABRT_USER_ABRT_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_USER_ABRT_VOID`"]
    #[inline(always)]
    pub fn is_abrt_user_abrt_void(&self) -> bool {
        **self == ABRT_USER_ABRT_A::ABRT_USER_ABRT_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_USER_ABRT_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_user_abrt_generated(&self) -> bool {
        **self == ABRT_USER_ABRT_A::ABRT_USER_ABRT_GENERATED
    }
}
impl core::ops::Deref for ABRT_USER_ABRT_R {
    type Target = crate::FieldReader<bool, ABRT_USER_ABRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_SLVRD_INTX_A {
    #[doc = "0: Slave trying to transmit to remote master in read mode- scenario not present"]
    ABRT_SLVRD_INTX_VOID = 0,
    #[doc = "1: Slave trying to transmit to remote master in read mode"]
    ABRT_SLVRD_INTX_GENERATED = 1,
}
impl From<ABRT_SLVRD_INTX_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_SLVRD_INTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_SLVRD_INTX` reader - 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter"]
pub struct ABRT_SLVRD_INTX_R(crate::FieldReader<bool, ABRT_SLVRD_INTX_A>);
impl ABRT_SLVRD_INTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SLVRD_INTX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_SLVRD_INTX_A {
        match self.bits {
            false => ABRT_SLVRD_INTX_A::ABRT_SLVRD_INTX_VOID,
            true => ABRT_SLVRD_INTX_A::ABRT_SLVRD_INTX_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_SLVRD_INTX_VOID`"]
    #[inline(always)]
    pub fn is_abrt_slvrd_intx_void(&self) -> bool {
        **self == ABRT_SLVRD_INTX_A::ABRT_SLVRD_INTX_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_SLVRD_INTX_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_slvrd_intx_generated(&self) -> bool {
        **self == ABRT_SLVRD_INTX_A::ABRT_SLVRD_INTX_GENERATED
    }
}
impl core::ops::Deref for ABRT_SLVRD_INTX_R {
    type Target = crate::FieldReader<bool, ABRT_SLVRD_INTX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE\\[12\\]
is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_SLV_ARBLOST_A {
    #[doc = "0: Slave lost arbitration to remote master- scenario not present"]
    ABRT_SLV_ARBLOST_VOID = 0,
    #[doc = "1: Slave lost arbitration to remote master"]
    ABRT_SLV_ARBLOST_GENERATED = 1,
}
impl From<ABRT_SLV_ARBLOST_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_SLV_ARBLOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_SLV_ARBLOST` reader - This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE\\[12\\]
is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter"]
pub struct ABRT_SLV_ARBLOST_R(crate::FieldReader<bool, ABRT_SLV_ARBLOST_A>);
impl ABRT_SLV_ARBLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SLV_ARBLOST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_SLV_ARBLOST_A {
        match self.bits {
            false => ABRT_SLV_ARBLOST_A::ABRT_SLV_ARBLOST_VOID,
            true => ABRT_SLV_ARBLOST_A::ABRT_SLV_ARBLOST_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_SLV_ARBLOST_VOID`"]
    #[inline(always)]
    pub fn is_abrt_slv_arblost_void(&self) -> bool {
        **self == ABRT_SLV_ARBLOST_A::ABRT_SLV_ARBLOST_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_SLV_ARBLOST_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_slv_arblost_generated(&self) -> bool {
        **self == ABRT_SLV_ARBLOST_A::ABRT_SLV_ARBLOST_GENERATED
    }
}
impl core::ops::Deref for ABRT_SLV_ARBLOST_R {
    type Target = crate::FieldReader<bool, ABRT_SLV_ARBLOST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_SLVFLUSH_TXFIFO_A {
    #[doc = "0: Slave flushes existing data in TX-FIFO upon getting read command- scenario not present"]
    ABRT_SLVFLUSH_TXFIFO_VOID = 0,
    #[doc = "1: Slave flushes existing data in TX-FIFO upon getting read command"]
    ABRT_SLVFLUSH_TXFIFO_GENERATED = 1,
}
impl From<ABRT_SLVFLUSH_TXFIFO_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_SLVFLUSH_TXFIFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_SLVFLUSH_TXFIFO` reader - This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter"]
pub struct ABRT_SLVFLUSH_TXFIFO_R(crate::FieldReader<bool, ABRT_SLVFLUSH_TXFIFO_A>);
impl ABRT_SLVFLUSH_TXFIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SLVFLUSH_TXFIFO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_SLVFLUSH_TXFIFO_A {
        match self.bits {
            false => ABRT_SLVFLUSH_TXFIFO_A::ABRT_SLVFLUSH_TXFIFO_VOID,
            true => ABRT_SLVFLUSH_TXFIFO_A::ABRT_SLVFLUSH_TXFIFO_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_SLVFLUSH_TXFIFO_VOID`"]
    #[inline(always)]
    pub fn is_abrt_slvflush_txfifo_void(&self) -> bool {
        **self == ABRT_SLVFLUSH_TXFIFO_A::ABRT_SLVFLUSH_TXFIFO_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_SLVFLUSH_TXFIFO_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_slvflush_txfifo_generated(&self) -> bool {
        **self == ABRT_SLVFLUSH_TXFIFO_A::ABRT_SLVFLUSH_TXFIFO_GENERATED
    }
}
impl core::ops::Deref for ABRT_SLVFLUSH_TXFIFO_R {
    type Target = crate::FieldReader<bool, ABRT_SLVFLUSH_TXFIFO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE\\[14\\]
is also set, then the slave transmitter has lost arbitration.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_LOST_A {
    #[doc = "0: Master or Slave-Transmitter lost arbitration- scenario not present"]
    ABRT_LOST_VOID = 0,
    #[doc = "1: Master or Slave-Transmitter lost arbitration"]
    ABRT_LOST_GENERATED = 1,
}
impl From<ARB_LOST_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_LOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB_LOST` reader - This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE\\[14\\]
is also set, then the slave transmitter has lost arbitration.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
pub struct ARB_LOST_R(crate::FieldReader<bool, ARB_LOST_A>);
impl ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_LOST_A {
        match self.bits {
            false => ARB_LOST_A::ABRT_LOST_VOID,
            true => ARB_LOST_A::ABRT_LOST_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_LOST_VOID`"]
    #[inline(always)]
    pub fn is_abrt_lost_void(&self) -> bool {
        **self == ARB_LOST_A::ABRT_LOST_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_LOST_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_lost_generated(&self) -> bool {
        **self == ARB_LOST_A::ABRT_LOST_GENERATED
    }
}
impl core::ops::Deref for ARB_LOST_R {
    type Target = crate::FieldReader<bool, ARB_LOST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the User tries to initiate a Master operation with the Master mode disabled.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_MASTER_DIS_A {
    #[doc = "0: User initiating master operation when MASTER disabled- scenario not present"]
    ABRT_MASTER_DIS_VOID = 0,
    #[doc = "1: User initiating master operation when MASTER disabled"]
    ABRT_MASTER_DIS_GENERATED = 1,
}
impl From<ABRT_MASTER_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_MASTER_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_MASTER_DIS` reader - This field indicates that the User tries to initiate a Master operation with the Master mode disabled.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
pub struct ABRT_MASTER_DIS_R(crate::FieldReader<bool, ABRT_MASTER_DIS_A>);
impl ABRT_MASTER_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_MASTER_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_MASTER_DIS_A {
        match self.bits {
            false => ABRT_MASTER_DIS_A::ABRT_MASTER_DIS_VOID,
            true => ABRT_MASTER_DIS_A::ABRT_MASTER_DIS_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_MASTER_DIS_VOID`"]
    #[inline(always)]
    pub fn is_abrt_master_dis_void(&self) -> bool {
        **self == ABRT_MASTER_DIS_A::ABRT_MASTER_DIS_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_MASTER_DIS_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_master_dis_generated(&self) -> bool {
        **self == ABRT_MASTER_DIS_A::ABRT_MASTER_DIS_GENERATED
    }
}
impl core::ops::Deref for ABRT_MASTER_DIS_R {
    type Target = crate::FieldReader<bool, ABRT_MASTER_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the master sends a read command in 10-bit addressing mode.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Receiver  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_10B_RD_NORSTRT_A {
    #[doc = "0: Master not trying to read in 10Bit addressing mode when RESTART disabled"]
    ABRT_10B_RD_VOID = 0,
    #[doc = "1: Master trying to read in 10Bit addressing mode when RESTART disabled"]
    ABRT_10B_RD_GENERATED = 1,
}
impl From<ABRT_10B_RD_NORSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_10B_RD_NORSTRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_10B_RD_NORSTRT` reader - This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the master sends a read command in 10-bit addressing mode.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Receiver"]
pub struct ABRT_10B_RD_NORSTRT_R(crate::FieldReader<bool, ABRT_10B_RD_NORSTRT_A>);
impl ABRT_10B_RD_NORSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_10B_RD_NORSTRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_10B_RD_NORSTRT_A {
        match self.bits {
            false => ABRT_10B_RD_NORSTRT_A::ABRT_10B_RD_VOID,
            true => ABRT_10B_RD_NORSTRT_A::ABRT_10B_RD_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_10B_RD_VOID`"]
    #[inline(always)]
    pub fn is_abrt_10b_rd_void(&self) -> bool {
        **self == ABRT_10B_RD_NORSTRT_A::ABRT_10B_RD_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_10B_RD_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_10b_rd_generated(&self) -> bool {
        **self == ABRT_10B_RD_NORSTRT_A::ABRT_10B_RD_GENERATED
    }
}
impl core::ops::Deref for ABRT_10B_RD_NORSTRT_R {
    type Target = crate::FieldReader<bool, ABRT_10B_RD_NORSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the user is trying to send a START Byte.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_SBYTE_NORSTRT_A {
    #[doc = "0: User trying to send START byte when RESTART disabled- scenario not present"]
    ABRT_SBYTE_NORSTRT_VOID = 0,
    #[doc = "1: User trying to send START byte when RESTART disabled"]
    ABRT_SBYTE_NORSTRT_GENERATED = 1,
}
impl From<ABRT_SBYTE_NORSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_SBYTE_NORSTRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_SBYTE_NORSTRT` reader - To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the user is trying to send a START Byte.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master"]
pub struct ABRT_SBYTE_NORSTRT_R(crate::FieldReader<bool, ABRT_SBYTE_NORSTRT_A>);
impl ABRT_SBYTE_NORSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SBYTE_NORSTRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_SBYTE_NORSTRT_A {
        match self.bits {
            false => ABRT_SBYTE_NORSTRT_A::ABRT_SBYTE_NORSTRT_VOID,
            true => ABRT_SBYTE_NORSTRT_A::ABRT_SBYTE_NORSTRT_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_SBYTE_NORSTRT_VOID`"]
    #[inline(always)]
    pub fn is_abrt_sbyte_norstrt_void(&self) -> bool {
        **self == ABRT_SBYTE_NORSTRT_A::ABRT_SBYTE_NORSTRT_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_SBYTE_NORSTRT_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_sbyte_norstrt_generated(&self) -> bool {
        **self == ABRT_SBYTE_NORSTRT_A::ABRT_SBYTE_NORSTRT_GENERATED
    }
}
impl core::ops::Deref for ABRT_SBYTE_NORSTRT_R {
    type Target = crate::FieldReader<bool, ABRT_SBYTE_NORSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the user is trying to use the master to transfer data in High Speed mode.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_HS_NORSTRT_A {
    #[doc = "0: User trying to switch Master to HS mode when RESTART disabled- scenario not present"]
    ABRT_HS_NORSTRT_VOID = 0,
    #[doc = "1: User trying to switch Master to HS mode when RESTART disabled"]
    ABRT_HS_NORSTRT_GENERATED = 1,
}
impl From<ABRT_HS_NORSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_HS_NORSTRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_HS_NORSTRT` reader - This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the user is trying to use the master to transfer data in High Speed mode.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
pub struct ABRT_HS_NORSTRT_R(crate::FieldReader<bool, ABRT_HS_NORSTRT_A>);
impl ABRT_HS_NORSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_HS_NORSTRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_HS_NORSTRT_A {
        match self.bits {
            false => ABRT_HS_NORSTRT_A::ABRT_HS_NORSTRT_VOID,
            true => ABRT_HS_NORSTRT_A::ABRT_HS_NORSTRT_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_HS_NORSTRT_VOID`"]
    #[inline(always)]
    pub fn is_abrt_hs_norstrt_void(&self) -> bool {
        **self == ABRT_HS_NORSTRT_A::ABRT_HS_NORSTRT_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_HS_NORSTRT_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_hs_norstrt_generated(&self) -> bool {
        **self == ABRT_HS_NORSTRT_A::ABRT_HS_NORSTRT_GENERATED
    }
}
impl core::ops::Deref for ABRT_HS_NORSTRT_R {
    type Target = crate::FieldReader<bool, ABRT_HS_NORSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_SBYTE_ACKDET_A {
    #[doc = "0: ACK detected for START byte- scenario not present"]
    ABRT_SBYTE_ACKDET_VOID = 0,
    #[doc = "1: ACK detected for START byte"]
    ABRT_SBYTE_ACKDET_GENERATED = 1,
}
impl From<ABRT_SBYTE_ACKDET_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_SBYTE_ACKDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_SBYTE_ACKDET` reader - This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master"]
pub struct ABRT_SBYTE_ACKDET_R(crate::FieldReader<bool, ABRT_SBYTE_ACKDET_A>);
impl ABRT_SBYTE_ACKDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SBYTE_ACKDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_SBYTE_ACKDET_A {
        match self.bits {
            false => ABRT_SBYTE_ACKDET_A::ABRT_SBYTE_ACKDET_VOID,
            true => ABRT_SBYTE_ACKDET_A::ABRT_SBYTE_ACKDET_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_SBYTE_ACKDET_VOID`"]
    #[inline(always)]
    pub fn is_abrt_sbyte_ackdet_void(&self) -> bool {
        **self == ABRT_SBYTE_ACKDET_A::ABRT_SBYTE_ACKDET_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_SBYTE_ACKDET_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_sbyte_ackdet_generated(&self) -> bool {
        **self == ABRT_SBYTE_ACKDET_A::ABRT_SBYTE_ACKDET_GENERATED
    }
}
impl core::ops::Deref for ABRT_SBYTE_ACKDET_R {
    type Target = crate::FieldReader<bool, ABRT_SBYTE_ACKDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_HS_ACKDET_A {
    #[doc = "0: HS Master code ACKed in HS Mode- scenario not present"]
    ABRT_HS_ACK_VOID = 0,
    #[doc = "1: HS Master code ACKed in HS Mode"]
    ABRT_HS_ACK_GENERATED = 1,
}
impl From<ABRT_HS_ACKDET_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_HS_ACKDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_HS_ACKDET` reader - This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master"]
pub struct ABRT_HS_ACKDET_R(crate::FieldReader<bool, ABRT_HS_ACKDET_A>);
impl ABRT_HS_ACKDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_HS_ACKDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_HS_ACKDET_A {
        match self.bits {
            false => ABRT_HS_ACKDET_A::ABRT_HS_ACK_VOID,
            true => ABRT_HS_ACKDET_A::ABRT_HS_ACK_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_HS_ACK_VOID`"]
    #[inline(always)]
    pub fn is_abrt_hs_ack_void(&self) -> bool {
        **self == ABRT_HS_ACKDET_A::ABRT_HS_ACK_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_HS_ACK_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_hs_ack_generated(&self) -> bool {
        **self == ABRT_HS_ACKDET_A::ABRT_HS_ACK_GENERATED
    }
}
impl core::ops::Deref for ABRT_HS_ACKDET_R {
    type Target = crate::FieldReader<bool, ABRT_HS_ACKDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD\\[9\\]
is set to 1).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_GCALL_READ_A {
    #[doc = "0: GCALL is followed by read from bus-scenario not present"]
    ABRT_GCALL_READ_VOID = 0,
    #[doc = "1: GCALL is followed by read from bus"]
    ABRT_GCALL_READ_GENERATED = 1,
}
impl From<ABRT_GCALL_READ_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_GCALL_READ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_GCALL_READ` reader - This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD\\[9\\]
is set to 1).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
pub struct ABRT_GCALL_READ_R(crate::FieldReader<bool, ABRT_GCALL_READ_A>);
impl ABRT_GCALL_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_GCALL_READ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_GCALL_READ_A {
        match self.bits {
            false => ABRT_GCALL_READ_A::ABRT_GCALL_READ_VOID,
            true => ABRT_GCALL_READ_A::ABRT_GCALL_READ_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_GCALL_READ_VOID`"]
    #[inline(always)]
    pub fn is_abrt_gcall_read_void(&self) -> bool {
        **self == ABRT_GCALL_READ_A::ABRT_GCALL_READ_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_GCALL_READ_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_gcall_read_generated(&self) -> bool {
        **self == ABRT_GCALL_READ_A::ABRT_GCALL_READ_GENERATED
    }
}
impl core::ops::Deref for ABRT_GCALL_READ_R {
    type Target = crate::FieldReader<bool, ABRT_GCALL_READ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_GCALL_NOACK_A {
    #[doc = "0: GCALL not ACKed by any slave-scenario not present"]
    ABRT_GCALL_NOACK_VOID = 0,
    #[doc = "1: GCALL not ACKed by any slave"]
    ABRT_GCALL_NOACK_GENERATED = 1,
}
impl From<ABRT_GCALL_NOACK_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_GCALL_NOACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_GCALL_NOACK` reader - This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
pub struct ABRT_GCALL_NOACK_R(crate::FieldReader<bool, ABRT_GCALL_NOACK_A>);
impl ABRT_GCALL_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_GCALL_NOACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_GCALL_NOACK_A {
        match self.bits {
            false => ABRT_GCALL_NOACK_A::ABRT_GCALL_NOACK_VOID,
            true => ABRT_GCALL_NOACK_A::ABRT_GCALL_NOACK_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_GCALL_NOACK_VOID`"]
    #[inline(always)]
    pub fn is_abrt_gcall_noack_void(&self) -> bool {
        **self == ABRT_GCALL_NOACK_A::ABRT_GCALL_NOACK_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_GCALL_NOACK_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_gcall_noack_generated(&self) -> bool {
        **self == ABRT_GCALL_NOACK_A::ABRT_GCALL_NOACK_GENERATED
    }
}
impl core::ops::Deref for ABRT_GCALL_NOACK_R {
    type Target = crate::FieldReader<bool, ABRT_GCALL_NOACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_TXDATA_NOACK_A {
    #[doc = "0: Transmitted data non-ACKed by addressed slave-scenario not present"]
    ABRT_TXDATA_NOACK_VOID = 0,
    #[doc = "1: Transmitted data not ACKed by addressed slave"]
    ABRT_TXDATA_NOACK_GENERATED = 1,
}
impl From<ABRT_TXDATA_NOACK_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_TXDATA_NOACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_TXDATA_NOACK` reader - This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
pub struct ABRT_TXDATA_NOACK_R(crate::FieldReader<bool, ABRT_TXDATA_NOACK_A>);
impl ABRT_TXDATA_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_TXDATA_NOACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_TXDATA_NOACK_A {
        match self.bits {
            false => ABRT_TXDATA_NOACK_A::ABRT_TXDATA_NOACK_VOID,
            true => ABRT_TXDATA_NOACK_A::ABRT_TXDATA_NOACK_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `ABRT_TXDATA_NOACK_VOID`"]
    #[inline(always)]
    pub fn is_abrt_txdata_noack_void(&self) -> bool {
        **self == ABRT_TXDATA_NOACK_A::ABRT_TXDATA_NOACK_VOID
    }
    #[doc = "Checks if the value of the field is `ABRT_TXDATA_NOACK_GENERATED`"]
    #[inline(always)]
    pub fn is_abrt_txdata_noack_generated(&self) -> bool {
        **self == ABRT_TXDATA_NOACK_A::ABRT_TXDATA_NOACK_GENERATED
    }
}
impl core::ops::Deref for ABRT_TXDATA_NOACK_R {
    type Target = crate::FieldReader<bool, ABRT_TXDATA_NOACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_10ADDR2_NOACK_A {
    #[doc = "0: This abort is not generated"]
    INACTIVE = 0,
    #[doc = "1: Byte 2 of 10Bit Address not ACKed by any slave"]
    ACTIVE = 1,
}
impl From<ABRT_10ADDR2_NOACK_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_10ADDR2_NOACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_10ADDR2_NOACK` reader - This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
pub struct ABRT_10ADDR2_NOACK_R(crate::FieldReader<bool, ABRT_10ADDR2_NOACK_A>);
impl ABRT_10ADDR2_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_10ADDR2_NOACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_10ADDR2_NOACK_A {
        match self.bits {
            false => ABRT_10ADDR2_NOACK_A::INACTIVE,
            true => ABRT_10ADDR2_NOACK_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ABRT_10ADDR2_NOACK_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ABRT_10ADDR2_NOACK_A::ACTIVE
    }
}
impl core::ops::Deref for ABRT_10ADDR2_NOACK_R {
    type Target = crate::FieldReader<bool, ABRT_10ADDR2_NOACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_10ADDR1_NOACK_A {
    #[doc = "0: This abort is not generated"]
    INACTIVE = 0,
    #[doc = "1: Byte 1 of 10Bit Address not ACKed by any slave"]
    ACTIVE = 1,
}
impl From<ABRT_10ADDR1_NOACK_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_10ADDR1_NOACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_10ADDR1_NOACK` reader - This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
pub struct ABRT_10ADDR1_NOACK_R(crate::FieldReader<bool, ABRT_10ADDR1_NOACK_A>);
impl ABRT_10ADDR1_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_10ADDR1_NOACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_10ADDR1_NOACK_A {
        match self.bits {
            false => ABRT_10ADDR1_NOACK_A::INACTIVE,
            true => ABRT_10ADDR1_NOACK_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ABRT_10ADDR1_NOACK_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ABRT_10ADDR1_NOACK_A::ACTIVE
    }
}
impl core::ops::Deref for ABRT_10ADDR1_NOACK_R {
    type Target = crate::FieldReader<bool, ABRT_10ADDR1_NOACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRT_7B_ADDR_NOACK_A {
    #[doc = "0: This abort is not generated"]
    INACTIVE = 0,
    #[doc = "1: This abort is generated because of NOACK for 7-bit address"]
    ACTIVE = 1,
}
impl From<ABRT_7B_ADDR_NOACK_A> for bool {
    #[inline(always)]
    fn from(variant: ABRT_7B_ADDR_NOACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRT_7B_ADDR_NOACK` reader - This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
pub struct ABRT_7B_ADDR_NOACK_R(crate::FieldReader<bool, ABRT_7B_ADDR_NOACK_A>);
impl ABRT_7B_ADDR_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_7B_ADDR_NOACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABRT_7B_ADDR_NOACK_A {
        match self.bits {
            false => ABRT_7B_ADDR_NOACK_A::INACTIVE,
            true => ABRT_7B_ADDR_NOACK_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ABRT_7B_ADDR_NOACK_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ABRT_7B_ADDR_NOACK_A::ACTIVE
    }
}
impl core::ops::Deref for ABRT_7B_ADDR_NOACK_R {
    type Target = crate::FieldReader<bool, ABRT_7B_ADDR_NOACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 23:31 - This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
    #[inline(always)]
    pub fn tx_flush_cnt(&self) -> TX_FLUSH_CNT_R {
        TX_FLUSH_CNT_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE\\[1\\])  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
    #[inline(always)]
    pub fn abrt_user_abrt(&self) -> ABRT_USER_ABRT_R {
        ABRT_USER_ABRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter"]
    #[inline(always)]
    pub fn abrt_slvrd_intx(&self) -> ABRT_SLVRD_INTX_R {
        ABRT_SLVRD_INTX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE\\[12\\]
is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter"]
    #[inline(always)]
    pub fn abrt_slv_arblost(&self) -> ABRT_SLV_ARBLOST_R {
        ABRT_SLV_ARBLOST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Slave-Transmitter"]
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(&self) -> ABRT_SLVFLUSH_TXFIFO_R {
        ABRT_SLVFLUSH_TXFIFO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE\\[14\\]
is also set, then the slave transmitter has lost arbitration.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This field indicates that the User tries to initiate a Master operation with the Master mode disabled.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    #[inline(always)]
    pub fn abrt_master_dis(&self) -> ABRT_MASTER_DIS_R {
        ABRT_MASTER_DIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the master sends a read command in 10-bit addressing mode.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Receiver"]
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(&self) -> ABRT_10B_RD_NORSTRT_R {
        ABRT_10B_RD_NORSTRT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the user is trying to send a START Byte.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master"]
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(&self) -> ABRT_SBYTE_NORSTRT_R {
        ABRT_SBYTE_NORSTRT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON\\[5\\]) =0) and the user is trying to use the master to transfer data in High Speed mode.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    #[inline(always)]
    pub fn abrt_hs_norstrt(&self) -> ABRT_HS_NORSTRT_R {
        ABRT_HS_NORSTRT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master"]
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(&self) -> ABRT_SBYTE_ACKDET_R {
        ABRT_SBYTE_ACKDET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master"]
    #[inline(always)]
    pub fn abrt_hs_ackdet(&self) -> ABRT_HS_ACKDET_R {
        ABRT_HS_ACKDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD\\[9\\]
is set to 1).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
    #[inline(always)]
    pub fn abrt_gcall_read(&self) -> ABRT_GCALL_READ_R {
        ABRT_GCALL_READ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
    #[inline(always)]
    pub fn abrt_gcall_noack(&self) -> ABRT_GCALL_NOACK_R {
        ABRT_GCALL_NOACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s).  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter"]
    #[inline(always)]
    pub fn abrt_txdata_noack(&self) -> ABRT_TXDATA_NOACK_R {
        ABRT_TXDATA_NOACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    #[inline(always)]
    pub fn abrt_10addr2_noack(&self) -> ABRT_10ADDR2_NOACK_R {
        ABRT_10ADDR2_NOACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    #[inline(always)]
    pub fn abrt_10addr1_noack(&self) -> ABRT_10ADDR1_NOACK_R {
        ABRT_10ADDR1_NOACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave.  

 Reset value: 0x0  

 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    #[inline(always)]
    pub fn abrt_7b_addr_noack(&self) -> ABRT_7B_ADDR_NOACK_R {
        ABRT_7B_ADDR_NOACK_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "I2C Transmit Abort Source Register  

 This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON\\[5\\]=1), the SPECIAL bit must be cleared (IC_TAR\\[11\\]), or the GC_OR_START bit must be cleared (IC_TAR\\[10\\]).  

 Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_tx_abrt_source](index.html) module"]
pub struct IC_TX_ABRT_SOURCE_SPEC;
impl crate::RegisterSpec for IC_TX_ABRT_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_tx_abrt_source::R](R) reader structure"]
impl crate::Readable for IC_TX_ABRT_SOURCE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_TX_ABRT_SOURCE to value 0"]
impl crate::Resettable for IC_TX_ABRT_SOURCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
