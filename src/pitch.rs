use std::fmt;
use std::default::Default;

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct AuctionType(pub u8);
impl AuctionType {
  pub const OPENING_AUCTION : u8 = 'O' as u8;
  pub const CLOSING_AUCTION : u8 = 'C' as u8;
  pub const HALT_AUCTION : u8 = 'H' as u8;
  pub const IPO_AUCTION : u8 = 'I' as u8;
  pub const CBOE_MARKET_CLOSE : u8 = 'M' as u8;
  pub const GTH_OPENING : u8 = 'G' as u8;
  pub const VOLATILITY_OPENING : u8 = 'V' as u8;
  pub const BATS_AUCTION_MECHANISM : u8 = 'B' as u8;
  pub const SOLICITATION_AUCTION_MECHANISM : u8 = 'S' as u8;
  pub const STEP_UP_MECHANISM : u8 = 'T' as u8;
  pub const SUM_ALL_OR_NONE : u8 = 'A' as u8;
} // AuctionType

impl fmt::Display for AuctionType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      AuctionType::OPENING_AUCTION => "'O' (opening auction)".to_string(),
      AuctionType::CLOSING_AUCTION => "'C' (closing auction)".to_string(),
      AuctionType::HALT_AUCTION => "'H' (halt auction)".to_string(),
      AuctionType::IPO_AUCTION => "'I' (ipo auction)".to_string(),
      AuctionType::CBOE_MARKET_CLOSE => "'M' (cboe market close)".to_string(),
      AuctionType::GTH_OPENING => "'G' (gth opening)".to_string(),
      AuctionType::VOLATILITY_OPENING => "'V' (volatility opening)".to_string(),
      AuctionType::BATS_AUCTION_MECHANISM => "'B' (bats auction mechanism)".to_string(),
      AuctionType::SOLICITATION_AUCTION_MECHANISM => "'S' (solicitation auction mechanism)".to_string(),
      AuctionType::STEP_UP_MECHANISM => "'T' (step up mechanism)".to_string(),
      AuctionType::SUM_ALL_OR_NONE => "'A' (sum all or none)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for AuctionType

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct CustomerIndicator(pub u8);
impl CustomerIndicator {
  pub const UNUSED : u8 = ' ' as u8;
  pub const NON_CUSTOMER : u8 = 'N' as u8;
  pub const CUSTOMER : u8 = 'C' as u8;
  pub const RETAIL_ORDER : u8 = 'R' as u8;
} // CustomerIndicator

impl fmt::Display for CustomerIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      CustomerIndicator::UNUSED => "' ' (unused)".to_string(),
      CustomerIndicator::NON_CUSTOMER => "'N' (non customer)".to_string(),
      CustomerIndicator::CUSTOMER => "'C' (customer)".to_string(),
      CustomerIndicator::RETAIL_ORDER => "'R' (retail order)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for CustomerIndicator

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct GapResponseStatus(pub u8);
impl GapResponseStatus {
  pub const ACCEPTED : u8 = 'A' as u8;
  pub const OUT_OF_RANGE : u8 = 'O' as u8;
  pub const DAILY_GAP_REQUEST_ALLOCATION_EXHAUSTED : u8 = 'D' as u8;
  pub const MINUTE_GAP_REQUEST_ALLOCATION_EXHAUSTED : u8 = 'M' as u8;
  pub const SECOND_GAP_REQUEST_ALLOCATION_EXHAUSTED : u8 = 'S' as u8;
  pub const COUNT_REQUEST_LIMIT_FOR_ONE_GAP_REQUEST_EXCEEDED : u8 = 'C' as u8;
  pub const INVALID_UNIT_SPECIFIED_IN_REQUEST : u8 = 'I' as u8;
  pub const UNIT_IS_CURRENTLY_UNAVAILABLE : u8 = 'U' as u8;
} // GapResponseStatus

impl fmt::Display for GapResponseStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      GapResponseStatus::ACCEPTED => "'A' (accepted)".to_string(),
      GapResponseStatus::OUT_OF_RANGE => "'O' (out of range)".to_string(),
      GapResponseStatus::DAILY_GAP_REQUEST_ALLOCATION_EXHAUSTED => "'D' (daily gap request allocation exhausted)".to_string(),
      GapResponseStatus::MINUTE_GAP_REQUEST_ALLOCATION_EXHAUSTED => "'M' (minute gap request allocation exhausted)".to_string(),
      GapResponseStatus::SECOND_GAP_REQUEST_ALLOCATION_EXHAUSTED => "'S' (second gap request allocation exhausted)".to_string(),
      GapResponseStatus::COUNT_REQUEST_LIMIT_FOR_ONE_GAP_REQUEST_EXCEEDED => "'C' (count request limit for one gap request exceeded)".to_string(),
      GapResponseStatus::INVALID_UNIT_SPECIFIED_IN_REQUEST => "'I' (invalid unit specified in request)".to_string(),
      GapResponseStatus::UNIT_IS_CURRENTLY_UNAVAILABLE => "'U' (unit is currently unavailable)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for GapResponseStatus

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct GthTradingStatus(pub u8);
impl GthTradingStatus {
  pub const UNUSED : u8 = ' ' as u8;
  pub const HALTED : u8 = 'H' as u8;
  pub const QUOTE_ONLY : u8 = 'Q' as u8;
  pub const TRADING : u8 = 'T' as u8;
} // GthTradingStatus

impl fmt::Display for GthTradingStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      GthTradingStatus::UNUSED => "' ' (unused)".to_string(),
      GthTradingStatus::HALTED => "'H' (halted)".to_string(),
      GthTradingStatus::QUOTE_ONLY => "'Q' (quote only)".to_string(),
      GthTradingStatus::TRADING => "'T' (trading)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for GthTradingStatus

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct InstrumentDefinitionResponseStatus(pub u8);
impl InstrumentDefinitionResponseStatus {
  pub const ACCEPTED : u8 = 'A' as u8;
  pub const OUT_OF_RANGE : u8 = 'O' as u8;
  pub const SPIN_ALREADY_IN_PROGRESS : u8 = 'S' as u8;
} // InstrumentDefinitionResponseStatus

impl fmt::Display for InstrumentDefinitionResponseStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      InstrumentDefinitionResponseStatus::ACCEPTED => "'A' (accepted)".to_string(),
      InstrumentDefinitionResponseStatus::OUT_OF_RANGE => "'O' (out of range)".to_string(),
      InstrumentDefinitionResponseStatus::SPIN_ALREADY_IN_PROGRESS => "'S' (spin already in progress)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for InstrumentDefinitionResponseStatus

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct LoginResponseStatus(pub u8);
impl LoginResponseStatus {
  pub const LOGIN_ACCEPTED : u8 = 'A' as u8;
  pub const NOT_AUTHORIZED : u8 = 'N' as u8;
  pub const SESSION_IN_USE : u8 = 'B' as u8;
  pub const INVALID_SESSION : u8 = 'S' as u8;
} // LoginResponseStatus

impl fmt::Display for LoginResponseStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      LoginResponseStatus::LOGIN_ACCEPTED => "'A' (login accepted)".to_string(),
      LoginResponseStatus::NOT_AUTHORIZED => "'N' (not authorized)".to_string(),
      LoginResponseStatus::SESSION_IN_USE => "'B' (session in use)".to_string(),
      LoginResponseStatus::INVALID_SESSION => "'S' (invalid session)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for LoginResponseStatus

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct OpeningCondition(pub u8);
impl OpeningCondition {
  pub const WOULD_OPEN : u8 = 'O' as u8;
  pub const NEED_QUOTE_TO_OPEN : u8 = 'Q' as u8;
  pub const NEED_MORE_BUYERS : u8 = 'B' as u8;
  pub const NEED_MORE_SELLERS : u8 = 'S' as u8;
  pub const CROSSED_COMPOSITE_MARKET : u8 = 'C' as u8;
} // OpeningCondition

impl fmt::Display for OpeningCondition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      OpeningCondition::WOULD_OPEN => "'O' (would open)".to_string(),
      OpeningCondition::NEED_QUOTE_TO_OPEN => "'Q' (need quote to open)".to_string(),
      OpeningCondition::NEED_MORE_BUYERS => "'B' (need more buyers)".to_string(),
      OpeningCondition::NEED_MORE_SELLERS => "'S' (need more sellers)".to_string(),
      OpeningCondition::CROSSED_COMPOSITE_MARKET => "'C' (crossed composite market)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for OpeningCondition

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct RegShoAction(pub u8);
impl RegShoAction {
  pub const NO_PRICE_TEST_IN_EFFECT : u8 = '0' as u8;
  pub const REG_SHO_PRICE_TEST_RESTRICTION_IN_EFFECT : u8 = '1' as u8;
} // RegShoAction

impl fmt::Display for RegShoAction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      RegShoAction::NO_PRICE_TEST_IN_EFFECT => "'0' (no price test in effect)".to_string(),
      RegShoAction::REG_SHO_PRICE_TEST_RESTRICTION_IN_EFFECT => "'1' (reg sho price test restriction in effect)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for RegShoAction

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct RetailPriceImprovementKind(pub u8);
impl RetailPriceImprovementKind {
  pub const BUY_SIDE_RPI : u8 = 'B' as u8;
  pub const SELL_SIDE_RPI : u8 = 'S' as u8;
  pub const BUY_AND_SELL_RPI : u8 = 'A' as u8;
  pub const NO_RPI : u8 = 'N' as u8;
} // RetailPriceImprovementKind

