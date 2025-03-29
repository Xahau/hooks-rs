use crate::c;

mod control;
mod etxn;
mod float;
mod hook;
mod internal;
mod ledger;
mod otxn;
mod slot;
mod state;
mod sto;
mod trace;
mod util;

pub use control::*;
pub use etxn::*;
pub use float::*;
pub use hook::*;
pub(crate) use internal::*;
pub use ledger::*;
pub use otxn::*;
pub use slot::*;
pub use state::*;
pub use sto::*;
pub use trace::*;
pub use util::*;

/// Flags canonical
pub const TF_CANONICAL: u32 = c::tfCANONICAL;

/// Account id byte length
pub const ACC_ID_LEN: usize = 20;
/// Currency code byte length
pub const CURRENCY_CODE_SIZE: usize = 20;
/// Ledger hash byte length
pub const LEDGER_HASH_LEN: usize = 32;
/// Keylet byte length
pub const KEYLET_LEN: usize = 34;
/// State key byte length
pub const STATE_KEY_LEN: usize = 32;
/// Nonce byte length
pub const NONCE_LEN: usize = 32;
/// Hash byte length
pub const HASH_LEN: usize = 32;
/// Hook Hash byte length
pub const HOOK_HASH_LEN: usize = 32;
/// Amount byte length
pub const AMOUNT_LEN: usize = 48;
/// Payment simple transaction byte length
pub const PREPARE_PAYMENT_SIMPLE_SIZE: usize = c::PREPARE_PAYMENT_SIMPLE_SIZE as _;
/// Emit details byte length
pub const EMIT_DETAILS_SIZE: usize = 105;
/// XFL byte length
pub const XFL_LEN: usize = 8;

/// Buffer of the specified size
pub type Buffer<const T: usize> = [u8; T];

/// Account id buffer
pub type AccountId = Buffer<ACC_ID_LEN>;
/// Hash buffer
pub type Hash = Buffer<HASH_LEN>;
/// Keylet buffer
pub type Keylet = Buffer<KEYLET_LEN>;
/// State key buffer
pub type StateKey = Buffer<STATE_KEY_LEN>;
/// Nonce buffer
pub type Nonce = Buffer<NONCE_LEN>;
/// Amount buffer
pub type Amount = Buffer<AMOUNT_LEN>;
/// Simple payment transaction buffer
pub type TxnPaymentSimple = Buffer<PREPARE_PAYMENT_SIMPLE_SIZE>;
/// Emit details buffer
pub type EmitDetails = Buffer<EMIT_DETAILS_SIZE>;
/// Currency code buffer
pub type CurrencyCode = Buffer<CURRENCY_CODE_SIZE>;

/// Transaction type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TxnType {
    Payment = c::ttPAYMENT,
    EscrowCreate = 1,
    EscrowFinish = 2,
    AccountSet = 3,
    EscrowCancel = 4,
    RegularKeySet = 5,
    OfferCreate = 7,
    OfferCancel = 8,
    TicketCreate = 10,
    TicketCancel = 11,
    SignerListSet = 12,
    PaychanCreate = 13,
    PaychanFund = 14,
    PaychanClaim = 15,
    CheckCreate = 16,
    CheckCash = 17,
    CheckCancel = 18,
    DepositPreauth = 19,
    TrustSet = 20,
    AccountDelete = 21,
    HookSet = 22,
    Amendment = 100,
    Fee = 101,
    UnlModify = 102,
}

/// Account type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum AccountType {
    Account = c::atACCOUNT,
    Owner = c::atOWNER,
    Destination = c::atDESTINATION,
    Issuer = c::atISSUER,
    Authorize = c::atAUTHORIZE,
    Unauthorize = c::atUNAUTHORIZE,
    Target = c::atTARGET,
    RegularKey = c::atREGULARKEY,
    PseudoCallback = c::atPSEUDOCALLBACK,
}

/// Amount type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum AmountType {
    Amount = c::amAMOUNT,
    Balance = c::amBALANCE,
    LimitAmount = c::amLIMITAMOUNT,
    TakerPays = c::amTAKERPAYS,
    TakerGets = c::amTAKERGETS,
    LowLimit = c::amLOWLIMIT,
    HighLimit = c::amHIGHLIMIT,
    Fee = c::amFEE,
    SendMax = c::amSENDMAX,
    DeliverMin = c::amDELIVERMIN,
    MinimumOffer = c::amMINIMUMOFFER,
    RippleEscrow = c::amRIPPLEESCROW,
    DeliveredAmount = c::amDELIVEREDAMOUNT,
}

