#[doc = "Register `IC_SDA_HOLD` reader"]
pub type R = crate::R<IC_SDA_HOLD_SPEC>;
#[doc = "Register `IC_SDA_HOLD` writer"]
pub type W = crate::W<IC_SDA_HOLD_SPEC>;
#[doc = "Field `IC_SDA_TX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
pub type IC_SDA_TX_HOLD_R = crate::FieldReader<u16>;
#[doc = "Field `IC_SDA_TX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
pub type IC_SDA_TX_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IC_SDA_RX_HOLD` reader - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
pub type IC_SDA_RX_HOLD_R = crate::FieldReader;
#[doc = "Field `IC_SDA_RX_HOLD` writer - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
pub type IC_SDA_RX_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
    #[inline(always)]
    pub fn ic_sda_tx_hold(&self) -> IC_SDA_TX_HOLD_R {
        IC_SDA_TX_HOLD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
    #[inline(always)]
    pub fn ic_sda_rx_hold(&self) -> IC_SDA_RX_HOLD_R {
        IC_SDA_RX_HOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[15:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sda_tx_hold(&mut self) -> IC_SDA_TX_HOLD_W<IC_SDA_HOLD_SPEC> {
        IC_SDA_TX_HOLD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver.  

 Reset value: IC_DEFAULT_SDA_HOLD\\[23:16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sda_rx_hold(&mut self) -> IC_SDA_RX_HOLD_W<IC_SDA_HOLD_SPEC> {
        IC_SDA_RX_HOLD_W::new(self, 16)
    }
}
#[doc = "I2C SDA Hold Time Length Register  

 The bits \\[15:0\\]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW).  

 The bits \\[23:16\\]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]=0.  

 The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode (one cycle in master mode, seven cycles in slave mode) for the value to be implemented.  

 The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles.  

You can [`read`](crate::Reg::read) this register and get [`ic_sda_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_SDA_HOLD_SPEC;
impl crate::RegisterSpec for IC_SDA_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sda_hold::R`](R) reader structure"]
impl crate::Readable for IC_SDA_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_sda_hold::W`](W) writer structure"]
impl crate::Writable for IC_SDA_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_SDA_HOLD to value 0x01"]
impl crate::Resettable for IC_SDA_HOLD_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