impl fmt::Display for RetailPriceImprovementKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      RetailPriceImprovementKind::BUY_SIDE_RPI => "'B' (buy side rpi)".to_string(),
      RetailPriceImprovementKind::SELL_SIDE_RPI => "'S' (sell side rpi)".to_string(),
      RetailPriceImprovementKind::BUY_AND_SELL_RPI => "'A' (buy and sell rpi)".to_string(),
      RetailPriceImprovementKind::NO_RPI => "'N' (no rpi)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for RetailPriceImprovementKind

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct Side(pub u8);
impl Side {
  pub const BUY_ORDER : u8 = 'B' as u8;
  pub const SELL_ORDER : u8 = 'S' as u8;
} // Side

impl fmt::Display for Side {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      Side::BUY_ORDER => "'B' (buy order)".to_string(),
      Side::SELL_ORDER => "'S' (sell order)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for Side

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct SpinResponseStatus(pub u8);
impl SpinResponseStatus {
  pub const ACCEPTED : u8 = 'A' as u8;
  pub const OUT_OF_RANGE : u8 = 'O' as u8;
  pub const SPIN_ALREADY_IN_PROGRESS : u8 = 'S' as u8;
} // SpinResponseStatus

impl fmt::Display for SpinResponseStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      SpinResponseStatus::ACCEPTED => "'A' (accepted)".to_string(),
      SpinResponseStatus::OUT_OF_RANGE => "'O' (out of range)".to_string(),
      SpinResponseStatus::SPIN_ALREADY_IN_PROGRESS => "'S' (spin already in progress)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for SpinResponseStatus

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct SymbolCondition(pub u8);
impl SymbolCondition {
  pub const NORMAL : u8 = 'N' as u8;
  pub const CLOSING_ONLY : u8 = 'C' as u8;
} // SymbolCondition

impl fmt::Display for SymbolCondition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      SymbolCondition::NORMAL => "'N' (normal)".to_string(),
      SymbolCondition::CLOSING_ONLY => "'C' (closing only)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for SymbolCondition

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct TradeCondition(pub u8);
impl TradeCondition {
  pub const NORMAL_TRADE : u8 = ' ' as u8;
  pub const SINGLE_LEG_AUCTION_NON_ISO : u8 = 'a' as u8;
  pub const SINGLE_LEG_AUCTION_ISO : u8 = 'b' as u8;
  pub const SINGLE_LEG_CROSS_NON_ISO : u8 = 'c' as u8;
  pub const SINGLE_LEG_CROSS_ISO : u8 = 'd' as u8;
  pub const SINGLE_LEG_FLOOR_TRADE : u8 = 'e' as u8;
  pub const COMPLEX_TO_COMPLEX_ELECTRONIC_TRADE : u8 = 'f' as u8;
  pub const COMPLEX_AUCTION_TRADE : u8 = 'g' as u8;
  pub const COMPLEX_CROSS : u8 = 'h' as u8;
  pub const COMPLEX_FLOOR_TRADE : u8 = 'i' as u8;
  pub const COMPLEX_ELECTRONIC_TRADE_AGAINST_SINGLE_LEG : u8 = 'j' as u8;
  pub const COMPLEX_WITH_STOCK_OPTIONS_AUCTION_TRADE : u8 = 'k' as u8;
  pub const COMPLEX_AUCTION_AGAINST_SINGLE_LEGS : u8 = 'l' as u8;
  pub const COMPLEX_FLOOR_TRADE_AGAINST_SINGLE_LEG : u8 = 'm' as u8;
  pub const COMPLEX_WITH_STOCK_ELECTRONIC_TRADE : u8 = 'n' as u8;
  pub const COMPLEX_WITH_STOCK_CROSS : u8 = 'o' as u8;
  pub const COMPLEX_WITH_STOCK_FLOOR_TRADE : u8 = 'p' as u8;
  pub const COMPLEX_FLOOR_TRADE_OF_PROPRIETARY_PRODUCTS : u8 = 't' as u8;
  pub const OPENING_TRADE : u8 = 'O' as u8;
  pub const ISO_TRADE_OR_SPREAD_TRADE : u8 = 'S' as u8;
  pub const SPIM_TRADE : u8 = 'A' as u8;
  pub const ELECTRONIC_TRADE_OR_ISO : u8 = 'I' as u8;
  pub const COMBO_TRADE : u8 = 'C' as u8;
  pub const CABINET_TRADE : u8 = 'K' as u8;
} // TradeCondition

impl fmt::Display for TradeCondition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      TradeCondition::NORMAL_TRADE => "' ' (normal trade)".to_string(),
      TradeCondition::SINGLE_LEG_AUCTION_NON_ISO => "'a' (single leg auction non iso)".to_string(),
      TradeCondition::SINGLE_LEG_AUCTION_ISO => "'b' (single leg auction iso)".to_string(),
      TradeCondition::SINGLE_LEG_CROSS_NON_ISO => "'c' (single leg cross non iso)".to_string(),
      TradeCondition::SINGLE_LEG_CROSS_ISO => "'d' (single leg cross iso)".to_string(),
      TradeCondition::SINGLE_LEG_FLOOR_TRADE => "'e' (single leg floor trade)".to_string(),
      TradeCondition::COMPLEX_TO_COMPLEX_ELECTRONIC_TRADE => "'f' (complex to complex electronic trade)".to_string(),
      TradeCondition::COMPLEX_AUCTION_TRADE => "'g' (complex auction trade)".to_string(),
      TradeCondition::COMPLEX_CROSS => "'h' (complex cross)".to_string(),
      TradeCondition::COMPLEX_FLOOR_TRADE => "'i' (complex floor trade)".to_string(),
      TradeCondition::COMPLEX_ELECTRONIC_TRADE_AGAINST_SINGLE_LEG => "'j' (complex electronic trade against single leg)".to_string(),
      TradeCondition::COMPLEX_WITH_STOCK_OPTIONS_AUCTION_TRADE => "'k' (complex with stock options auction trade)".to_string(),
      TradeCondition::COMPLEX_AUCTION_AGAINST_SINGLE_LEGS => "'l' (complex auction against single legs)".to_string(),
      TradeCondition::COMPLEX_FLOOR_TRADE_AGAINST_SINGLE_LEG => "'m' (complex floor trade against single leg)".to_string(),
      TradeCondition::COMPLEX_WITH_STOCK_ELECTRONIC_TRADE => "'n' (complex with stock electronic trade)".to_string(),
      TradeCondition::COMPLEX_WITH_STOCK_CROSS => "'o' (complex with stock cross)".to_string(),
      TradeCondition::COMPLEX_WITH_STOCK_FLOOR_TRADE => "'p' (complex with stock floor trade)".to_string(),
      TradeCondition::COMPLEX_FLOOR_TRADE_OF_PROPRIETARY_PRODUCTS => "'t' (complex floor trade of proprietary products)".to_string(),
      TradeCondition::OPENING_TRADE => "'O' (opening trade)".to_string(),
      TradeCondition::ISO_TRADE_OR_SPREAD_TRADE => "'S' (iso trade or spread trade)".to_string(),
      TradeCondition::SPIM_TRADE => "'A' (spim trade)".to_string(),
      TradeCondition::ELECTRONIC_TRADE_OR_ISO => "'I' (electronic trade or iso)".to_string(),
      TradeCondition::COMBO_TRADE => "'C' (combo trade)".to_string(),
      TradeCondition::CABINET_TRADE => "'K' (cabinet trade)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for TradeCondition

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct TradingStatusKind(pub u8);
impl TradingStatusKind {
  pub const ACCEPTING_ORDERS_FOR_QUEUING : u8 = 'A' as u8;
  pub const HALTED : u8 = 'H' as u8;
  pub const QUOTE_ONLY : u8 = 'Q' as u8;
  pub const EXCHANGE_SPECIFIC_SUSPENSION : u8 = 'S' as u8;
  pub const TRADING : u8 = 'T' as u8;
  pub const OPENING_ROTATION : u8 = 'R' as u8;
} // TradingStatusKind

impl fmt::Display for TradingStatusKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      TradingStatusKind::ACCEPTING_ORDERS_FOR_QUEUING => "'A' (accepting orders for queuing)".to_string(),
      TradingStatusKind::HALTED => "'H' (halted)".to_string(),
      TradingStatusKind::QUOTE_ONLY => "'Q' (quote only)".to_string(),
      TradingStatusKind::EXCHANGE_SPECIFIC_SUSPENSION => "'S' (exchange specific suspension)".to_string(),
      TradingStatusKind::TRADING => "'T' (trading)".to_string(),
      TradingStatusKind::OPENING_ROTATION => "'R' (opening rotation)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for TradingStatusKind

#[derive(PartialEq, Default, Clone, Copy, Debug)]
pub struct WidthType(pub u8);
impl WidthType {
  pub const REGULAR : u8 = 'R' as u8;
  pub const VOLATILITY : u8 = 'V' as u8;
} // WidthType

impl fmt::Display for WidthType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      WidthType::REGULAR => "'R' (regular)".to_string(),
      WidthType::VOLATILITY => "'V' (volatility)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
} // fmt::Display for WidthType

