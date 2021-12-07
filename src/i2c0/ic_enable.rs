#[doc = "Register `IC_ENABLE` reader"]
pub struct R(crate::R<IC_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_ENABLE` writer"]
pub struct W(crate::W<IC_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IC_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS\\[2\\]==1) and Master is in Idle state (IC_STATUS\\[5\\]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CMD_BLOCK_A {
    #[doc = "0: Tx Command execution not blocked"]
    NOT_BLOCKED = 0,
    #[doc = "1: Tx Command execution blocked"]
    BLOCKED = 1,
}
impl From<TX_CMD_BLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CMD_BLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CMD_BLOCK` reader - In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS\\[2\\]==1) and Master is in Idle state (IC_STATUS\\[5\\]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT"]
pub struct TX_CMD_BLOCK_R(crate::FieldReader<bool, TX_CMD_BLOCK_A>);
impl TX_CMD_BLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CMD_BLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CMD_BLOCK_A {
        match self.bits {
            false => TX_CMD_BLOCK_A::NOT_BLOCKED,
            true => TX_CMD_BLOCK_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BLOCKED`"]
    #[inline(always)]
    pub fn is_not_blocked(&self) -> bool {
        **self == TX_CMD_BLOCK_A::NOT_BLOCKED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == TX_CMD_BLOCK_A::BLOCKED
    }
}
impl core::ops::Deref for TX_CMD_BLOCK_R {
    type Target = crate::FieldReader<bool, TX_CMD_BLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CMD_BLOCK` writer - In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS\\[2\\]==1) and Master is in Idle state (IC_STATUS\\[5\\]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT"]
pub struct TX_CMD_BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CMD_BLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CMD_BLOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx Command execution not blocked"]
    #[inline(always)]
    pub fn not_blocked(self) -> &'a mut W {
        self.variant(TX_CMD_BLOCK_A::NOT_BLOCKED)
    }
    #[doc = "Tx Command execution blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(TX_CMD_BLOCK_A::BLOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.  

 For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_A {
    #[doc = "0: ABORT operation not in progress"]
    DISABLE = 0,
    #[doc = "1: ABORT operation in progress"]
    ENABLED = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.  

 For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.  

 Reset value: 0x0"]
pub struct ABORT_R(crate::FieldReader<bool, ABORT_A>);
impl ABORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::DISABLE,
            true => ABORT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ABORT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ABORT_A::ENABLED
    }
}
impl core::ops::Deref for ABORT_R {
    type Target = crate::FieldReader<bool, ABORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT` writer - When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.  

 For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.  

 Reset value: 0x0"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABORT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ABORT operation not in progress"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABORT_A::DISABLE)
    }
    #[doc = "ABORT operation in progress"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABORT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.  

 When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.  

 In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: I2C is disabled"]
    DISABLED = 0,
    #[doc = "1: I2C is enabled"]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.  

 When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.  

 In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'  

 Reset value: 0x0"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLE_A::ENABLED
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.  

 When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.  

 In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'  

 Reset value: 0x0"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "I2C is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS\\[2\\]==1) and Master is in Idle state (IC_STATUS\\[5\\]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT"]
    #[inline(always)]
    pub fn tx_cmd_block(&self) -> TX_CMD_BLOCK_R {
        TX_CMD_BLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.  

 For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.  

 When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.  

 In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS\\[2\\]==1) and Master is in Idle state (IC_STATUS\\[5\\]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT"]
    #[inline(always)]
    pub fn tx_cmd_block(&mut self) -> TX_CMD_BLOCK_W {
        TX_CMD_BLOCK_W { w: self }
    }
    #[doc = "Bit 1 - When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation.  

 For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 0 - Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'.  

 When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer.  

 In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c'  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Enable Register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_enable](index.html) module"]
pub struct IC_ENABLE_SPEC;
impl crate::RegisterSpec for IC_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_enable::R](R) reader structure"]
impl crate::Readable for IC_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_enable::W](W) writer structure"]
impl crate::Writable for IC_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_ENABLE to value 0"]
impl crate::Resettable for IC_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
