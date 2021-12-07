#[doc = "Register `IC_DATA_CMD` reader"]
pub struct R(crate::R<IC_DATA_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_DATA_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_DATA_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_DATA_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_DATA_CMD` writer"]
pub struct W(crate::W<IC_DATA_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_DATA_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IC_DATA_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_DATA_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.  

 Reset value : 0x0  

 NOTE: In case of APB_DATA_WIDTH=8,  

 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.  

 2. In order to read the 11 bit, the user has to perform the first data byte read \\[7:0\\]
(offset 0x10) and then perform the second read \\[15:8\\]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).  

 3. The 11th bit is an optional read field, user can ignore 2nd byte read \\[15:8\\]
(offset 0x11) if not interested in FIRST_DATA_BYTE status.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRST_DATA_BYTE_A {
    #[doc = "0: Sequential data byte received"]
    INACTIVE = 0,
    #[doc = "1: Non sequential data byte received"]
    ACTIVE = 1,
}
impl From<FIRST_DATA_BYTE_A> for bool {
    #[inline(always)]
    fn from(variant: FIRST_DATA_BYTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIRST_DATA_BYTE` reader - Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.  

 Reset value : 0x0  

 NOTE: In case of APB_DATA_WIDTH=8,  

 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.  

 2. In order to read the 11 bit, the user has to perform the first data byte read \\[7:0\\]
(offset 0x10) and then perform the second read \\[15:8\\]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).  

 3. The 11th bit is an optional read field, user can ignore 2nd byte read \\[15:8\\]
(offset 0x11) if not interested in FIRST_DATA_BYTE status."]
pub struct FIRST_DATA_BYTE_R(crate::FieldReader<bool, FIRST_DATA_BYTE_A>);
impl FIRST_DATA_BYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIRST_DATA_BYTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRST_DATA_BYTE_A {
        match self.bits {
            false => FIRST_DATA_BYTE_A::INACTIVE,
            true => FIRST_DATA_BYTE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == FIRST_DATA_BYTE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == FIRST_DATA_BYTE_A::ACTIVE
    }
}
impl core::ops::Deref for FIRST_DATA_BYTE_R {
    type Target = crate::FieldReader<bool, FIRST_DATA_BYTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTART_A {
    #[doc = "0: Don't Issue RESTART before this command"]
    DISABLE = 0,
    #[doc = "1: Issue RESTART before this command"]
    ENABLE = 1,
}
impl From<RESTART_A> for bool {
    #[inline(always)]
    fn from(variant: RESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESTART` reader - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
pub struct RESTART_R(crate::FieldReader<bool, RESTART_A>);
impl RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTART_A {
        match self.bits {
            false => RESTART_A::DISABLE,
            true => RESTART_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RESTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RESTART_A::ENABLE
    }
}
impl core::ops::Deref for RESTART_R {
    type Target = crate::FieldReader<bool, RESTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESTART` writer - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Don't Issue RESTART before this command"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESTART_A::DISABLE)
    }
    #[doc = "Issue RESTART before this command"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESTART_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: Don't Issue STOP after this command"]
    DISABLE = 0,
    #[doc = "1: Issue STOP after this command"]
    ENABLE = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
pub struct STOP_R(crate::FieldReader<bool, STOP_A>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::DISABLE,
            true => STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == STOP_A::ENABLE
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Don't Issue STOP after this command"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STOP_A::DISABLE)
    }
    #[doc = "Issue STOP after this command"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STOP_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_A {
    #[doc = "0: Master Write Command"]
    WRITE = 0,
    #[doc = "1: Master Read Command"]
    READ = 1,
}
impl From<CMD_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD` reader - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
pub struct CMD_R(crate::FieldReader<bool, CMD_A>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_A {
        match self.bits {
            false => CMD_A::WRITE,
            true => CMD_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == CMD_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == CMD_A::READ
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<bool, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master Write Command"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_A::WRITE)
    }
    #[doc = "Master Read Command"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_A::READ)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DAT` reader - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
pub struct DAT_R(crate::FieldReader<u8, u8>);
impl DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT` writer - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
pub struct DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.  

 Reset value : 0x0  

 NOTE: In case of APB_DATA_WIDTH=8,  

 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.  

 2. In order to read the 11 bit, the user has to perform the first data byte read \\[7:0\\]
(offset 0x10) and then perform the second read \\[15:8\\]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).  

 3. The 11th bit is an optional read field, user can ignore 2nd byte read \\[15:8\\]
(offset 0x11) if not interested in FIRST_DATA_BYTE status."]
    #[inline(always)]
    pub fn first_data_byte(&self) -> FIRST_DATA_BYTE_R {
        FIRST_DATA_BYTE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 9 - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 0:7 - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_data_cmd](index.html) module"]
pub struct IC_DATA_CMD_SPEC;
impl crate::RegisterSpec for IC_DATA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_data_cmd::R](R) reader structure"]
impl crate::Readable for IC_DATA_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_data_cmd::W](W) writer structure"]
impl crate::Writable for IC_DATA_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_DATA_CMD to value 0"]
impl crate::Resettable for IC_DATA_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