// Bitfields
#[derive(Default, Clone)]
pub struct AddFlagsByte1(pub u8);
impl AddFlagsByte1 {
  pub const DISPLAY: u8 = 1;
  pub fn get_display(&self) -> bool { self.0 & AddFlagsByte1::DISPLAY == AddFlagsByte1::DISPLAY }
  pub fn set_has_display(&mut self) { self.0 |= AddFlagsByte1::DISPLAY; }
  pub const AON: u8 = 8;
  pub fn get_aon(&self) -> bool { self.0 & AddFlagsByte1::AON == AddFlagsByte1::AON }
  pub fn set_has_aon(&mut self) { self.0 |= AddFlagsByte1::AON; }
}

#[derive(Default, Clone)]
pub struct AddFlagsT {
  pub byte1 : AddFlagsByte1
} // AddFlagsT
impl AddFlagsT {
  pub fn get_display(&self) -> bool { self.byte1.get_display() }
  pub fn set_has_display(&mut self) { self.byte1.set_has_display() }
  pub fn get_aon(&self) -> bool { self.byte1.get_aon() }
  pub fn set_has_aon(&mut self) { self.byte1.set_has_aon() }
} // impl AddFlagsT

impl fmt::Display for AddFlagsT {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#02x}", self.byte1.0)
  }
} // impl fmt::Display for AddFlagsT

#[derive(Default, Clone)]
pub struct ModifyFlagsByte1(pub u8);
impl ModifyFlagsByte1 {
  pub const DISPLAY: u8 = 1;
  pub fn get_display(&self) -> bool { self.0 & ModifyFlagsByte1::DISPLAY == ModifyFlagsByte1::DISPLAY }
  pub fn set_has_display(&mut self) { self.0 |= ModifyFlagsByte1::DISPLAY; }
  pub const MAINTAINPRIORITY: u8 = 2;
  pub fn get_maintainpriority(&self) -> bool { self.0 & ModifyFlagsByte1::MAINTAINPRIORITY == ModifyFlagsByte1::MAINTAINPRIORITY }
  pub fn set_has_maintainpriority(&mut self) { self.0 |= ModifyFlagsByte1::MAINTAINPRIORITY; }
}

#[derive(Default, Clone)]
pub struct ModifyFlagsT {
  pub byte1 : ModifyFlagsByte1
} // ModifyFlagsT
impl ModifyFlagsT {
  pub fn get_display(&self) -> bool { self.byte1.get_display() }
  pub fn set_has_display(&mut self) { self.byte1.set_has_display() }
  pub fn get_maintainpriority(&self) -> bool { self.byte1.get_maintainpriority() }
  pub fn set_has_maintainpriority(&mut self) { self.byte1.set_has_maintainpriority() }
} // impl ModifyFlagsT

impl fmt::Display for ModifyFlagsT {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#02x}", self.byte1.0)
  }
} // impl fmt::Display for ModifyFlagsT

// Structs
#[derive(Clone, Default)]
pub struct AddOrderExpanded<'a> {
  data: &'a [u8],
} // AddOrderExpanded

impl<'a> AddOrderExpanded<'a> {
  pub const TYPE : u8 = 0x2F as u8;
  pub fn new(data: &'a [u8]) -> AddOrderExpanded<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn side_indicator(&self) -> Side {
    Side(self.data[14])
  }
  pub fn quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[15..19].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 8] {
    self.data[19..27].try_into().unwrap()
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[27..35].try_into().unwrap())
  }
  pub fn add_flags(&self) -> AddFlagsT {
    AddFlagsT{ byte1: AddFlagsByte1(self.data[35]) }
  }
  pub fn participantid(&self) -> [u8; 4] {
    self.data[36..40].try_into().unwrap()
  }
  pub fn customer_indicator(&self) -> CustomerIndicator {
    CustomerIndicator(self.data[40])
  }
  pub fn client_id(&self) -> [u8; 4] {
    self.data[41..45].try_into().unwrap()
  }

} // impl AddOrderExpanded

impl<'a> fmt::Display for AddOrderExpanded<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AddOrderExpanded(length:{},message_type:{},time_offset:{},order_id:{},side_indicator:{},quantity:{},symbol:{},price:{},add_flags:{},participantid:{},customer_indicator:{},client_id:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.side_indicator(), self.quantity(), String::from_utf8_lossy(&self.symbol()[..]), self.price(), self.add_flags(), String::from_utf8_lossy(&self.participantid()[..]), self.customer_indicator(), String::from_utf8_lossy(&self.client_id()[..]))
  }
}

#[derive(Clone, Default)]
pub struct AddOrderLong<'a> {
  data: &'a [u8],
} // AddOrderLong

impl<'a> AddOrderLong<'a> {
  pub const TYPE : u8 = 0x21 as u8;
  pub fn new(data: &'a [u8]) -> AddOrderLong<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn side_indicator(&self) -> Side {
    Side(self.data[14])
  }
  pub fn quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[15..19].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 6] {
    self.data[19..25].try_into().unwrap()
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[25..33].try_into().unwrap())
  }
  pub fn add_flags(&self) -> AddFlagsT {
    AddFlagsT{ byte1: AddFlagsByte1(self.data[33]) }
  }

} // impl AddOrderLong

impl<'a> fmt::Display for AddOrderLong<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AddOrderLong(length:{},message_type:{},time_offset:{},order_id:{},side_indicator:{},quantity:{},symbol:{},price:{},add_flags:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.side_indicator(), self.quantity(), String::from_utf8_lossy(&self.symbol()[..]), self.price(), self.add_flags())
  }
}

#[derive(Clone, Default)]
pub struct AddOrderShort<'a> {
  data: &'a [u8],
} // AddOrderShort

impl<'a> AddOrderShort<'a> {
  pub const TYPE : u8 = 0x22 as u8;
  pub fn new(data: &'a [u8]) -> AddOrderShort<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn side_indicator(&self) -> Side {
    Side(self.data[14])
  }
  pub fn quantity(&self) -> u16 {
    u16::from_le_bytes(self.data[15..17].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 6] {
    self.data[17..23].try_into().unwrap()
  }
  pub fn price(&self) -> u16 {
    u16::from_le_bytes(self.data[23..25].try_into().unwrap())
  }
  pub fn add_flags(&self) -> AddFlagsT {
    AddFlagsT{ byte1: AddFlagsByte1(self.data[25]) }
  }

} // impl AddOrderShort

impl<'a> fmt::Display for AddOrderShort<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AddOrderShort(length:{},message_type:{},time_offset:{},order_id:{},side_indicator:{},quantity:{},symbol:{},price:{},add_flags:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.side_indicator(), self.quantity(), String::from_utf8_lossy(&self.symbol()[..]), self.price(), self.add_flags())
  }
}

#[derive(Clone, Default)]
pub struct AuctionCancel<'a> {
  data: &'a [u8],
} // AuctionCancel

impl<'a> AuctionCancel<'a> {
  pub const TYPE : u8 = 0xAE as u8;
  pub fn new(data: &'a [u8]) -> AuctionCancel<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn auction_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }

} // impl AuctionCancel

impl<'a> fmt::Display for AuctionCancel<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AuctionCancel(length:{},message_type:{},time_offset:{},auction_id:{})", self.length(), self.message_type(), self.time_offset(), self.auction_id())
  }
}

#[derive(Clone, Default)]
pub struct AuctionNotification<'a> {
  data: &'a [u8],
} // AuctionNotification

impl<'a> AuctionNotification<'a> {
  pub const TYPE : u8 = 0xAD as u8;
  pub fn new(data: &'a [u8]) -> AuctionNotification<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 6] {
    self.data[6..12].try_into().unwrap()
  }
  pub fn auction_id(&self) -> u64 {
    u64::from_le_bytes(self.data[12..20].try_into().unwrap())
  }
  pub fn auction_type(&self) -> AuctionType {
    AuctionType(self.data[20])
  }
  pub fn side(&self) -> Side {
    Side(self.data[21])
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[22..30].try_into().unwrap())
  }
  pub fn contracts(&self) -> u32 {
    u32::from_le_bytes(self.data[30..34].try_into().unwrap())
  }
  pub fn customer_indicator(&self) -> CustomerIndicator {
    CustomerIndicator(self.data[34])
  }
  pub fn participantid(&self) -> [u8; 4] {
    self.data[35..39].try_into().unwrap()
  }
  pub fn auction_end_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[39..43].try_into().unwrap())
  }
  pub fn client_id(&self) -> [u8; 4] {
    self.data[43..47].try_into().unwrap()
  }

} // impl AuctionNotification

impl<'a> fmt::Display for AuctionNotification<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AuctionNotification(length:{},message_type:{},time_offset:{},symbol:{},auction_id:{},auction_type:{},side:{},price:{},contracts:{},customer_indicator:{},participantid:{},auction_end_offset:{},client_id:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.symbol()[..]), self.auction_id(), self.auction_type(), self.side(), self.price(), self.contracts(), self.customer_indicator(), String::from_utf8_lossy(&self.participantid()[..]), self.auction_end_offset(), String::from_utf8_lossy(&self.client_id()[..]))
  }
}

