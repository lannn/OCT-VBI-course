#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct Tweet {
		id: u8,
		user_id: u8,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Tweet),
		ClaimRevoked(T::AccountId, Tweet),
		Tranferred(T::AccountId, T::AccountId, Tweet),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Tweet, (T::AccountId, T::BlockNumber), ValueQuery>;
	
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_claim(origin: OriginFor<T>, proof: Tweet) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();

			Proofs::<T>::insert(&proof, (&sender, current_block));

			Self::deposit_event(Event::ClaimCreated(sender, proof));
			
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(origin: OriginFor<T>, proof: Tweet) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof);

			ensure!(sender == owner, Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);

			Self::deposit_event(Event::ClaimRevoked(sender, proof));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn tranfer(origin: OriginFor<T>, to: T::AccountId, proof: Tweet) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			
			let (owner, _) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			
			let current_block = <frame_system::Pallet<T>>::block_number();
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			// tranfer
			Proofs::<T>::remove(&proof);
			Proofs::<T>::insert(&proof, (&to, current_block));

			Self::deposit_event(Event::Tranferred(sender, to, proof));

			Ok(())
		}
	}
}
