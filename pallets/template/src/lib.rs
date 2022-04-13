#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // The struct on which we build all of our Pallet logic.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /* Placeholder for defining custom types. */

    // TODO: Update the `config` block below
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type MaxBytesInHash: Get<u32>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
	}

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
    }

    #[pallet::storage]
    pub(super) type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxBytesInHash>,
        (T::AccountId, T::BlockNumber),
        OptionQuery
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_0000)]
        pub fn create_claim(
            origin: OriginFor<T>,
            proof: BoundedVec<u8, T::MaxBytesInHash>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?; 

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <frame_system::Pallet<T>>::block_number();

            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(Event::ClaimCreated(sender, proof));

            Ok(())
        }

        #[pallet::weight(10_0000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            proof: BoundedVec<u8, T::MaxBytesInHash>
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            let (owner, _) = Proofs::<T>::get(&proof)
                .expect("All proof need to have an owner and block number!");
            ensure!(owner == sender, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(Event::ClaimRevoked(owner, proof));

            Ok(())
        }
    }
}