#[derive(Clone, Default)]
pub struct AuctionSummary<'a> {
  data: &'a [u8],
} // AuctionSummary

impl<'a> AuctionSummary<'a> {
  pub const TYPE : u8 = 0x96 as u8;
  pub fn new(data: &'a [u8]) -> AuctionSummary<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn stock_symbol(&self) -> [u8; 8] {
    self.data[6..14].try_into().unwrap()
  }
  pub fn auction_type(&self) -> AuctionType {
    AuctionType(self.data[14])
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[15..23].try_into().unwrap())
  }
  pub fn shares_per_contracts(&self) -> u32 {
    u32::from_le_bytes(self.data[23..27].try_into().unwrap())
  }

} // impl AuctionSummary

impl<'a> fmt::Display for AuctionSummary<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AuctionSummary(length:{},message_type:{},time_offset:{},stock_symbol:{},auction_type:{},price:{},shares_per_contracts:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.stock_symbol()[..]), self.auction_type(), self.price(), self.shares_per_contracts())
  }
}

#[derive(Clone, Default)]
pub struct AuctionTrade<'a> {
  data: &'a [u8],
} // AuctionTrade

impl<'a> AuctionTrade<'a> {
  pub const TYPE : u8 = 0xAF as u8;
  pub fn new(data: &'a [u8]) -> AuctionTrade<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn auction_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[14..22].try_into().unwrap())
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[22..30].try_into().unwrap())
  }
  pub fn contracts(&self) -> u32 {
    u32::from_le_bytes(self.data[30..34].try_into().unwrap())
  }

} // impl AuctionTrade

impl<'a> fmt::Display for AuctionTrade<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AuctionTrade(length:{},message_type:{},time_offset:{},auction_id:{},execution_id:{},price:{},contracts:{})", self.length(), self.message_type(), self.time_offset(), self.auction_id(), self.execution_id(), self.price(), self.contracts())
  }
}

#[derive(Clone, Default)]
pub struct AuctionUpdate<'a> {
  data: &'a [u8],
} // AuctionUpdate

impl<'a> AuctionUpdate<'a> {
  pub const TYPE : u8 = 0x95 as u8;
  pub fn new(data: &'a [u8]) -> AuctionUpdate<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn stock_symbol(&self) -> [u8; 8] {
    self.data[6..14].try_into().unwrap()
  }
  pub fn auction_type(&self) -> AuctionType {
    AuctionType(self.data[14])
  }
  pub fn reference_price(&self) -> u64 {
    u64::from_le_bytes(self.data[15..23].try_into().unwrap())
  }
  pub fn buy_shares(&self) -> u32 {
    u32::from_le_bytes(self.data[23..27].try_into().unwrap())
  }
  pub fn sell_shares(&self) -> u32 {
    u32::from_le_bytes(self.data[27..31].try_into().unwrap())
  }
  pub fn indicative_price(&self) -> u64 {
    u64::from_le_bytes(self.data[31..39].try_into().unwrap())
  }
  pub fn auction_only_price(&self) -> u64 {
    u64::from_le_bytes(self.data[39..47].try_into().unwrap())
  }

} // impl AuctionUpdate

impl<'a> fmt::Display for AuctionUpdate<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AuctionUpdate(length:{},message_type:{},time_offset:{},stock_symbol:{},auction_type:{},reference_price:{},buy_shares:{},sell_shares:{},indicative_price:{},auction_only_price:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.stock_symbol()[..]), self.auction_type(), self.reference_price(), self.buy_shares(), self.sell_shares(), self.indicative_price(), self.auction_only_price())
  }
}

#[derive(Clone, Default)]
pub struct ConstituentSymbolMapping<'a> {
  data: &'a [u8],
} // ConstituentSymbolMapping

impl<'a> ConstituentSymbolMapping<'a> {
  pub const TYPE : u8 = 0x9E as u8;
  pub fn new(data: &'a [u8]) -> ConstituentSymbolMapping<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn feed_symbol(&self) -> [u8; 6] {
    self.data[2..8].try_into().unwrap()
  }
  pub fn osi_symbol(&self) -> [u8; 21] {
    self.data[8..29].try_into().unwrap()
  }
  pub fn symbol_condition(&self) -> SymbolCondition {
    SymbolCondition(self.data[29])
  }
  pub fn underlying(&self) -> [u8; 8] {
    self.data[30..38].try_into().unwrap()
  }
  pub fn soq_identifier(&self) -> [u8; 20] {
    self.data[38..58].try_into().unwrap()
  }

} // impl ConstituentSymbolMapping

impl<'a> fmt::Display for ConstituentSymbolMapping<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ConstituentSymbolMapping(length:{},message_type:{},feed_symbol:{},osi_symbol:{},symbol_condition:{},underlying:{},soq_identifier:{})", self.length(), self.message_type(), String::from_utf8_lossy(&self.feed_symbol()[..]), String::from_utf8_lossy(&self.osi_symbol()[..]), self.symbol_condition(), String::from_utf8_lossy(&self.underlying()[..]), String::from_utf8_lossy(&self.soq_identifier()[..]))
  }
}

#[derive(Clone, Default)]
pub struct DeleteOrder<'a> {
  data: &'a [u8],
} // DeleteOrder

impl<'a> DeleteOrder<'a> {
  pub const TYPE : u8 = 0x29 as u8;
  pub fn new(data: &'a [u8]) -> DeleteOrder<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }

} // impl DeleteOrder

impl<'a> fmt::Display for DeleteOrder<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "DeleteOrder(length:{},message_type:{},time_offset:{},order_id:{})", self.length(), self.message_type(), self.time_offset(), self.order_id())
  }
}

#[derive(Clone, Default)]
pub struct EndOfSession<'a> {
  data: &'a [u8],
} // EndOfSession

impl<'a> EndOfSession<'a> {
  pub const TYPE : u8 = 0x2D as u8;
  pub fn new(data: &'a [u8]) -> EndOfSession<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn timestamp(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl EndOfSession

impl<'a> fmt::Display for EndOfSession<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "EndOfSession(length:{},message_type:{},timestamp:{})", self.length(), self.message_type(), self.timestamp())
  }
}

#[derive(Clone, Default)]
pub struct GapRequest<'a> {
  data: &'a [u8],
} // GapRequest

impl<'a> GapRequest<'a> {
  pub const TYPE : u8 = 0x03 as u8;
  pub fn new(data: &'a [u8]) -> GapRequest<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn unit(&self) -> u8 {
    self.data[2]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[3..7].try_into().unwrap())
  }
  pub fn count(&self) -> u16 {
    u16::from_le_bytes(self.data[7..9].try_into().unwrap())
  }

} // impl GapRequest

impl<'a> fmt::Display for GapRequest<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "GapRequest(length:{},message_type:{},unit:{},sequence:{},count:{})", self.length(), self.message_type(), self.unit(), self.sequence(), self.count())
  }
}

#[derive(Clone, Default)]
pub struct GapResponse<'a> {
  data: &'a [u8],
} // GapResponse

impl<'a> GapResponse<'a> {
  pub const TYPE : u8 = 0x04 as u8;
  pub fn new(data: &'a [u8]) -> GapResponse<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn unit(&self) -> u8 {
    self.data[2]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[3..7].try_into().unwrap())
  }
  pub fn count(&self) -> u16 {
    u16::from_le_bytes(self.data[7..9].try_into().unwrap())
  }
  pub fn status(&self) -> GapResponseStatus {
    GapResponseStatus(self.data[9])
  }

} // impl GapResponse

impl<'a> fmt::Display for GapResponse<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "GapResponse(length:{},message_type:{},unit:{},sequence:{},count:{},status:{})", self.length(), self.message_type(), self.unit(), self.sequence(), self.count(), self.status())
  }
}

#[derive(Clone, Default)]
pub struct InstrumentDefinitionFinished<'a> {
  data: &'a [u8],
} // InstrumentDefinitionFinished

impl<'a> InstrumentDefinitionFinished<'a> {
  pub const TYPE : u8 = 0x86 as u8;
  pub fn new(data: &'a [u8]) -> InstrumentDefinitionFinished<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }

} // impl InstrumentDefinitionFinished

impl<'a> fmt::Display for InstrumentDefinitionFinished<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "InstrumentDefinitionFinished(length:{},message_type:{})", self.length(), self.message_type())
  }
}

#[derive(Clone, Default)]
pub struct InstrumentDefinitionRequest<'a> {
  data: &'a [u8],
} // InstrumentDefinitionRequest

impl<'a> InstrumentDefinitionRequest<'a> {
  pub const TYPE : u8 = 0x84 as u8;
  pub fn new(data: &'a [u8]) -> InstrumentDefinitionRequest<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl InstrumentDefinitionRequest

impl<'a> fmt::Display for InstrumentDefinitionRequest<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "InstrumentDefinitionRequest(length:{},message_type:{},sequence:{})", self.length(), self.message_type(), self.sequence())
  }
}

#[derive(Clone, Default)]
pub struct InstrumentDefinitionResponse<'a> {
  data: &'a [u8],
} // InstrumentDefinitionResponse

