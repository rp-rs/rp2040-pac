#[doc = "Register `PRIM` reader"]
pub struct R(crate::R<PRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIM` writer"]
pub struct W(crate::W<PRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIM_SPEC>;
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
impl From<crate::W<PRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSTDIV2` reader - divide by 1-7"]
pub type POSTDIV2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSTDIV2` writer - divide by 1-7"]
pub type POSTDIV2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `POSTDIV1` reader - divide by 1-7"]
pub type POSTDIV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSTDIV1` writer - divide by 1-7"]
pub type POSTDIV1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&self) -> POSTDIV2_R {
        POSTDIV2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv2(&mut self) -> POSTDIV2_W<12> {
        POSTDIV2_W::new(self)
    }
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv1(&mut self) -> POSTDIV1_W<16> {
        POSTDIV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the PLL post dividers for the primary output  
 (note: this PLL does not have a secondary output)  
 the primary output is driven from VCO divided by postdiv1*postdiv2  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [prim](index.html) module"]
pub struct PRIM_SPEC;
impl crate::RegisterSpec for PRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prim::R](R) reader structure"]
impl crate::Readable for PRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prim::W](W) writer structure"]
impl crate::Writable for PRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIM to value 0x0007_7000"]
impl crate::Resettable for PRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_7000;
}
