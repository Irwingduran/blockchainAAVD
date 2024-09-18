use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, dispatch,
    traits::Get,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Vote<AccountId> {
    folio: u32,
    candidate_id: u32,
    voter: AccountId,
    is_coalition: bool,
}

decl_storage! {
    trait Store for Module<T: Config> as VotingSystem {
        FolioUsed get(fn folio_used): map hasher(blake2_128_concat) u32 => bool;
        Votes get(fn votes): map hasher(blake2_128_concat) u32 => Vote<T::AccountId>;
        HasVoted get(fn has_voted): map hasher(blake2_128_concat) T::AccountId => bool;
        Validators get(fn validators): Vec<T::AccountId>;
        ValidatorCount get(fn validator_count): u32;
        VotingClosed get(fn voting_closed): bool;
        VotingStartTime get(fn voting_start_time): T::BlockNumber;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        VoteCast(u32, u32, AccountId, bool),
        VotingClosed(),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        NotValidator,
        VotingIsClosed,
        FolioAlreadyUsed,
        AlreadyVoted,
        InvalidCandidateId,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn cast_vote(origin, folio: u32, candidate_id: u32, is_coalition: bool) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Self::voting_closed(), Error::<T>::VotingIsClosed);
            ensure!(!Self::folio_used(folio), Error::<T>::FolioAlreadyUsed);
            ensure!(!Self::has_voted(&sender), Error::<T>::AlreadyVoted);
            ensure!(candidate_id > 0, Error::<T>::InvalidCandidateId);

            let vote = Vote {
                folio,
                candidate_id,
                voter: sender.clone(),
                is_coalition,
            };

            <FolioUsed>::insert(folio, true);
            <Votes<T>>::insert(folio, vote);
            <HasVoted<T>>::insert(&sender, true);

            Self::deposit_event(RawEvent::VoteCast(folio, candidate_id, sender, is_coalition));
            Ok(())
        }

        #[weight = 10_000]
        pub fn close_voting(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Self::is_validator(&sender), Error::<T>::NotValidator);

            let count = Self::validator_count().saturating_add(1);
            ValidatorCount::put(count);

            if count == Self::validators().len() as u32 {
                VotingClosed::put(true);
                Self::deposit_event(RawEvent::VotingClosed());
            }

            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    fn is_validator(address: &T::AccountId) -> bool {
        Self::validators().contains(address)
    }
}