impl<'a> InstrumentDefinitionResponse<'a> {
  pub const TYPE : u8 = 0x85 as u8;
  pub fn new(data: &'a [u8]) -> InstrumentDefinitionResponse<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn instrument_count(&self) -> u32 {
    u32::from_le_bytes(self.data[6..10].try_into().unwrap())
  }
  pub fn status(&self) -> InstrumentDefinitionResponseStatus {
    InstrumentDefinitionResponseStatus(self.data[10])
  }

} // impl InstrumentDefinitionResponse

impl<'a> fmt::Display for InstrumentDefinitionResponse<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "InstrumentDefinitionResponse(length:{},message_type:{},sequence:{},instrument_count:{},status:{})", self.length(), self.message_type(), self.sequence(), self.instrument_count(), self.status())
  }
}

#[derive(Clone, Default)]
pub struct Login<'a> {
  data: &'a [u8],
} // Login

impl<'a> Login<'a> {
  pub const TYPE : u8 = 0x01 as u8;
  pub fn new(data: &'a [u8]) -> Login<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sessionsubid(&self) -> [u8; 4] {
    self.data[2..6].try_into().unwrap()
  }
  pub fn username(&self) -> [u8; 4] {
    self.data[6..10].try_into().unwrap()
  }
  pub fn filler(&self) -> [u8; 2] {
    self.data[10..12].try_into().unwrap()
  }
  pub fn password(&self) -> [u8; 10] {
    self.data[12..22].try_into().unwrap()
  }

} // impl Login

impl<'a> fmt::Display for Login<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Login(length:{},message_type:{},sessionsubid:{},username:{},filler:{},password:{})", self.length(), self.message_type(), String::from_utf8_lossy(&self.sessionsubid()[..]), String::from_utf8_lossy(&self.username()[..]), String::from_utf8_lossy(&self.filler()[..]), String::from_utf8_lossy(&self.password()[..]))
  }
}

#[derive(Clone, Default)]
pub struct LoginResponse<'a> {
  data: &'a [u8],
} // LoginResponse

impl<'a> LoginResponse<'a> {
  pub const TYPE : u8 = 0x02 as u8;
  pub fn new(data: &'a [u8]) -> LoginResponse<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn status(&self) -> LoginResponseStatus {
    LoginResponseStatus(self.data[2])
  }

} // impl LoginResponse

impl<'a> fmt::Display for LoginResponse<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "LoginResponse(length:{},message_type:{},status:{})", self.length(), self.message_type(), self.status())
  }
}

#[derive(Clone, Default)]
pub struct ModifyLong<'a> {
  data: &'a [u8],
} // ModifyLong

impl<'a> ModifyLong<'a> {
  pub const TYPE : u8 = 0x27 as u8;
  pub fn new(data: &'a [u8]) -> ModifyLong<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[14..18].try_into().unwrap())
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[18..26].try_into().unwrap())
  }
  pub fn modify_flags(&self) -> ModifyFlagsT {
    ModifyFlagsT{ byte1: ModifyFlagsByte1(self.data[26]) }
  }

} // impl ModifyLong

impl<'a> fmt::Display for ModifyLong<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ModifyLong(length:{},message_type:{},time_offset:{},order_id:{},quantity:{},price:{},modify_flags:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.quantity(), self.price(), self.modify_flags())
  }
}

#[derive(Clone, Default)]
pub struct ModifyShort<'a> {
  data: &'a [u8],
} // ModifyShort

impl<'a> ModifyShort<'a> {
  pub const TYPE : u8 = 0x28 as u8;
  pub fn new(data: &'a [u8]) -> ModifyShort<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn quantity(&self) -> u16 {
    u16::from_le_bytes(self.data[14..16].try_into().unwrap())
  }
  pub fn price(&self) -> u16 {
    u16::from_le_bytes(self.data[16..18].try_into().unwrap())
  }
  pub fn modify_flags(&self) -> ModifyFlagsT {
    ModifyFlagsT{ byte1: ModifyFlagsByte1(self.data[18]) }
  }

} // impl ModifyShort

impl<'a> fmt::Display for ModifyShort<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ModifyShort(length:{},message_type:{},time_offset:{},order_id:{},quantity:{},price:{},modify_flags:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.quantity(), self.price(), self.modify_flags())
  }
}

#[derive(Clone, Default)]
pub struct OptionsAuctionUpdate<'a> {
  data: &'a [u8],
} // OptionsAuctionUpdate

impl<'a> OptionsAuctionUpdate<'a> {
  pub const TYPE : u8 = 0xD1 as u8;
  pub fn new(data: &'a [u8]) -> OptionsAuctionUpdate<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 8] {
    self.data[6..14].try_into().unwrap()
  }
  pub fn auction_type(&self) -> AuctionType {
    AuctionType(self.data[14])
  }
  pub fn reference_price(&self) -> u64 {
    u64::from_le_bytes(self.data[15..23].try_into().unwrap())
  }
  pub fn buy_contracts(&self) -> u32 {
    u32::from_le_bytes(self.data[23..27].try_into().unwrap())
  }
  pub fn sell_contracts(&self) -> u32 {
    u32::from_le_bytes(self.data[27..31].try_into().unwrap())
  }
  pub fn indicative_price(&self) -> u64 {
    u64::from_le_bytes(self.data[31..39].try_into().unwrap())
  }
  pub fn auction_only_price(&self) -> u64 {
    u64::from_le_bytes(self.data[39..47].try_into().unwrap())
  }
  pub fn opening_condition(&self) -> OpeningCondition {
    OpeningCondition(self.data[47])
  }
  pub fn composite_market_bid_price(&self) -> u64 {
    u64::from_le_bytes(self.data[48..56].try_into().unwrap())
  }
  pub fn composite_market_offer_price(&self) -> u64 {
    u64::from_le_bytes(self.data[56..64].try_into().unwrap())
  }

} // impl OptionsAuctionUpdate

impl<'a> fmt::Display for OptionsAuctionUpdate<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OptionsAuctionUpdate(length:{},message_type:{},time_offset:{},symbol:{},auction_type:{},reference_price:{},buy_contracts:{},sell_contracts:{},indicative_price:{},auction_only_price:{},opening_condition:{},composite_market_bid_price:{},composite_market_offer_price:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.symbol()[..]), self.auction_type(), self.reference_price(), self.buy_contracts(), self.sell_contracts(), self.indicative_price(), self.auction_only_price(), self.opening_condition(), self.composite_market_bid_price(), self.composite_market_offer_price())
  }
}

#[derive(Clone, Default)]
pub struct OrderExecuted<'a> {
  data: &'a [u8],
} // OrderExecuted

impl<'a> OrderExecuted<'a> {
  pub const TYPE : u8 = 0x23 as u8;
  pub fn new(data: &'a [u8]) -> OrderExecuted<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn executed_quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[14..18].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[18..26].try_into().unwrap())
  }
  pub fn trade_condition(&self) -> TradeCondition {
    TradeCondition(self.data[26])
  }

} // impl OrderExecuted

impl<'a> fmt::Display for OrderExecuted<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderExecuted(length:{},message_type:{},time_offset:{},order_id:{},executed_quantity:{},execution_id:{},trade_condition:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.executed_quantity(), self.execution_id(), self.trade_condition())
  }
}

#[derive(Clone, Default)]
pub struct OrderExecutedAtPriceSize<'a> {
  data: &'a [u8],
} // OrderExecutedAtPriceSize

impl<'a> OrderExecutedAtPriceSize<'a> {
  pub const TYPE : u8 = 0x24 as u8;
  pub fn new(data: &'a [u8]) -> OrderExecutedAtPriceSize<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn executed_quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[14..18].try_into().unwrap())
  }
  pub fn remaining_quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[18..22].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[22..30].try_into().unwrap())
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[30..38].try_into().unwrap())
  }
  pub fn trade_condition(&self) -> TradeCondition {
    TradeCondition(self.data[38])
  }

} // impl OrderExecutedAtPriceSize

impl<'a> fmt::Display for OrderExecutedAtPriceSize<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderExecutedAtPriceSize(length:{},message_type:{},time_offset:{},order_id:{},executed_quantity:{},remaining_quantity:{},execution_id:{},price:{},trade_condition:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.executed_quantity(), self.remaining_quantity(), self.execution_id(), self.price(), self.trade_condition())
  }
}

#[derive(Clone, Default)]
pub struct ReduceSizeLong<'a> {
  data: &'a [u8],
} // ReduceSizeLong

impl<'a> ReduceSizeLong<'a> {
  pub const TYPE : u8 = 0x25 as u8;
  pub fn new(data: &'a [u8]) -> ReduceSizeLong<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn canceled_quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[14..18].try_into().unwrap())
  }

} // impl ReduceSizeLong

impl<'a> fmt::Display for ReduceSizeLong<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ReduceSizeLong(length:{},message_type:{},time_offset:{},order_id:{},canceled_quantity:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.canceled_quantity())
  }
}

#[derive(Clone, Default)]
pub struct ReduceSizeShort<'a> {
  data: &'a [u8],
} // ReduceSizeShort

impl<'a> ReduceSizeShort<'a> {
  pub const TYPE : u8 = 0x26 as u8;
  pub fn new(data: &'a [u8]) -> ReduceSizeShort<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn canceled_quantity(&self) -> u16 {
    u16::from_le_bytes(self.data[14..16].try_into().unwrap())
  }

} // impl ReduceSizeShort

