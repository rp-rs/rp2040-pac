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
#[doc = "Field `EN` reader - enable  
 0=not enabled, 1=enabled"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - enable  
 0=not enabled, 1=enabled"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD_SPEC, bool, O>;
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
pub type VSEL_R = crate::FieldReader<u8, u8>;
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
pub type VSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
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
}
impl W {
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
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
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<4> {
        VSEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD to value 0x91"]
impl crate::Resettable for BOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x91;
}
