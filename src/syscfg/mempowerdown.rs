#[doc = "Register `MEMPOWERDOWN` reader"]
pub struct R(crate::R<MEMPOWERDOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMPOWERDOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMPOWERDOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMPOWERDOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMPOWERDOWN` writer"]
pub struct W(crate::W<MEMPOWERDOWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMPOWERDOWN_SPEC>;
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
impl From<crate::W<MEMPOWERDOWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMPOWERDOWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM0` reader - "]
pub type SRAM0_R = crate::BitReader<bool>;
#[doc = "Field `SRAM0` writer - "]
pub type SRAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `SRAM1` reader - "]
pub type SRAM1_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1` writer - "]
pub type SRAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `SRAM2` reader - "]
pub type SRAM2_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2` writer - "]
pub type SRAM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `SRAM3` reader - "]
pub type SRAM3_R = crate::BitReader<bool>;
#[doc = "Field `SRAM3` writer - "]
pub type SRAM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `SRAM4` reader - "]
pub type SRAM4_R = crate::BitReader<bool>;
#[doc = "Field `SRAM4` writer - "]
pub type SRAM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `SRAM5` reader - "]
pub type SRAM5_R = crate::BitReader<bool>;
#[doc = "Field `SRAM5` writer - "]
pub type SRAM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `USB` reader - "]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - "]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
#[doc = "Field `ROM` reader - "]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `ROM` writer - "]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMPOWERDOWN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram0(&mut self) -> SRAM0_W<0> {
        SRAM0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sram1(&mut self) -> SRAM1_W<1> {
        SRAM1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sram2(&mut self) -> SRAM2_W<2> {
        SRAM2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sram3(&mut self) -> SRAM3_W<3> {
        SRAM3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sram4(&mut self) -> SRAM4_W<4> {
        SRAM4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sram5(&mut self) -> SRAM5_W<5> {
        SRAM5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<6> {
        USB_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<7> {
        ROM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control power downs to memories. Set high to power down memories.  
 Use with extreme caution  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mempowerdown](index.html) module"]
pub struct MEMPOWERDOWN_SPEC;
impl crate::RegisterSpec for MEMPOWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mempowerdown::R](R) reader structure"]
impl crate::Readable for MEMPOWERDOWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mempowerdown::W](W) writer structure"]
impl crate::Writable for MEMPOWERDOWN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMPOWERDOWN to value 0"]
impl crate::Resettable for MEMPOWERDOWN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
