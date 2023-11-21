#[doc = "Register `SER` reader"]
pub type R = crate::R<SER_SPEC>;
#[doc = "Register `SER` writer"]
pub type W = crate::W<SER_SPEC>;
#[doc = "Field `SER` reader - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
pub type SER_R = crate::BitReader;
#[doc = "Field `SER` writer - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
pub type SER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<SER_SPEC, 0> {
        SER_W::new(self)
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
#[doc = "Slave enable  

You can [`read`](crate::generic::Reg::read) this register and get [`ser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SER_SPEC;
impl crate::RegisterSpec for SER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser::R`](R) reader structure"]
impl crate::Readable for SER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ser::W`](W) writer structure"]
impl crate::Writable for SER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
