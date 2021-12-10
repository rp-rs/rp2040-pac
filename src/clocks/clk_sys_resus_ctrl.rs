#[doc = "Register `CLK_SYS_RESUS_CTRL` reader"]
pub struct R(crate::R<CLK_SYS_RESUS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SYS_RESUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SYS_RESUS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SYS_RESUS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SYS_RESUS_CTRL` writer"]
pub struct W(crate::W<CLK_SYS_RESUS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SYS_RESUS_CTRL_SPEC>;
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
impl From<crate::W<CLK_SYS_RESUS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SYS_RESUS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR` reader - For clearing the resus after the fault that triggered it has been corrected"]
pub struct CLEAR_R(crate::FieldReader<bool, bool>);
impl CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAR` writer - For clearing the resus after the fault that triggered it has been corrected"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FRCE` reader - Force a resus, for test purposes only"]
pub struct FRCE_R(crate::FieldReader<bool, bool>);
impl FRCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRCE` writer - Force a resus, for test purposes only"]
pub struct FRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Enable resus"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable resus"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TIMEOUT` reader - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
pub struct TIMEOUT_R(crate::FieldReader<u8, u8>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    pub fn frce(&self) -> FRCE_R {
        FRCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    pub fn frce(&mut self) -> FRCE_W {
        FRCE_W { w: self }
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_sys_resus_ctrl](index.html) module"]
pub struct CLK_SYS_RESUS_CTRL_SPEC;
impl crate::RegisterSpec for CLK_SYS_RESUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_sys_resus_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_sys_resus_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_SYS_RESUS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_SYS_RESUS_CTRL to value 0xff"]
impl crate::Resettable for CLK_SYS_RESUS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
