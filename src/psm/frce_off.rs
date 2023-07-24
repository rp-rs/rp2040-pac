#[doc = "Register `FRCE_OFF` reader"]
pub struct R(crate::R<FRCE_OFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCE_OFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCE_OFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCE_OFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRCE_OFF` writer"]
pub struct W(crate::W<FRCE_OFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCE_OFF_SPEC>;
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
impl From<crate::W<FRCE_OFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCE_OFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rosc` reader - "]
pub type ROSC_R = crate::BitReader<bool>;
#[doc = "Field `rosc` writer - "]
pub type ROSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `xosc` reader - "]
pub type XOSC_R = crate::BitReader<bool>;
#[doc = "Field `xosc` writer - "]
pub type XOSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `clocks` reader - "]
pub type CLOCKS_R = crate::BitReader<bool>;
#[doc = "Field `clocks` writer - "]
pub type CLOCKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `resets` reader - "]
pub type RESETS_R = crate::BitReader<bool>;
#[doc = "Field `resets` writer - "]
pub type RESETS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `busfabric` reader - "]
pub type BUSFABRIC_R = crate::BitReader<bool>;
#[doc = "Field `busfabric` writer - "]
pub type BUSFABRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `rom` reader - "]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `rom` writer - "]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sram0` reader - "]
pub type SRAM0_R = crate::BitReader<bool>;
#[doc = "Field `sram0` writer - "]
pub type SRAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sram1` reader - "]
pub type SRAM1_R = crate::BitReader<bool>;
#[doc = "Field `sram1` writer - "]
pub type SRAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sram2` reader - "]
pub type SRAM2_R = crate::BitReader<bool>;
#[doc = "Field `sram2` writer - "]
pub type SRAM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sram3` reader - "]
pub type SRAM3_R = crate::BitReader<bool>;
#[doc = "Field `sram3` writer - "]
pub type SRAM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sram4` reader - "]
pub type SRAM4_R = crate::BitReader<bool>;
#[doc = "Field `sram4` writer - "]
pub type SRAM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sram5` reader - "]
pub type SRAM5_R = crate::BitReader<bool>;
#[doc = "Field `sram5` writer - "]
pub type SRAM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `xip` reader - "]
pub type XIP_R = crate::BitReader<bool>;
#[doc = "Field `xip` writer - "]
pub type XIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `vreg_and_chip_reset` reader - "]
pub type VREG_AND_CHIP_RESET_R = crate::BitReader<bool>;
#[doc = "Field `vreg_and_chip_reset` writer - "]
pub type VREG_AND_CHIP_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `sio` reader - "]
pub type SIO_R = crate::BitReader<bool>;
#[doc = "Field `sio` writer - "]
pub type SIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `proc0` reader - "]
pub type PROC0_R = crate::BitReader<bool>;
#[doc = "Field `proc0` writer - "]
pub type PROC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
#[doc = "Field `proc1` reader - "]
pub type PROC1_R = crate::BitReader<bool>;
#[doc = "Field `proc1` writer - "]
pub type PROC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCE_OFF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&self) -> ROSC_R {
        ROSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&self) -> CLOCKS_R {
        CLOCKS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&self) -> RESETS_R {
        RESETS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&self) -> BUSFABRIC_R {
        BUSFABRIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&self) -> VREG_AND_CHIP_RESET_R {
        VREG_AND_CHIP_RESET_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rosc(&mut self) -> ROSC_W<0> {
        ROSC_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn xosc(&mut self) -> XOSC_W<1> {
        XOSC_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clocks(&mut self) -> CLOCKS_W<2> {
        CLOCKS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn resets(&mut self) -> RESETS_W<3> {
        RESETS_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn busfabric(&mut self) -> BUSFABRIC_W<4> {
        BUSFABRIC_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<5> {
        ROM_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sram0(&mut self) -> SRAM0_W<6> {
        SRAM0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sram1(&mut self) -> SRAM1_W<7> {
        SRAM1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sram2(&mut self) -> SRAM2_W<8> {
        SRAM2_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sram3(&mut self) -> SRAM3_W<9> {
        SRAM3_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sram4(&mut self) -> SRAM4_W<10> {
        SRAM4_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sram5(&mut self) -> SRAM5_W<11> {
        SRAM5_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xip(&mut self) -> XIP_W<12> {
        XIP_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_and_chip_reset(&mut self) -> VREG_AND_CHIP_RESET_W<13> {
        VREG_AND_CHIP_RESET_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn sio(&mut self) -> SIO_W<14> {
        SIO_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn proc0(&mut self) -> PROC0_W<15> {
        PROC0_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<16> {
        PROC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force into reset (i.e. power it off)  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [frce_off](index.html) module"]
pub struct FRCE_OFF_SPEC;
impl crate::RegisterSpec for FRCE_OFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frce_off::R](R) reader structure"]
impl crate::Readable for FRCE_OFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frce_off::W](W) writer structure"]
impl crate::Writable for FRCE_OFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRCE_OFF to value 0"]
impl crate::Resettable for FRCE_OFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
