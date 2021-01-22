#[doc = "Reader of register GPIO_HI_IN"]
pub type R = crate::R<u32, super::GPIO_HI_IN>;
#[doc = "Reader of field `GPIO_HI_IN`"]
pub type GPIO_HI_IN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Input value on QSPI IO in order 0..5: SCLK, SSn, SD0, SD1, SD2, SD3"]
    #[inline(always)]
    pub fn gpio_hi_in(&self) -> GPIO_HI_IN_R {
        GPIO_HI_IN_R::new((self.bits & 0x3f) as u8)
    }
}
