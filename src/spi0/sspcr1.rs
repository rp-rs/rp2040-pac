#[doc = "Register `SSPCR1` reader"]
pub struct R(crate::R<SSPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPCR1` writer"]
pub struct W(crate::W<SSPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPCR1_SPEC>;
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
impl From<crate::W<SSPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOD` reader - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
pub struct SOD_R(crate::FieldReader<bool, bool>);
impl SOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOD` writer - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
pub struct SOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MS` reader - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
pub struct MS_R(crate::FieldReader<bool, bool>);
impl MS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS` writer - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
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
#[doc = "Field `SSE` reader - Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
pub struct SSE_R(crate::FieldReader<bool, bool>);
impl SSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSE` writer - Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
pub struct SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_W<'a> {
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
#[doc = "Field `LBM` reader - Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
pub struct LBM_R(crate::FieldReader<bool, bool>);
impl LBM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBM` writer - Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
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
    #[doc = "Bit 3 - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    #[inline(always)]
    pub fn sod(&mut self) -> SOD_W {
        SOD_W { w: self }
    }
    #[doc = "Bit 2 - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bit 1 - Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W { w: self }
    }
    #[doc = "Bit 0 - Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1, SSPCR1 on page 3-5  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspcr1](index.html) module"]
pub struct SSPCR1_SPEC;
impl crate::RegisterSpec for SSPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspcr1::R](R) reader structure"]
impl crate::Readable for SSPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspcr1::W](W) writer structure"]
impl crate::Writable for SSPCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSPCR1 to value 0"]
impl crate::Resettable for SSPCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
