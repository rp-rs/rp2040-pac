#[doc = "Register `FC0_INTERVAL` reader"]
pub type R = crate::R<FC0_INTERVAL_SPEC>;
#[doc = "Register `FC0_INTERVAL` writer"]
pub type W = crate::W<FC0_INTERVAL_SPEC>;
#[doc = "Field `FC0_INTERVAL` reader - "]
pub type FC0_INTERVAL_R = crate::FieldReader;
#[doc = "Field `FC0_INTERVAL` writer - "]
pub type FC0_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fc0_interval(&self) -> FC0_INTERVAL_R {
        FC0_INTERVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_interval(&mut self) -> FC0_INTERVAL_W<FC0_INTERVAL_SPEC> {
        FC0_INTERVAL_W::new(self, 0)
    }
}
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval  
 The default gives a test interval of 250us  

You can [`read`](crate::Reg::read) this register and get [`fc0_interval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc0_interval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_INTERVAL_SPEC;
impl crate::RegisterSpec for FC0_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_interval::R`](R) reader structure"]
impl crate::Readable for FC0_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_interval::W`](W) writer structure"]
impl crate::Writable for FC0_INTERVAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC0_INTERVAL to value 0x08"]
impl crate::Resettable for FC0_INTERVAL_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
