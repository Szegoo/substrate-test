#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
	use frame_support::dispatch::HasCompact;
	use frame_support::dispatch::EncodeLike;

	#[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
	
	#[pallet::config]
    pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo
			+ EncodeLike;
		type MaxName: Get<u32>;
	}

	#[pallet::storage]
	pub(super) type Assets<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
		T::AssetId,
		(T::AccountId, BoundedVec<u8, T::MaxName>)
    >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NftMinted(T::AssetId, T::AccountId),
		NftTransferred(T::AssetId, T::AccountId, T::AccountId)
	}

	#[pallet::error]
	pub enum Error<T> {
		InUse,
		Unauthorized
	}

	#[pallet::call]		
	impl <T:Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn mint(origin: OriginFor<T>, id: T::AssetId, name: BoundedVec<u8, T::MaxName>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Assets::<T>::contains_key(&id), Error::<T>::InUse);

			Assets::<T>::insert(&id, (&sender, name));

			Self::deposit_event(Event::<T>::NftMinted(id, sender));
			Ok(())
		}

		#[pallet::weight(500)]
		pub fn transfer(origin: OriginFor<T>, id: T::AssetId, benefiary: T::AccountId)
		-> DispatchResult {
			let sender = ensure_signed(origin)?;
			let (owner, name) = Assets::<T>::get(&id)
				.expect("Every asset must have an owner and a name!");
			ensure!(owner == sender, Error::<T>::Unauthorized);

			Assets::<T>::remove(&id);
			Assets::<T>::insert(&id, (&benefiary, name));

			Self::deposit_event(Event::NftTransferred(id, owner, benefiary));

			Ok(())
		} 
	}
}