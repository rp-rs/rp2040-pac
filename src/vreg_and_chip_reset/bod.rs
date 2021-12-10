#[doc = "Register `BOD` reader"]
pub struct R(crate::R<BOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD` writer"]
pub struct W(crate::W<BOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD_SPEC>;
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
impl From<crate::W<BOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSEL` reader - threshold select  
 0000 - 0.473V  
 0001 - 0.516V  
 0010 - 0.559V  
 0011 - 0.602V  
 0100 - 0.645V  
 0101 - 0.688V  
 0110 - 0.731V  
 0111 - 0.774V  
 1000 - 0.817V  
 1001 - 0.860V (default)  
 1010 - 0.903V  
 1011 - 0.946V  
 1100 - 0.989V  
 1101 - 1.032V  
 1110 - 1.075V  
 1111 - 1.118V"]
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
#[doc = "Field `VSEL` writer - threshold select  
 0000 - 0.473V  
 0001 - 0.516V  
 0010 - 0.559V  
 0011 - 0.602V  
 0100 - 0.645V  
 0101 - 0.688V  
 0110 - 0.731V  
 0111 - 0.774V  
 1000 - 0.817V  
 1001 - 0.860V (default)  
 1010 - 0.903V  
 1011 - 0.946V  
 1100 - 0.989V  
 1101 - 1.032V  
 1110 - 1.075V  
 1111 - 1.118V"]
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
    #[doc = "Bits 4:7 - threshold select  
 0000 - 0.473V  
 0001 - 0.516V  
 0010 - 0.559V  
 0011 - 0.602V  
 0100 - 0.645V  
 0101 - 0.688V  
 0110 - 0.731V  
 0111 - 0.774V  
 1000 - 0.817V  
 1001 - 0.860V (default)  
 1010 - 0.903V  
 1011 - 0.946V  
 1100 - 0.989V  
 1101 - 1.032V  
 1110 - 1.075V  
 1111 - 1.118V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - threshold select  
 0000 - 0.473V  
 0001 - 0.516V  
 0010 - 0.559V  
 0011 - 0.602V  
 0100 - 0.645V  
 0101 - 0.688V  
 0110 - 0.731V  
 0111 - 0.774V  
 1000 - 0.817V  
 1001 - 0.860V (default)  
 1010 - 0.903V  
 1011 - 0.946V  
 1100 - 0.989V  
 1101 - 1.032V  
 1110 - 1.075V  
 1111 - 1.118V"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W {
        VSEL_W { w: self }
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
#[doc = "brown-out detection control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [bod](index.html) module"]
pub struct BOD_SPEC;
impl crate::RegisterSpec for BOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod::R](R) reader structure"]
impl crate::Readable for BOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod::W](W) writer structure"]
impl crate::Writable for BOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD to value 0x91"]
impl crate::Resettable for BOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x91
    }
}
