#[doc = "Register `VREG` reader"]
pub struct R(crate::R<VREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREG` writer"]
pub struct W(crate::W<VREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREG_SPEC>;
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
impl From<crate::W<VREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROK` reader - regulation status  
 0=not in regulation, 1=in regulation"]
pub struct ROK_R(crate::FieldReader<bool, bool>);
impl ROK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSEL` reader - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
pub struct VSEL_R(crate::FieldReader<u8, u8>);
impl VSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSEL` writer - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
pub struct VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `HIZ` reader - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
pub struct HIZ_R(crate::FieldReader<bool, bool>);
impl HIZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIZ` writer - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
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
#[doc = "Field `EN` reader - enable  
 0=not enabled, 1=enabled"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - enable  
 0=not enabled, 1=enabled"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 12 - regulation status  
 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W {
        VSEL_W { w: self }
    }
    #[doc = "Bit 1 - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage regulator control and status  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [vreg](index.html) module"]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreg::R](R) reader structure"]
impl crate::Readable for VREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreg::W](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREG to value 0xb1"]
impl crate::Resettable for VREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
