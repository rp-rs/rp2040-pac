#[doc = "Register `CLK_GPOUT0_SELECTED` reader"]
pub type R = crate::R<CLK_GPOUT0_SELECTED_SPEC>;
#[doc = "Register `CLK_GPOUT0_SELECTED` writer"]
pub type W = crate::W<CLK_GPOUT0_SELECTED_SPEC>;
#[doc = "Field `CLK_GPOUT0_SELECTED` reader - This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
pub type CLK_GPOUT0_SELECTED_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn clk_gpout0_selected(&self) -> CLK_GPOUT0_SELECTED_R {
        CLK_GPOUT0_SELECTED_R::new(self.bits)
    }
}
impl W {}
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout0_selected::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpout0_selected::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GPOUT0_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_GPOUT0_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gpout0_selected::R`](R) reader structure"]
impl crate::Readable for CLK_GPOUT0_SELECTED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_gpout0_selected::W`](W) writer structure"]
impl crate::Writable for CLK_GPOUT0_SELECTED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_GPOUT0_SELECTED to value 0x01"]
impl crate::Resettable for CLK_GPOUT0_SELECTED_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