/// Keylet type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum KeyletType {
    Hook = c::KEYLET_HOOK,
    HookState = c::KEYLET_HOOK_STATE,
    Account = c::KEYLET_ACCOUNT,
    Amendments = c::KEYLET_AMENDMENTS,
    Child = c::KEYLET_CHILD,
    Skip = c::KEYLET_SKIP,
    Fees = c::KEYLET_FEES,
    NegativeUnl = c::KEYLET_NEGATIVE_UNL,
    Line = c::KEYLET_LINE,
    Offer = c::KEYLET_OFFER,
    Quality = c::KEYLET_QUALITY,
    EmittedDir = c::KEYLET_EMITTED_DIR,
    Signers = c::KEYLET_SIGNERS,
    Check = c::KEYLET_CHECK,
    DepositPreauth = c::KEYLET_DEPOSIT_PREAUTH,
    Unchecked = c::KEYLET_UNCHECKED,
    OwnerDir = c::KEYLET_OWNER_DIR,
    Page = c::KEYLET_PAGE,
    Escrow = c::KEYLET_ESCROW,
    Paychan = c::KEYLET_PAYCHAN,
    Emitted = c::KEYLET_EMITTED,
    NftOffer = c::KEYLET_NFT_OFFER,
    HookDefinition = c::KEYLET_HOOK_DEFINITION,
    HookStateDir = c::KEYLET_HOOK_STATE_DIR,
    UriToken = c::KEYLET_URITOKEN,
}

/// Field or amount type
///
/// Used as return of [slot_type] function
#[derive(Clone, Copy)]
pub enum FieldOrXrpAmount {
    /// Field ID
    Field(FieldId),
    /// STI_AMOUNT type contains a native (XRP) amount
    XrpAmount,
    /// STI_AMOUNT type contains non-XRP amount
    NonXrpAmount,
}

/// Flags for [slot_type]
#[derive(Clone, Copy)]
pub enum SlotTypeFlags {
    /// Field
    Field,
    /// STI_AMOUNT type contains a native (XRP) amount
    XrpAmount,
}