impl<'a> fmt::Display for ReduceSizeShort<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ReduceSizeShort(length:{},message_type:{},time_offset:{},order_id:{},canceled_quantity:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.canceled_quantity())
  }
}

#[derive(Clone, Default)]
pub struct RetailPriceImprovement<'a> {
  data: &'a [u8],
} // RetailPriceImprovement

impl<'a> RetailPriceImprovement<'a> {
  pub const TYPE : u8 = 0x98 as u8;
  pub fn new(data: &'a [u8]) -> RetailPriceImprovement<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 8] {
    self.data[6..14].try_into().unwrap()
  }
  pub fn retail_price_improvement(&self) -> RetailPriceImprovementKind {
    RetailPriceImprovementKind(self.data[14])
  }

} // impl RetailPriceImprovement

impl<'a> fmt::Display for RetailPriceImprovement<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RetailPriceImprovement(length:{},message_type:{},time_offset:{},symbol:{},retail_price_improvement:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.symbol()[..]), self.retail_price_improvement())
  }
}

#[derive(Clone, Default)]
pub struct SequencedUnitHeader<'a> {
  data: &'a [u8],
} // SequencedUnitHeader

impl<'a> SequencedUnitHeader<'a> {
  pub fn new(data: &'a [u8]) -> SequencedUnitHeader<'a> { Self{ data } }
  pub fn hdr_length(&self) -> u16 {
    u16::from_le_bytes(self.data[0..2].try_into().unwrap())
  }
  pub fn hdr_count(&self) -> u8 {
    self.data[2]
  }
  pub fn hdr_unit(&self) -> u8 {
    self.data[3]
  }
  pub fn hdr_sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[4..8].try_into().unwrap())
  }

} // impl SequencedUnitHeader

impl<'a> fmt::Display for SequencedUnitHeader<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SequencedUnitHeader(hdr_length:{},hdr_count:{},hdr_unit:{},hdr_sequence:{})", self.hdr_length(), self.hdr_count(), self.hdr_unit(), self.hdr_sequence())
  }
}

#[derive(Clone, Default)]
pub struct SoqStrikeRangeUpdate<'a> {
  data: &'a [u8],
} // SoqStrikeRangeUpdate

impl<'a> SoqStrikeRangeUpdate<'a> {
  pub const TYPE : u8 = 0x9D as u8;
  pub fn new(data: &'a [u8]) -> SoqStrikeRangeUpdate<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn soq_identifier(&self) -> [u8; 20] {
    self.data[6..26].try_into().unwrap()
  }
  pub fn lower_strike_price(&self) -> u64 {
    u64::from_le_bytes(self.data[26..34].try_into().unwrap())
  }
  pub fn upper_strike_price(&self) -> u64 {
    u64::from_le_bytes(self.data[34..42].try_into().unwrap())
  }

} // impl SoqStrikeRangeUpdate

impl<'a> fmt::Display for SoqStrikeRangeUpdate<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SoqStrikeRangeUpdate(length:{},message_type:{},time_offset:{},soq_identifier:{},lower_strike_price:{},upper_strike_price:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.soq_identifier()[..]), self.lower_strike_price(), self.upper_strike_price())
  }
}

#[derive(Clone, Default)]
pub struct SpinFinished<'a> {
  data: &'a [u8],
} // SpinFinished

impl<'a> SpinFinished<'a> {
  pub const TYPE : u8 = 0x83 as u8;
  pub fn new(data: &'a [u8]) -> SpinFinished<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl SpinFinished

impl<'a> fmt::Display for SpinFinished<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SpinFinished(length:{},message_type:{},sequence:{})", self.length(), self.message_type(), self.sequence())
  }
}

#[derive(Clone, Default)]
pub struct SpinImageAvailable<'a> {
  data: &'a [u8],
} // SpinImageAvailable

impl<'a> SpinImageAvailable<'a> {
  pub const TYPE : u8 = 0x80 as u8;
  pub fn new(data: &'a [u8]) -> SpinImageAvailable<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl SpinImageAvailable

impl<'a> fmt::Display for SpinImageAvailable<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SpinImageAvailable(length:{},message_type:{},sequence:{})", self.length(), self.message_type(), self.sequence())
  }
}

#[derive(Clone, Default)]
pub struct SpinRequest<'a> {
  data: &'a [u8],
} // SpinRequest

impl<'a> SpinRequest<'a> {
  pub const TYPE : u8 = 0x81 as u8;
  pub fn new(data: &'a [u8]) -> SpinRequest<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl SpinRequest

impl<'a> fmt::Display for SpinRequest<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SpinRequest(length:{},message_type:{},sequence:{})", self.length(), self.message_type(), self.sequence())
  }
}

#[derive(Clone, Default)]
pub struct SpinResponse<'a> {
  data: &'a [u8],
} // SpinResponse

impl<'a> SpinResponse<'a> {
  pub const TYPE : u8 = 0x82 as u8;
  pub fn new(data: &'a [u8]) -> SpinResponse<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn sequence(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_count(&self) -> u32 {
    u32::from_le_bytes(self.data[6..10].try_into().unwrap())
  }
  pub fn status(&self) -> SpinResponseStatus {
    SpinResponseStatus(self.data[10])
  }

} // impl SpinResponse

impl<'a> fmt::Display for SpinResponse<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SpinResponse(length:{},message_type:{},sequence:{},order_count:{},status:{})", self.length(), self.message_type(), self.sequence(), self.order_count(), self.status())
  }
}

#[derive(Clone, Default)]
pub struct SymbolMapping<'a> {
  data: &'a [u8],
} // SymbolMapping

impl<'a> SymbolMapping<'a> {
  pub const TYPE : u8 = 0x2E as u8;
  pub fn new(data: &'a [u8]) -> SymbolMapping<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn feed_symbol(&self) -> [u8; 6] {
    self.data[2..8].try_into().unwrap()
  }
  pub fn osi_symbol(&self) -> [u8; 21] {
    self.data[8..29].try_into().unwrap()
  }
  pub fn symbol_condition(&self) -> SymbolCondition {
    SymbolCondition(self.data[29])
  }
  pub fn underlying(&self) -> [u8; 8] {
    self.data[30..38].try_into().unwrap()
  }

} // impl SymbolMapping

impl<'a> fmt::Display for SymbolMapping<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SymbolMapping(length:{},message_type:{},feed_symbol:{},osi_symbol:{},symbol_condition:{},underlying:{})", self.length(), self.message_type(), String::from_utf8_lossy(&self.feed_symbol()[..]), String::from_utf8_lossy(&self.osi_symbol()[..]), self.symbol_condition(), String::from_utf8_lossy(&self.underlying()[..]))
  }
}

#[derive(Clone, Default)]
pub struct Time<'a> {
  data: &'a [u8],
} // Time

impl<'a> Time<'a> {
  pub const TYPE : u8 = 0x20 as u8;
  pub fn new(data: &'a [u8]) -> Time<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl Time

impl<'a> fmt::Display for Time<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Time(length:{},message_type:{},time:{})", self.length(), self.message_type(), self.time())
  }
}

#[derive(Clone, Default)]
pub struct TradeBreak<'a> {
  data: &'a [u8],
} // TradeBreak

impl<'a> TradeBreak<'a> {
  pub const TYPE : u8 = 0x2C as u8;
  pub fn new(data: &'a [u8]) -> TradeBreak<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }

} // impl TradeBreak

impl<'a> fmt::Display for TradeBreak<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TradeBreak(length:{},message_type:{},time_offset:{},execution_id:{})", self.length(), self.message_type(), self.time_offset(), self.execution_id())
  }
}

#[derive(Clone, Default)]
pub struct TradeExpanded<'a> {
  data: &'a [u8],
} // TradeExpanded

impl<'a> TradeExpanded<'a> {
  pub const TYPE : u8 = 0x30 as u8;
  pub fn new(data: &'a [u8]) -> TradeExpanded<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn side_indicator(&self) -> Side {
    Side(self.data[14])
  }
  pub fn quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[15..19].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 8] {
    self.data[19..27].try_into().unwrap()
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[27..35].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[35..43].try_into().unwrap())
  }
  pub fn trade_condition(&self) -> TradeCondition {
    TradeCondition(self.data[43])
  }

} // impl TradeExpanded

impl<'a> fmt::Display for TradeExpanded<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TradeExpanded(length:{},message_type:{},time_offset:{},order_id:{},side_indicator:{},quantity:{},symbol:{},price:{},execution_id:{},trade_condition:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.side_indicator(), self.quantity(), String::from_utf8_lossy(&self.symbol()[..]), self.price(), self.execution_id(), self.trade_condition())
  }
}

#[derive(Clone, Default)]
pub struct TradeLong<'a> {
  data: &'a [u8],
} // TradeLong

impl<'a> TradeLong<'a> {
  pub const TYPE : u8 = 0x2A as u8;
  pub fn new(data: &'a [u8]) -> TradeLong<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn side_indicator(&self) -> Side {
    Side(self.data[14])
  }
  pub fn quantity(&self) -> u32 {
    u32::from_le_bytes(self.data[15..19].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 6] {
    self.data[19..25].try_into().unwrap()
  }
  pub fn price(&self) -> u64 {
    u64::from_le_bytes(self.data[25..33].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[33..41].try_into().unwrap())
  }
  pub fn trade_condition(&self) -> TradeCondition {
    TradeCondition(self.data[41])
  }

} // impl TradeLong

impl<'a> fmt::Display for TradeLong<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TradeLong(length:{},message_type:{},time_offset:{},order_id:{},side_indicator:{},quantity:{},symbol:{},price:{},execution_id:{},trade_condition:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.side_indicator(), self.quantity(), String::from_utf8_lossy(&self.symbol()[..]), self.price(), self.execution_id(), self.trade_condition())
  }
}

