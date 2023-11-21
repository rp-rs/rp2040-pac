#[doc = "Register `CLK_GPOUT2_SELECTED` reader"]
pub type R = crate::R<CLK_GPOUT2_SELECTED_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLK_GPOUT2_SELECTED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_gpout2_selected::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GPOUT2_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_GPOUT2_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gpout2_selected::R`](R) reader structure"]
impl crate::Readable for CLK_GPOUT2_SELECTED_SPEC {}
#[doc = "`reset()` method sets CLK_GPOUT2_SELECTED to value 0x01"]
impl crate::Resettable for CLK_GPOUT2_SELECTED_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