/// Field type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum FieldId {
    CloseResolution = c::sfCloseResolution,
    Method = c::sfMethod,
    TransactionResult = c::sfTransactionResult,
    TickSize = c::sfTickSize,
    UNLModifyDisabling = c::sfUNLModifyDisabling,
    HookResult = c::sfHookResult,
    LedgerEntryType = c::sfLedgerEntryType,
    TransactionType = c::sfTransactionType,
    SignerWeight = c::sfSignerWeight,
    TransferFee = c::sfTransferFee,
    Version = c::sfVersion,
    HookStateChangeCount = c::sfHookStateChangeCount,
    HookEmitCount = c::sfHookEmitCount,
    HookExecutionIndex = c::sfHookExecutionIndex,
    HookApiVersion = c::sfHookApiVersion,
    NetworkID = c::sfNetworkID,
    Flags = c::sfFlags,
    SourceTag = c::sfSourceTag,
    Sequence = c::sfSequence,
    PreviousTxnLgrSeq = c::sfPreviousTxnLgrSeq,
    LedgerSequence = c::sfLedgerSequence,
    CloseTime = c::sfCloseTime,
    ParentCloseTime = c::sfParentCloseTime,
    SigningTime = c::sfSigningTime,
    Expiration = c::sfExpiration,
    TransferRate = c::sfTransferRate,
    WalletSize = c::sfWalletSize,
    OwnerCount = c::sfOwnerCount,
    DestinationTag = c::sfDestinationTag,
    HighQualityIn = c::sfHighQualityIn,
    HighQualityOut = c::sfHighQualityOut,
    LowQualityIn = c::sfLowQualityIn,
    LowQualityOut = c::sfLowQualityOut,
    QualityIn = c::sfQualityIn,
    QualityOut = c::sfQualityOut,
    StampEscrow = c::sfStampEscrow,
    BondAmount = c::sfBondAmount,
    LoadFee = c::sfLoadFee,
    OfferSequence = c::sfOfferSequence,
    FirstLedgerSequence = c::sfFirstLedgerSequence,
    LastLedgerSequence = c::sfLastLedgerSequence,
    TransactionIndex = c::sfTransactionIndex,
    OperationLimit = c::sfOperationLimit,
    ReferenceFeeUnits = c::sfReferenceFeeUnits,
    ReserveBase = c::sfReserveBase,
    ReserveIncrement = c::sfReserveIncrement,
    SetFlag = c::sfSetFlag,
    ClearFlag = c::sfClearFlag,
    SignerQuorum = c::sfSignerQuorum,
    CancelAfter = c::sfCancelAfter,
    FinishAfter = c::sfFinishAfter,
    SignerListID = c::sfSignerListID,
    SettleDelay = c::sfSettleDelay,
    TicketCount = c::sfTicketCount,
    TicketSequence = c::sfTicketSequence,
    NFTokenTaxon = c::sfNFTokenTaxon,
    MintedNFTokens = c::sfMintedNFTokens,
    BurnedNFTokens = c::sfBurnedNFTokens,
    HookStateCount = c::sfHookStateCount,
    EmitGeneration = c::sfEmitGeneration,
    LockCount = c::sfLockCount,
    RewardTime = c::sfRewardTime,
    RewardLgrFirst = c::sfRewardLgrFirst,
    RewardLgrLast = c::sfRewardLgrLast,
    IndexNext = c::sfIndexNext,
    IndexPrevious = c::sfIndexPrevious,
    BookNode = c::sfBookNode,
    OwnerNode = c::sfOwnerNode,
    BaseFee = c::sfBaseFee,
    ExchangeRate = c::sfExchangeRate,
    LowNode = c::sfLowNode,
    HighNode = c::sfHighNode,
    DestinationNode = c::sfDestinationNode,
    Cookie = c::sfCookie,
    ServerVersion = c::sfServerVersion,
    NFTokenOfferNode = c::sfNFTokenOfferNode,
    EmitBurden = c::sfEmitBurden,
    HookInstructionCount = c::sfHookInstructionCount,
    HookReturnCode = c::sfHookReturnCode,
    ReferenceCount = c::sfReferenceCount,
    RewardAccumulator = c::sfRewardAccumulator,
    EmailHash = c::sfEmailHash,
    TakerPaysCurrency = c::sfTakerPaysCurrency,
    TakerPaysIssuer = c::sfTakerPaysIssuer,
    TakerGetsCurrency = c::sfTakerGetsCurrency,
    TakerGetsIssuer = c::sfTakerGetsIssuer,
    LedgerHash = c::sfLedgerHash,
    ParentHash = c::sfParentHash,
    TransactionHash = c::sfTransactionHash,
    AccountHash = c::sfAccountHash,
    PreviousTxnID = c::sfPreviousTxnID,
    LedgerIndex = c::sfLedgerIndex,
    WalletLocator = c::sfWalletLocator,
    RootIndex = c::sfRootIndex,
    AccountTxnID = c::sfAccountTxnID,
    NFTokenID = c::sfNFTokenID,
    EmitParentTxnID = c::sfEmitParentTxnID,
    EmitNonce = c::sfEmitNonce,
    EmitHookHash = c::sfEmitHookHash,
    BookDirectory = c::sfBookDirectory,
    InvoiceID = c::sfInvoiceID,
    Nickname = c::sfNickname,
    Amendment = c::sfAmendment,
    HookOn = c::sfHookOn,
    Digest = c::sfDigest,
    Channel = c::sfChannel,
    ConsensusHash = c::sfConsensusHash,
    CheckID = c::sfCheckID,
    ValidatedHash = c::sfValidatedHash,
    PreviousPageMin = c::sfPreviousPageMin,
    NextPageMin = c::sfNextPageMin,
    NFTokenBuyOffer = c::sfNFTokenBuyOffer,
    NFTokenSellOffer = c::sfNFTokenSellOffer,
    HookStateKey = c::sfHookStateKey,
    HookHash = c::sfHookHash,
    HookNamespace = c::sfHookNamespace,
    HookSetTxnID = c::sfHookSetTxnID,
    OfferID = c::sfOfferID,
    EscrowID = c::sfEscrowID,
    URITokenID = c::sfURITokenID,
    Amount = c::sfAmount,
    Balance = c::sfBalance,
    LimitAmount = c::sfLimitAmount,
    TakerPays = c::sfTakerPays,
    TakerGets = c::sfTakerGets,
    LowLimit = c::sfLowLimit,
    HighLimit = c::sfHighLimit,
    Fee = c::sfFee,
    SendMax = c::sfSendMax,
    DeliverMin = c::sfDeliverMin,
    MinimumOffer = c::sfMinimumOffer,
    RippleEscrow = c::sfRippleEscrow,
    DeliveredAmount = c::sfDeliveredAmount,
    NFTokenBrokerFee = c::sfNFTokenBrokerFee,
    HookCallbackFee = c::sfHookCallbackFee,
    LockedBalance = c::sfLockedBalance,
    PublicKey = c::sfPublicKey,
    MessageKey = c::sfMessageKey,
    SigningPubKey = c::sfSigningPubKey,
    TxnSignature = c::sfTxnSignature,
    URI = c::sfURI,
    Signature = c::sfSignature,
    Domain = c::sfDomain,
    FundCode = c::sfFundCode,
    RemoveCode = c::sfRemoveCode,
    ExpireCode = c::sfExpireCode,
    CreateCode = c::sfCreateCode,
    MemoType = c::sfMemoType,
    MemoData = c::sfMemoData,
    MemoFormat = c::sfMemoFormat,
    Fulfillment = c::sfFulfillment,
    Condition = c::sfCondition,
    MasterSignature = c::sfMasterSignature,
    UNLModifyValidator = c::sfUNLModifyValidator,
    ValidatorToDisable = c::sfValidatorToDisable,
    ValidatorToReEnable = c::sfValidatorToReEnable,
    HookStateData = c::sfHookStateData,
    HookReturnString = c::sfHookReturnString,
    HookParameterName = c::sfHookParameterName,
    HookParameterValue = c::sfHookParameterValue,
    Blob = c::sfBlob,
    Account = c::sfAccount,
    Owner = c::sfOwner,
    Destination = c::sfDestination,
    Issuer = c::sfIssuer,
    Authorize = c::sfAuthorize,
    Unauthorize = c::sfUnauthorize,
    RegularKey = c::sfRegularKey,
    NFTokenMinter = c::sfNFTokenMinter,
    EmitCallback = c::sfEmitCallback,
    HookAccount = c::sfHookAccount,
    Indexes = c::sfIndexes,
    Hashes = c::sfHashes,
    Amendments = c::sfAmendments,
    NFTokenOffers = c::sfNFTokenOffers,
    HookNamespaces = c::sfHookNamespaces,
    Paths = c::sfPaths,
    TransactionMetaData = c::sfTransactionMetaData,
    CreatedNode = c::sfCreatedNode,
    DeletedNode = c::sfDeletedNode,
    ModifiedNode = c::sfModifiedNode,
    PreviousFields = c::sfPreviousFields,
    FinalFields = c::sfFinalFields,
    NewFields = c::sfNewFields,
    TemplateEntry = c::sfTemplateEntry,
    Memo = c::sfMemo,
    SignerEntry = c::sfSignerEntry,
    NFToken = c::sfNFToken,
    EmitDetails = c::sfEmitDetails,
    Hook = c::sfHook,
    Signer = c::sfSigner,
    Majority = c::sfMajority,
    DisabledValidator = c::sfDisabledValidator,
    EmittedTxn = c::sfEmittedTxn,
    HookExecution = c::sfHookExecution,
    HookDefinition = c::sfHookDefinition,
    HookParameter = c::sfHookParameter,
    HookGrant = c::sfHookGrant,
    Signers = c::sfSigners,
    SignerEntries = c::sfSignerEntries,
    Template = c::sfTemplate,
    Necessary = c::sfNecessary,
    Sufficient = c::sfSufficient,
    AffectedNodes = c::sfAffectedNodes,
    Memos = c::sfMemos,
    NFTokens = c::sfNFTokens,
    Hooks = c::sfHooks,
    Majorities = c::sfMajorities,
    DisabledValidators = c::sfDisabledValidators,
    HookExecutions = c::sfHookExecutions,
    HookParameters = c::sfHookParameters,
    HookGrants = c::sfHookGrants,
}

