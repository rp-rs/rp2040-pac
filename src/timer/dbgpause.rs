#[doc = "Register `DBGPAUSE` reader"]
pub struct R(crate::R<DBGPAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGPAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGPAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGPAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGPAUSE` writer"]
pub struct W(crate::W<DBGPAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGPAUSE_SPEC>;
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
impl From<crate::W<DBGPAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGPAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG0` reader - Pause when processor 0 is in debug mode"]
pub type DBG0_R = crate::BitReader<bool>;
#[doc = "Field `DBG0` writer - Pause when processor 0 is in debug mode"]
pub type DBG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGPAUSE_SPEC, bool, O>;
#[doc = "Field `DBG1` reader - Pause when processor 1 is in debug mode"]
pub type DBG1_R = crate::BitReader<bool>;
#[doc = "Field `DBG1` writer - Pause when processor 1 is in debug mode"]
pub type DBG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGPAUSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn dbg0(&self) -> DBG0_R {
        DBG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn dbg1(&self) -> DBG1_R {
        DBG1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Pause when processor 0 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg0(&mut self) -> DBG0_W<1> {
        DBG0_W::new(self)
    }
    #[doc = "Bit 2 - Pause when processor 1 is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg1(&mut self) -> DBG1_W<2> {
        DBG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bits high to enable pause when the corresponding debug ports are active  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dbgpause](index.html) module"]
pub struct DBGPAUSE_SPEC;
impl crate::RegisterSpec for DBGPAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgpause::R](R) reader structure"]
impl crate::Readable for DBGPAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgpause::W](W) writer structure"]
impl crate::Writable for DBGPAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGPAUSE to value 0x07"]
impl crate::Resettable for DBGPAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
