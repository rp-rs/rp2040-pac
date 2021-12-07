#[doc = "Register `IC_SLV_DATA_NACK_ONLY` reader"]
pub struct R(crate::R<IC_SLV_DATA_NACK_ONLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SLV_DATA_NACK_ONLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SLV_DATA_NACK_ONLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SLV_DATA_NACK_ONLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_SLV_DATA_NACK_ONLY` writer"]
pub struct W(crate::W<IC_SLV_DATA_NACK_ONLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SLV_DATA_NACK_ONLY_SPEC>;
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
impl From<crate::W<IC_SLV_DATA_NACK_ONLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SLV_DATA_NACK_ONLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.  

 When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: Slave receiver generates NACK normally"]
    DISABLED = 0,
    #[doc = "1: Slave receiver generates NACK upon data reception only"]
    ENABLED = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.  

 When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0"]
pub struct NACK_R(crate::FieldReader<bool, NACK_A>);
impl NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::DISABLED,
            true => NACK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NACK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NACK_A::ENABLED
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, NACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK` writer - Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.  

 When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave receiver generates NACK normally"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACK_A::DISABLED)
    }
    #[doc = "Slave receiver generates NACK upon data reception only"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACK_A::ENABLED)
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
    #[doc = "Bit 0 - Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.  

 When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer.  

 When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generate Slave Data NACK Register  

 The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect.  

 A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE\\[0\\]
= 0) - Slave part is inactive (IC_STATUS\\[6\\]
= 0) Note: The IC_STATUS\\[6\\]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_slv_data_nack_only](index.html) module"]
pub struct IC_SLV_DATA_NACK_ONLY_SPEC;
impl crate::RegisterSpec for IC_SLV_DATA_NACK_ONLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_slv_data_nack_only::R](R) reader structure"]
impl crate::Readable for IC_SLV_DATA_NACK_ONLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_slv_data_nack_only::W](W) writer structure"]
impl crate::Writable for IC_SLV_DATA_NACK_ONLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_SLV_DATA_NACK_ONLY to value 0"]
impl crate::Resettable for IC_SLV_DATA_NACK_ONLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
