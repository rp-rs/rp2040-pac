#[doc = "Register `IC_SDA_HOLD` reader"]
pub struct R(crate::R<IC_SDA_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SDA_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SDA_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SDA_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_SDA_HOLD` writer"]
pub struct W(crate::W<IC_SDA_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SDA_HOLD_SPEC>;
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
impl From<crate::W<IC_SDA_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SDA_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_SDA_TX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
pub type IC_SDA_TX_HOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IC_SDA_TX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
pub type IC_SDA_TX_HOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IC_SDA_HOLD_SPEC, u16, u16, 16, O>;
#[doc = "Field `IC_SDA_RX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
pub type IC_SDA_RX_HOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC_SDA_RX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
pub type IC_SDA_RX_HOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IC_SDA_HOLD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
    #[inline(always)]
    pub fn ic_sda_tx_hold(&self) -> IC_SDA_TX_HOLD_R {
        IC_SDA_TX_HOLD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
    #[inline(always)]
    pub fn ic_sda_rx_hold(&self) -> IC_SDA_RX_HOLD_R {
        IC_SDA_RX_HOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sda_tx_hold(&mut self) -> IC_SDA_TX_HOLD_W<0> {
        IC_SDA_TX_HOLD_W::new(self)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sda_rx_hold(&mut self) -> IC_SDA_RX_HOLD_W<16> {
        IC_SDA_RX_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SDA Hold Time Length Register  

 The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).  

 The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]=0.  

 The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.  

 The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_sda_hold](index.html) module"]
pub struct IC_SDA_HOLD_SPEC;
impl crate::RegisterSpec for IC_SDA_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_sda_hold::R](R) reader structure"]
impl crate::Readable for IC_SDA_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_sda_hold::W](W) writer structure"]
impl crate::Writable for IC_SDA_HOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_SDA_HOLD to value 0x01"]
impl crate::Resettable for IC_SDA_HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
