#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `REFDIV` reader - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
pub type REFDIV_R = crate::FieldReader;
#[doc = "Field `REFDIV` writer - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
pub type REFDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `BYPASS` reader - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
pub type BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK` reader - PLL is locked"]
pub type LOCK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    pub fn refdiv(&self) -> REFDIV_R {
        REFDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL is locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    #[must_use]
    pub fn refdiv(&mut self) -> REFDIV_W<CS_SPEC, 0> {
        REFDIV_W::new(self)
    }
    #[doc = "Bit 8 - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<CS_SPEC, 8> {
        BYPASS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control and Status  
 GENERAL CONSTRAINTS:  
 Reference clock frequency min=5MHz, max=800MHz  
 Feedback divider min=16, max=320  
 VCO frequency min=750MHz, max=1600MHz  

You can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0x01"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