#[derive(Clone, Default)]
pub struct TradeShort<'a> {
  data: &'a [u8],
} // TradeShort

impl<'a> TradeShort<'a> {
  pub const TYPE : u8 = 0x2B as u8;
  pub fn new(data: &'a [u8]) -> TradeShort<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn order_id(&self) -> u64 {
    u64::from_le_bytes(self.data[6..14].try_into().unwrap())
  }
  pub fn side_indicator(&self) -> Side {
    Side(self.data[14])
  }
  pub fn quantity(&self) -> u16 {
    u16::from_le_bytes(self.data[15..17].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 6] {
    self.data[17..23].try_into().unwrap()
  }
  pub fn price(&self) -> u16 {
    u16::from_le_bytes(self.data[23..25].try_into().unwrap())
  }
  pub fn execution_id(&self) -> u64 {
    u64::from_le_bytes(self.data[25..33].try_into().unwrap())
  }
  pub fn trade_condition(&self) -> TradeCondition {
    TradeCondition(self.data[33])
  }

} // impl TradeShort

impl<'a> fmt::Display for TradeShort<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TradeShort(length:{},message_type:{},time_offset:{},order_id:{},side_indicator:{},quantity:{},symbol:{},price:{},execution_id:{},trade_condition:{})", self.length(), self.message_type(), self.time_offset(), self.order_id(), self.side_indicator(), self.quantity(), String::from_utf8_lossy(&self.symbol()[..]), self.price(), self.execution_id(), self.trade_condition())
  }
}

#[derive(Clone, Default)]
pub struct TradingStatus<'a> {
  data: &'a [u8],
} // TradingStatus

impl<'a> TradingStatus<'a> {
  pub const TYPE : u8 = 0x31 as u8;
  pub fn new(data: &'a [u8]) -> TradingStatus<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn symbol(&self) -> [u8; 8] {
    self.data[6..14].try_into().unwrap()
  }
  pub fn trading_status(&self) -> TradingStatusKind {
    TradingStatusKind(self.data[14])
  }
  pub fn reg_sho_action(&self) -> RegShoAction {
    RegShoAction(self.data[15])
  }
  pub fn gth_trading_status(&self) -> GthTradingStatus {
    GthTradingStatus(self.data[16])
  }
  pub fn reserved2(&self) -> u8 {
    self.data[17]
  }

} // impl TradingStatus

impl<'a> fmt::Display for TradingStatus<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TradingStatus(length:{},message_type:{},time_offset:{},symbol:{},trading_status:{},reg_sho_action:{},gth_trading_status:{},reserved2:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.symbol()[..]), self.trading_status(), self.reg_sho_action(), self.gth_trading_status(), self.reserved2())
  }
}

#[derive(Clone, Default)]
pub struct TransactionBegin<'a> {
  data: &'a [u8],
} // TransactionBegin

impl<'a> TransactionBegin<'a> {
  pub const TYPE : u8 = 0xBC as u8;
  pub fn new(data: &'a [u8]) -> TransactionBegin<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl TransactionBegin

impl<'a> fmt::Display for TransactionBegin<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TransactionBegin(length:{},message_type:{},time_offset:{})", self.length(), self.message_type(), self.time_offset())
  }
}

#[derive(Clone, Default)]
pub struct TransactionEnd<'a> {
  data: &'a [u8],
} // TransactionEnd

impl<'a> TransactionEnd<'a> {
  pub const TYPE : u8 = 0xBD as u8;
  pub fn new(data: &'a [u8]) -> TransactionEnd<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl TransactionEnd

impl<'a> fmt::Display for TransactionEnd<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TransactionEnd(length:{},message_type:{},time_offset:{})", self.length(), self.message_type(), self.time_offset())
  }
}

#[derive(Clone, Default)]
pub struct UnitClear<'a> {
  data: &'a [u8],
} // UnitClear

impl<'a> UnitClear<'a> {
  pub const TYPE : u8 = 0x97 as u8;
  pub fn new(data: &'a [u8]) -> UnitClear<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }

} // impl UnitClear

impl<'a> fmt::Display for UnitClear<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "UnitClear(length:{},message_type:{},time_offset:{})", self.length(), self.message_type(), self.time_offset())
  }
}

#[derive(Clone, Default)]
pub struct WidthUpdate<'a> {
  data: &'a [u8],
} // WidthUpdate

impl<'a> WidthUpdate<'a> {
  pub const TYPE : u8 = 0xD2 as u8;
  pub fn new(data: &'a [u8]) -> WidthUpdate<'a> { Self{ data } }
  pub fn length(&self) -> u8 {
    self.data[0]
  }
  pub fn message_type(&self) -> u8 {
    self.data[1]
  }
  pub fn time_offset(&self) -> u32 {
    u32::from_le_bytes(self.data[2..6].try_into().unwrap())
  }
  pub fn underlying(&self) -> [u8; 8] {
    self.data[6..14].try_into().unwrap()
  }
  pub fn width_type(&self) -> WidthType {
    WidthType(self.data[14])
  }
  pub fn multiplier(&self) -> u32 {
    u32::from_le_bytes(self.data[15..19].try_into().unwrap())
  }

} // impl WidthUpdate

impl<'a> fmt::Display for WidthUpdate<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "WidthUpdate(length:{},message_type:{},time_offset:{},underlying:{},width_type:{},multiplier:{})", self.length(), self.message_type(), self.time_offset(), String::from_utf8_lossy(&self.underlying()[..]), self.width_type(), self.multiplier())
  }
}

