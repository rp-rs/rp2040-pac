#[doc = "Register `TXD_DRIVE_EDGE` reader"]
pub type R = crate::R<TXD_DRIVE_EDGE_SPEC>;
#[doc = "Register `TXD_DRIVE_EDGE` writer"]
pub type W = crate::W<TXD_DRIVE_EDGE_SPEC>;
#[doc = "Field `TDE` reader - TXD drive edge"]
pub type TDE_R = crate::FieldReader;
#[doc = "Field `TDE` writer - TXD drive edge"]
pub type TDE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TXD drive edge"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXD drive edge"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<TXD_DRIVE_EDGE_SPEC> {
        TDE_W::new(self, 0)
    }
}
#[doc = "TX drive edge  

You can [`read`](crate::generic::Reg::read) this register and get [`txd_drive_edge::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd_drive_edge::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXD_DRIVE_EDGE_SPEC;
impl crate::RegisterSpec for TXD_DRIVE_EDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txd_drive_edge::R`](R) reader structure"]
impl crate::Readable for TXD_DRIVE_EDGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txd_drive_edge::W`](W) writer structure"]
impl crate::Writable for TXD_DRIVE_EDGE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXD_DRIVE_EDGE to value 0"]
impl crate::Resettable for TXD_DRIVE_EDGE_SPEC {
    const RESET_VALUE: u32 = 0;
}