/// Data representation
#[derive(Clone, Copy)]
pub enum DataRepr {
    /// As UTF-8
    AsUTF8 = 0,
    /// As hexadecimal
    AsHex = 1,
}

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
//
/// This is simple version of Result type
/// to comply XRPL Hooks Webassembly restrictions
#[must_use]
pub enum Result<T> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(Error),
}

pub use self::Result::*;

impl<T> Result<T> {
    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// # Rollbacks
    ///
    /// Rollbacks if the value is an [`Err`], with a rollback message and error code.
    #[inline(always)]
    pub fn expect(self, msg: &[u8]) -> T {
        match self {
            Err(e) => rollback(msg, e.code() as _),
            Ok(val) => val,
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may rollback, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly.
    ///
    /// # Rollbacks
    ///
    /// Rollbacks if the value is an [`Err`], with a "error" and error code provided by the
    /// [`Err`]'s value.
    #[inline(always)]
    pub fn unwrap(self) -> T {
        match self {
            Err(e) => rollback(b"error", e.code() as _),
            Ok(val) => val,
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value,
    /// without checking that the value is not an [`Err`].
    ///
    /// # Safety
    ///
    /// Calling this method on an [`Err`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline(always)]
    pub unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Ok(val) => val,
            // SAFETY: the safety contract must be upheld by the caller.
            Err(_) => core::hint::unreachable_unchecked(),
        }
    }

    /// Similar to `unwrap`, but rollbacks with line number.
    /// This can be useful for fast debugging.
    #[inline(always)]
    pub fn unwrap_line_number(self) -> T {
        match self {
            Err(_) => rollback(b"error", line!().into()),
            Ok(val) => val,
        }
    }

    /// Returns `true` if the result is [`Ok`].
    #[must_use]
    #[inline(always)]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, Ok(_))
    }

    /// Returns `true` if the result is [`Err`].
    #[must_use]
    #[inline(always)]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

/// Possible errors returned by Hook APIs.
///
/// Errors are global across all Hook APIs.
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Error {
    /// A pointer or buffer length provided as a parameter described memory outside of the Hook's allowed memory region.
    OutOfBounds = c::OUT_OF_BOUNDS,
    /// Reserved for internal invariant trips, generally unrelated to inputs.
    /// These should be reported with an issue.
    InternalError = c::INTERNAL_ERROR,
    /// Attempted to set a parameter or value larger than the allowed space .
    TooBig = c::TOO_BIG,
    /// The API was unable to produce output to the write_ptr because the specified write_len was too small
    TooSmall = c::TOO_SMALL,
    /// The requested object or item wasn't found
    DoesntExist = c::DOESNT_EXIST,
    /// The Hook attempted to allocate an item into a slot, but there were no slots free.
    /// To avoid ensure re-use of existing slots. The maximum number of slots is 255.
    NoFreeSlots = c::NO_FREE_SLOTS,
    /// One or more of the parameters to the API were invalid according to the individual API's specification.
    InvalidArgument = c::INVALID_ARGUMENT,
    /// Some APIs allow for a once-per-execution parameter to be set.
    /// A second attempt to set a once-per-execution parameter results in this error.
    AlreadySet = c::ALREADY_SET,
    /// An API required the Hook to do something before the API is allowed to be called.
    /// Check the API's documentation.
    PrerequisiteNotMet = c::PREREQUISITE_NOT_MET,
    /// During fee calculation if an absurdly large fee is calculated this error is returned.
    FeeTooLarge = c::FEE_TOO_LARGE,
    /// An attempt to emit() a TXN was unsccessful for any of a number of reasons.
    /// Check the trace log of the rippled to which you are submitting the originating TXN.
    EmissionFailure = c::EMISSION_FAILURE,
    /// A Hook may only use up to 256 calls to nonce() per execution.
    /// Further calls result in this error code.
    TooManyNonces = c::TOO_MANY_NONCES,
    /// A Hook must declare ahead of time how many TXN it intends to emit().
    /// If it emits fewer than this many, this is allowed.
    /// If it emits more than this many this error is returned.
    TooManyEmittedTxn = c::TOO_MANY_EMITTED_TXN,
    /// While Hooks is/was in development an API may return this if some or all of that API is planned but not yet implemented.
    NotImplemented = c::NOT_IMPLEMENTED,
    /// An API which accepts a 20 byte Account ID may return this if, in its opinion, the Account ID was not valid for any reason.
    InvalidAccount = c::INVALID_ACCOUNT,
    /// All loops inside a Hook must declare at the top of the loop, as the first non trivial instruction,
    /// before any branch instruction, the promised maximum number of iterations of the loop.
    /// If this promise is violated the hook terminates immediately with this error code.
    GuardViolation = c::GUARD_VIOLATION,
    /// The requested serialized field could not be found in the specified object.
    InvalidField = c::INVALID_FIELD,
    /// While parsing serialized content an error was encountered (typically indicating an invalidly serialized object).
    ParseError = c::PARSE_ERROR,
    /// Used internally to communicate a rollback event.
    RcRollback = c::RC_ROLLBACK,
    /// Used internally to communicate an accept event.
    RcAccept = c::RC_ACCEPT,
    /// Specified keylet could not be found, or keylet is invalid
    NoSuchKeylet = c::NO_SUCH_KEYLET,
    /// API was asked to assume object under analysis is an STArray but it was not.
    NotAnArray = -22,
    /// API was asked to assume object under analysis is an STObject but it was not.
    NotAnObject = -23,
    /// A floating point operation resulted in Not-A-Number or API call attempted to specify an XFL floating point number outside of the expressible range of XFL.
    InvalidFloat = c::INVALID_FLOAT,
    /// API call would result in a division by zero, so API ended early.
    DivisionByZero = -25,
    /// When attempting to create an XFL the mantissa must be 16 decimal digits.
    ManitssaOversized = -26,
    /// When attempting to create an XFL the mantissa must be 16 decimal digits.
    MantissaUndersized = -27,
    /// When attempting to create an XFL the exponent must not exceed 80.
    ExponentOversized = -28,
    /// When attempting to create an XFL the exponent must not be less than -96.
    ExponentUndersized = -29,
    /// A floating point operation done on an XFL resulted in a value larger than XFL format is able to represent.
    Overflow = -30,
    /// An API assumed an STAmount was an IOU when in fact it was XRP.
    NotIouAmount = -31,
    /// An API assumed an STObject was an STAmount when in fact it was not.
    NotAnAmount = -32,
    /// An API would have returned a negative integer except that negative integers are reserved for error codes (i.e. what you are reading.)
    CantReturnNegative = -33,

    // Custom errors from hooks-rs
    InvalidKeyletLength = -500,
}

impl Error {
    #[inline(always)]
    fn from_code(code: i32) -> Self {
        unsafe { core::mem::transmute(code) }
    }

    /// Error code
    #[inline(always)]
    pub fn code(self) -> i32 {
        self as _
    }
}

type Api1ArgsU32 = unsafe extern "C" fn(u32) -> i64;
type Api2ArgsU32 = unsafe extern "C" fn(u32, u32) -> i64;
type Api3ArgsU32 = unsafe extern "C" fn(u32, u32, u32) -> i64;

type BufReader = Api2ArgsU32;
type BufWriter1Arg = Api3ArgsU32;

#[inline(always)]
fn api_1arg_call(arg: u32, fun: Api1ArgsU32) -> Result<u64> {
    let res = unsafe { fun(arg) };

    res.into()
}

#[inline(always)]
fn api_3arg_call(arg_1: u32, arg_2: u32, arg_3: u32, fun: Api3ArgsU32) -> Result<u64> {
    let res = unsafe { fun(arg_1, arg_2, arg_3) };

    res.into()
}

#[inline(always)]
fn buf_write_1arg(buf_write: &mut [u8], arg: u32, fun: BufWriter1Arg) -> Result<u64> {
    let res = unsafe { fun(buf_write.as_mut_ptr() as u32, buf_write.len() as u32, arg) };

    res.into()
}

#[inline(always)]
fn buf_read(buf: &[u8], fun: BufReader) -> Result<u64> {
    let res = unsafe { fun(buf.as_ptr() as u32, buf.len() as u32) };

    res.into()
}

#[inline(always)]
fn range_from_location(location: i64) -> core::ops::Range<usize> {
    let offset: i32 = (location >> 32) as _;
    let lenght: i32 = (location & 0xFFFFFFFF) as _;

    core::ops::Range {
        start: offset as _,
        end: (offset + lenght) as _,
    }
}

impl From<i64> for Result<u64> {
    #[inline(always)]
    fn from(res: i64) -> Self {
        match res {
            res if res >= 0 => Ok(res as _),
            _ => Err(Error::from_code(res as _)),
        }
    }
}

impl From<Error> for i64 {
    #[inline(always)]
    fn from(err: Error) -> Self {
        err as _
    }
}

impl From<AccountType> for u8 {
    #[inline(always)]
    fn from(account_type: AccountType) -> Self {
        account_type as _
    }
}

impl From<AmountType> for u8 {
    #[inline(always)]
    fn from(amount_type: AmountType) -> Self {
        amount_type as _
    }
}

impl From<TxnType> for u8 {
    #[inline(always)]
    fn from(transaction_type: TxnType) -> Self {
        transaction_type as _
    }
}

impl From<KeyletType> for u32 {
    #[inline(always)]
    fn from(keylet_type: KeyletType) -> Self {
        keylet_type as _
    }
}
