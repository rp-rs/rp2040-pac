#[doc = "Register `SNIFF_DATA` reader"]
pub type R = crate::R<SNIFF_DATA_SPEC>;
#[doc = "Register `SNIFF_DATA` writer"]
pub type W = crate::W<SNIFF_DATA_SPEC>;
#[doc = "Field `SNIFF_DATA` reader - Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
pub type SNIFF_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `SNIFF_DATA` writer - Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
pub type SNIFF_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    #[inline(always)]
    pub fn sniff_data(&self) -> SNIFF_DATA_R {
        SNIFF_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    #[inline(always)]
    #[must_use]
    pub fn sniff_data(&mut self) -> SNIFF_DATA_W<SNIFF_DATA_SPEC> {
        SNIFF_DATA_W::new(self, 0)
    }
}
#[doc = "Data accumulator for sniff hardware  

You can [`read`](crate::generic::Reg::read) this register and get [`sniff_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sniff_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SNIFF_DATA_SPEC;
impl crate::RegisterSpec for SNIFF_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sniff_data::R`](R) reader structure"]
impl crate::Readable for SNIFF_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sniff_data::W`](W) writer structure"]
impl crate::Writable for SNIFF_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNIFF_DATA to value 0"]
impl crate::Resettable for SNIFF_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
