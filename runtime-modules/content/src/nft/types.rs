use super::*;

/// Metadata for vNFT issuance
pub type Metadata = Vec<u8>;

pub type CuratorGroupId<T> = <T as ContentActorAuthenticator>::CuratorGroupId;
pub type CuratorId<T> = <T as ContentActorAuthenticator>::CuratorId;
pub type MemberId<T> = <T as membership::Trait>::MemberId;

/// Owner royalty
pub type Royalty = Perbill;

/// Order details
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct OrderDetails<AccountId: Default, Balance: Default> {
    pub account_id: AccountId,
    pub price: Balance,
}

impl<AccountId: Default, Balance: Default> OrderDetails<AccountId, Balance> {
    /// Creates new `OrderDetails` instance
    pub fn new(account_id: AccountId, price: Balance) -> Self {
        Self { account_id, price }
    }
}

/// NFT transactional status
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum TransactionalStatus<
    AccountId: Default,
    BlockNumber: BaseArithmetic + Copy,
    MemberId: Default + Copy + Ord,
    Balance: Default,
> {
    Idle,
    InitiatedOfferToMember(MemberId, Option<OrderDetails<AccountId, Balance>>),
    Auction(AuctionRecord<AccountId, BlockNumber, Balance, MemberId>),
    BuyNow(OrderDetails<AccountId, Balance>),
}

impl<
        AccountId: Default,
        BlockNumber: BaseArithmetic + Copy,
        MemberId: Default + Copy + Ord,
        Balance: Default,
    > Default for TransactionalStatus<AccountId, BlockNumber, MemberId, Balance>
{
    fn default() -> Self {
        Self::Idle
    }
}

/// enum, representing NFT owner
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum NFTOwner<MemberId: Default + Copy, CuratorGroupId: Default + Copy, DAOId: Default + Copy> {
    ChannelOwner(ChannelOwner<MemberId, CuratorGroupId, DAOId>),
    Member(MemberId),
}

impl<MemberId: Default + Copy, CuratorGroupId: Default + Copy, DAOId: Default + Copy> Default
    for NFTOwner<MemberId, CuratorGroupId, DAOId>
{
    fn default() -> Self {
        Self::Member(MemberId::default())
    }
}

/// Owned vNFT representation
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct OwnedNFT<
    AccountId: Default,
    BlockNumber: BaseArithmetic + Copy,
    MemberId: Default + Copy + Ord,
    CuratorGroupId: Default + Copy,
    DAOId: Default + Copy,
    Balance: Default,
> {
    pub owner: NFTOwner<MemberId, CuratorGroupId, DAOId>,
    pub transactional_status: TransactionalStatus<AccountId, BlockNumber, MemberId, Balance>,
    pub creator_royalty: Option<Royalty>,
    // whether nft is issued
    pub is_issued: bool,
}

impl<
        AccountId: Default,
        BlockNumber: BaseArithmetic + Copy,
        MemberId: Default + Copy + PartialEq + Ord,
        CuratorGroupId: Default + Copy + PartialEq,
        DAOId: Default + Copy + PartialEq,
        Balance: Default,
    > OwnedNFT<AccountId, BlockNumber, MemberId, CuratorGroupId, DAOId, Balance>
{
    /// Whether content actor is nft owner
    pub fn is_owner(&self, channel_owner: &ChannelOwner<MemberId, CuratorGroupId, DAOId>) -> bool {
        match &self.owner {
            NFTOwner::ChannelOwner(owner) => owner.eq(channel_owner),
            NFTOwner::Member(member_id) => {
                if let ChannelOwner::Member(channel_owner_member_id) = channel_owner {
                    channel_owner_member_id == member_id
                } else {
                    false
                }
            }
        }
    }

    /// Create new vNFT
    pub fn new(
        owner: ChannelOwner<MemberId, CuratorGroupId, DAOId>,
        creator_royalty: Option<Royalty>,
    ) -> Self {
        Self {
            owner: NFTOwner::ChannelOwner(owner),
            transactional_status: TransactionalStatus::Idle,
            creator_royalty,
            is_issued: false,
        }
    }
}

/// Enum, representing nft issuance status
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum NFTStatus<
    AccountId: Default,
    BlockNumber: BaseArithmetic + Copy,
    MemberId: Default + Copy + Ord,
    CuratorGroupId: Default + Copy,
    DAOId: Default + Copy,
    Balance: Default,
> {
    NoneIssued,
    Owned(OwnedNFT<AccountId, BlockNumber, MemberId, CuratorGroupId, DAOId, Balance>),
}

impl<
        AccountId: Default,
        BlockNumber: BaseArithmetic + Copy,
        MemberId: Default + Copy + Ord,
        CuratorGroupId: Default + Copy,
        DAOId: Default + Copy,
        Balance: Default,
    > Default for NFTStatus<AccountId, BlockNumber, MemberId, CuratorGroupId, DAOId, Balance>
{
    fn default() -> Self {
        Self::NoneIssued
    }
}

/// Either new auction, which requires vNFT issance or auction for already existing nft.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum AuctionMode {
    // Auction, where nft issued at the end
    WithIssuance(Metadata),
    // Auction for already existing nft
    WithoutIsuance,
}

impl Default for AuctionMode {
    fn default() -> Self {
        Self::WithoutIsuance
    }
}

/// Information on the auction being created.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Bid<AccountId, BlockNumber: BaseArithmetic + Copy, Balance> {
    pub bidder: AccountId,
    pub amount: Balance,
    pub time: BlockNumber,
}

