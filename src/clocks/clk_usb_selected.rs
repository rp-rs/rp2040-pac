#[doc = "Register `CLK_USB_SELECTED` reader"]
pub struct R(crate::R<CLK_USB_SELECTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_USB_SELECTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_USB_SELECTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_USB_SELECTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_usb_selected](index.html) module"]
pub struct CLK_USB_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_USB_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_usb_selected::R](R) reader structure"]
impl crate::Readable for CLK_USB_SELECTED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_USB_SELECTED to value 0x01"]
impl crate::Resettable for CLK_USB_SELECTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
