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
#[doc = "Field `POSTDIV1` reader - divide by 1-7"]
pub struct POSTDIV1_R(crate::FieldReader<u8, u8>);
impl POSTDIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POSTDIV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSTDIV1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSTDIV1` writer - divide by 1-7"]
pub struct POSTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `POSTDIV2` reader - divide by 1-7"]
pub struct POSTDIV2_R(crate::FieldReader<u8, u8>);
impl POSTDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POSTDIV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSTDIV2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSTDIV2` writer - divide by 1-7"]
pub struct POSTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&self) -> POSTDIV2_R {
        POSTDIV2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&mut self) -> POSTDIV1_W {
        POSTDIV1_W { w: self }
    }
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&mut self) -> POSTDIV2_W {
        POSTDIV2_W { w: self }
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
}
#[doc = "`reset()` method sets PRIM to value 0x0007_7000"]
impl crate::Resettable for PRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_7000
    }
}