impl<AccountId, BlockNumber: BaseArithmetic + Copy, Balance> Bid<AccountId, BlockNumber, Balance> {
    fn new(bidder: AccountId, amount: Balance, time: BlockNumber) -> Self {
        Self {
            bidder,
            amount,
            time,
        }
    }
}

/// Information on the auction being created.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct AuctionRecord<AccountId, BlockNumber: BaseArithmetic + Copy, Balance, MemberId: Ord> {
    pub starting_price: Balance,
    pub buy_now_price: Option<Balance>,
    pub auction_duration: BlockNumber,
    pub minimal_bid_step: Balance,
    pub last_bid: Option<Bid<AccountId, BlockNumber, Balance>>,
    pub starts_at: Option<BlockNumber>,
    pub whitelist: Option<BTreeSet<MemberId>>,
}

impl<
        AccountId: Default + PartialEq,
        BlockNumber: BaseArithmetic + Copy + Default,
        Balance: Default + BaseArithmetic,
        MemberId: Default + PartialEq + Ord,
    > AuctionRecord<AccountId, BlockNumber, Balance, MemberId>
{
    /// Create a new auction record with provided parameters
    pub fn new<VideoId>(
        auction_params: AuctionParams<VideoId, BlockNumber, Balance, MemberId>,
    ) -> Self {
        let AuctionParams {
            auction_duration,
            starting_price,
            buy_now_price,
            minimal_bid_step,
            starts_at,
            whitelist,
            ..
        } = auction_params;
        Self {
            starting_price,
            buy_now_price,
            auction_duration,
            minimal_bid_step,
            last_bid: None,
            starts_at,
            whitelist,
        }
    }

    /// Ensure new bid is greater then last bid + minimal bid step
    pub fn ensure_is_valid_bid<T: Trait>(&self, new_bid: Balance) -> DispatchResult {
        // Always allow to buy now
        match &self.buy_now_price {
            Some(buy_now_price) if new_bid >= *buy_now_price => (),
            // Ensure new bid is greater then last bid + minimal bid step
            _ => {
                if let Some(last_bid) = &self.last_bid {
                    ensure!(
                        last_bid
                            .amount
                            .checked_add(&self.minimal_bid_step)
                            .ok_or(Error::<T>::OverflowOrUnderflowHappened)?
                            < new_bid,
                        Error::<T>::InvalidBid
                    );
                } else {
                    ensure!(self.minimal_bid_step < new_bid, Error::<T>::InvalidBid);
                }
            }
        }

        Ok(())
    }

    /// Make auction bid
    pub fn make_bid(&mut self, who: AccountId, bid: Balance, last_bid_block: BlockNumber) {
        let bid = Bid::new(who, bid, last_bid_block);
        self.last_bid = Some(bid);
    }

    /// Check whether auction have any bids
    fn is_active(&self) -> bool {
        self.last_bid.is_some()
    }

    /// Ensure auction is not active
    pub fn ensure_is_not_active<T: Trait>(&self) -> DispatchResult {
        ensure!(self.is_active(), Error::<T>::ActionIsAlreadyActive);
        Ok(())
    }

    /// Ensure auction have been already started
    pub fn ensure_auction_started<T: Trait>(&self, current_block: BlockNumber) -> DispatchResult {
        if let Some(starts_at) = self.starts_at {
            ensure!(starts_at <= current_block, Error::<T>::AuctionDidNotStart);
        }
        Ok(())
    }

    /// Check whether auction expired
    pub fn is_nft_auction_expired(&self, now: BlockNumber) -> bool {
        match &self.last_bid {
            Some(last_bid) => {
                // Check whether auction round time expired.
                let is_auction_round_expired = (now - last_bid.time) >= self.auction_duration;

                // Check whether buy now have been triggered.
                let is_buy_now_triggered =
                    matches!(&self.buy_now_price, Some(buy_now) if *buy_now == last_bid.amount);
                is_auction_round_expired || is_buy_now_triggered
            }
            _ => false,
        }
    }

    /// Ensure caller is auction winner.
    pub fn ensure_caller_is_auction_winner<T: Trait>(&self, who: AccountId) -> DispatchResult {
        ensure!(
            matches!(&self.last_bid, Some(last_bid) if last_bid.bidder == who),
            Error::<T>::CallerIsNotAWinner
        );
        Ok(())
    }

    /// If whitelist set, ensure provided member is authorized to make bids
    pub fn ensure_whitelisted_participant<T: Trait>(&self, who: MemberId) -> DispatchResult {
        if let Some(whitelist) = &self.whitelist {
            ensure!(
                whitelist.contains(&who),
                Error::<T>::MemberIsNotAllowedToParticipate
            );
        }
        Ok(())
    }
}

/// Auction alias type for simplification.
pub type Auction<T> = AuctionRecord<
    <T as frame_system::Trait>::AccountId,
    <T as frame_system::Trait>::BlockNumber,
    BalanceOf<T>,
    MemberId<T>,
>;

/// Parameters, needed for auction start
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct AuctionParams<VideoId, BlockNumber, Balance, MemberId: Ord> {
    pub video_id: VideoId,
    /// Should only be provided if nft is not issued yet
    pub creator_royalty: Option<Royalty>,
    pub auction_duration: BlockNumber,
    pub starting_price: Balance,
    pub minimal_bid_step: Balance,
    pub buy_now_price: Option<Balance>,
    pub starts_at: Option<BlockNumber>,
    pub whitelist: Option<BTreeSet<MemberId>>,
}