pub enum PitchMessage<'a> {
  AddOrderExpanded(AddOrderExpanded<'a>),
  AddOrderLong(AddOrderLong<'a>),
  AddOrderShort(AddOrderShort<'a>),
  AuctionCancel(AuctionCancel<'a>),
  AuctionNotification(AuctionNotification<'a>),
  AuctionSummary(AuctionSummary<'a>),
  AuctionTrade(AuctionTrade<'a>),
  AuctionUpdate(AuctionUpdate<'a>),
  ConstituentSymbolMapping(ConstituentSymbolMapping<'a>),
  DeleteOrder(DeleteOrder<'a>),
  EndOfSession(EndOfSession<'a>),
  GapRequest(GapRequest<'a>),
  GapResponse(GapResponse<'a>),
  InstrumentDefinitionFinished(InstrumentDefinitionFinished<'a>),
  InstrumentDefinitionRequest(InstrumentDefinitionRequest<'a>),
  InstrumentDefinitionResponse(InstrumentDefinitionResponse<'a>),
  Login(Login<'a>),
  LoginResponse(LoginResponse<'a>),
  ModifyLong(ModifyLong<'a>),
  ModifyShort(ModifyShort<'a>),
  OptionsAuctionUpdate(OptionsAuctionUpdate<'a>),
  OrderExecuted(OrderExecuted<'a>),
  OrderExecutedAtPriceSize(OrderExecutedAtPriceSize<'a>),
  ReduceSizeLong(ReduceSizeLong<'a>),
  ReduceSizeShort(ReduceSizeShort<'a>),
  RetailPriceImprovement(RetailPriceImprovement<'a>),
  SoqStrikeRangeUpdate(SoqStrikeRangeUpdate<'a>),
  SpinFinished(SpinFinished<'a>),
  SpinImageAvailable(SpinImageAvailable<'a>),
  SpinRequest(SpinRequest<'a>),
  SpinResponse(SpinResponse<'a>),
  SymbolMapping(SymbolMapping<'a>),
  Time(Time<'a>),
  TradeBreak(TradeBreak<'a>),
  TradeExpanded(TradeExpanded<'a>),
  TradeLong(TradeLong<'a>),
  TradeShort(TradeShort<'a>),
  TradingStatus(TradingStatus<'a>),
  TransactionBegin(TransactionBegin<'a>),
  TransactionEnd(TransactionEnd<'a>),
  UnitClear(UnitClear<'a>),
  WidthUpdate(WidthUpdate<'a>),
} // PitchMessage

impl<'a> fmt::Display for PitchMessage<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PitchMessage(")?;
    match self {
      PitchMessage::AddOrderExpanded(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AddOrderLong(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AddOrderShort(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AuctionCancel(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AuctionNotification(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AuctionSummary(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AuctionTrade(msg) => { write!(f, "{}", msg)? },
      PitchMessage::AuctionUpdate(msg) => { write!(f, "{}", msg)? },
      PitchMessage::ConstituentSymbolMapping(msg) => { write!(f, "{}", msg)? },
      PitchMessage::DeleteOrder(msg) => { write!(f, "{}", msg)? },
      PitchMessage::EndOfSession(msg) => { write!(f, "{}", msg)? },
      PitchMessage::GapRequest(msg) => { write!(f, "{}", msg)? },
      PitchMessage::GapResponse(msg) => { write!(f, "{}", msg)? },
      PitchMessage::InstrumentDefinitionFinished(msg) => { write!(f, "{}", msg)? },
      PitchMessage::InstrumentDefinitionRequest(msg) => { write!(f, "{}", msg)? },
      PitchMessage::InstrumentDefinitionResponse(msg) => { write!(f, "{}", msg)? },
      PitchMessage::Login(msg) => { write!(f, "{}", msg)? },
      PitchMessage::LoginResponse(msg) => { write!(f, "{}", msg)? },
      PitchMessage::ModifyLong(msg) => { write!(f, "{}", msg)? },
      PitchMessage::ModifyShort(msg) => { write!(f, "{}", msg)? },
      PitchMessage::OptionsAuctionUpdate(msg) => { write!(f, "{}", msg)? },
      PitchMessage::OrderExecuted(msg) => { write!(f, "{}", msg)? },
      PitchMessage::OrderExecutedAtPriceSize(msg) => { write!(f, "{}", msg)? },
      PitchMessage::ReduceSizeLong(msg) => { write!(f, "{}", msg)? },
      PitchMessage::ReduceSizeShort(msg) => { write!(f, "{}", msg)? },
      PitchMessage::RetailPriceImprovement(msg) => { write!(f, "{}", msg)? },
      PitchMessage::SoqStrikeRangeUpdate(msg) => { write!(f, "{}", msg)? },
      PitchMessage::SpinFinished(msg) => { write!(f, "{}", msg)? },
      PitchMessage::SpinImageAvailable(msg) => { write!(f, "{}", msg)? },
      PitchMessage::SpinRequest(msg) => { write!(f, "{}", msg)? },
      PitchMessage::SpinResponse(msg) => { write!(f, "{}", msg)? },
      PitchMessage::SymbolMapping(msg) => { write!(f, "{}", msg)? },
      PitchMessage::Time(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TradeBreak(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TradeExpanded(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TradeLong(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TradeShort(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TradingStatus(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TransactionBegin(msg) => { write!(f, "{}", msg)? },
      PitchMessage::TransactionEnd(msg) => { write!(f, "{}", msg)? },
      PitchMessage::UnitClear(msg) => { write!(f, "{}", msg)? },
      PitchMessage::WidthUpdate(msg) => { write!(f, "{}", msg)? },
    }
    write!(f, ")")
  }
}

pub struct PitchItem<'a> {
  pub msg: PitchMessage<'a>,
  pub unit: u8,
  pub seqno: u32,
  pub msg_len: usize,
}

pub struct PitchReader<'a> {
  data: &'a [u8],
}

pub struct PitchIterator<'a> {
  data: &'a [u8],
  bytes_eaten: usize,
  total_bytes: u16,
  num_messages: u8,
  unit: u8,
  seqno: u32,
}

impl<'a> PitchReader<'a> {
  pub fn new(data: &'a [u8]) -> Self { Self{data} }
  pub fn iter(&self) -> PitchIterator {
    PitchIterator{data: self.data, bytes_eaten: 0, total_bytes: 0, num_messages: 0, unit: 0, seqno: 0}
  }
}

impl<'a> IntoIterator for &'a PitchReader<'a> {
  type Item = PitchItem<'a>;
  type IntoIter = PitchIterator<'a>;
  fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a> Iterator for PitchIterator<'a> {
  type Item = PitchItem<'a>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.bytes_eaten == 0 {
      // do header
      if self.data.len() < 8 { return None; }
      let header = SequencedUnitHeader::new(self.data);
      self.total_bytes = header.hdr_length();
      self.num_messages = header.hdr_count();
      self.unit = header.hdr_unit();
      self.seqno = header.hdr_sequence();
      self.bytes_eaten += 8;
    }
    if self.num_messages == 0 { return None; }
    let bytes_remaining = self.data.len() - self.bytes_eaten;
    if bytes_remaining < 2 { return None; }
    let offset = self.bytes_eaten;
    let msg_len = self.data[offset] as usize;
    let msg_type = self.data[offset+1];
    if self.data.len() - self.bytes_eaten < msg_len {
      return None;
    }
    let seqno = self.seqno;
    self.seqno += 1;
    self.num_messages -= 1;
    self.bytes_eaten += msg_len;
    match msg_type {
      AddOrderExpanded::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AddOrderExpanded(AddOrderExpanded::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AddOrderLong::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AddOrderLong(AddOrderLong::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AddOrderShort::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AddOrderShort(AddOrderShort::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AuctionCancel::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AuctionCancel(AuctionCancel::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AuctionNotification::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AuctionNotification(AuctionNotification::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AuctionSummary::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AuctionSummary(AuctionSummary::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AuctionTrade::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AuctionTrade(AuctionTrade::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      AuctionUpdate::TYPE => {
        return Some(PitchItem{msg: PitchMessage::AuctionUpdate(AuctionUpdate::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      ConstituentSymbolMapping::TYPE => {
        return Some(PitchItem{msg: PitchMessage::ConstituentSymbolMapping(ConstituentSymbolMapping::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      DeleteOrder::TYPE => {
        return Some(PitchItem{msg: PitchMessage::DeleteOrder(DeleteOrder::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      EndOfSession::TYPE => {
        return Some(PitchItem{msg: PitchMessage::EndOfSession(EndOfSession::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      GapRequest::TYPE => {
        return Some(PitchItem{msg: PitchMessage::GapRequest(GapRequest::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      GapResponse::TYPE => {
        return Some(PitchItem{msg: PitchMessage::GapResponse(GapResponse::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      InstrumentDefinitionFinished::TYPE => {
        return Some(PitchItem{msg: PitchMessage::InstrumentDefinitionFinished(InstrumentDefinitionFinished::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      InstrumentDefinitionRequest::TYPE => {
        return Some(PitchItem{msg: PitchMessage::InstrumentDefinitionRequest(InstrumentDefinitionRequest::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      InstrumentDefinitionResponse::TYPE => {
        return Some(PitchItem{msg: PitchMessage::InstrumentDefinitionResponse(InstrumentDefinitionResponse::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      Login::TYPE => {
        return Some(PitchItem{msg: PitchMessage::Login(Login::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      LoginResponse::TYPE => {
        return Some(PitchItem{msg: PitchMessage::LoginResponse(LoginResponse::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      ModifyLong::TYPE => {
        return Some(PitchItem{msg: PitchMessage::ModifyLong(ModifyLong::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      ModifyShort::TYPE => {
        return Some(PitchItem{msg: PitchMessage::ModifyShort(ModifyShort::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      OptionsAuctionUpdate::TYPE => {
        return Some(PitchItem{msg: PitchMessage::OptionsAuctionUpdate(OptionsAuctionUpdate::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      OrderExecuted::TYPE => {
        return Some(PitchItem{msg: PitchMessage::OrderExecuted(OrderExecuted::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      OrderExecutedAtPriceSize::TYPE => {
        return Some(PitchItem{msg: PitchMessage::OrderExecutedAtPriceSize(OrderExecutedAtPriceSize::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      ReduceSizeLong::TYPE => {
        return Some(PitchItem{msg: PitchMessage::ReduceSizeLong(ReduceSizeLong::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      ReduceSizeShort::TYPE => {
        return Some(PitchItem{msg: PitchMessage::ReduceSizeShort(ReduceSizeShort::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      RetailPriceImprovement::TYPE => {
        return Some(PitchItem{msg: PitchMessage::RetailPriceImprovement(RetailPriceImprovement::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      SoqStrikeRangeUpdate::TYPE => {
        return Some(PitchItem{msg: PitchMessage::SoqStrikeRangeUpdate(SoqStrikeRangeUpdate::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      SpinFinished::TYPE => {
        return Some(PitchItem{msg: PitchMessage::SpinFinished(SpinFinished::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      SpinImageAvailable::TYPE => {
        return Some(PitchItem{msg: PitchMessage::SpinImageAvailable(SpinImageAvailable::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      SpinRequest::TYPE => {
        return Some(PitchItem{msg: PitchMessage::SpinRequest(SpinRequest::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      SpinResponse::TYPE => {
        return Some(PitchItem{msg: PitchMessage::SpinResponse(SpinResponse::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      SymbolMapping::TYPE => {
        return Some(PitchItem{msg: PitchMessage::SymbolMapping(SymbolMapping::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      Time::TYPE => {
        return Some(PitchItem{msg: PitchMessage::Time(Time::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TradeBreak::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TradeBreak(TradeBreak::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TradeExpanded::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TradeExpanded(TradeExpanded::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TradeLong::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TradeLong(TradeLong::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TradeShort::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TradeShort(TradeShort::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TradingStatus::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TradingStatus(TradingStatus::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TransactionBegin::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TransactionBegin(TransactionBegin::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      TransactionEnd::TYPE => {
        return Some(PitchItem{msg: PitchMessage::TransactionEnd(TransactionEnd::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      UnitClear::TYPE => {
        return Some(PitchItem{msg: PitchMessage::UnitClear(UnitClear::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      WidthUpdate::TYPE => {
        return Some(PitchItem{msg: PitchMessage::WidthUpdate(WidthUpdate::new(&self.data[offset..])), unit: self.unit, seqno, msg_len: msg_len as usize});
      },
      _ => unimplemented!("message type unknown"),
    }
  }
}



