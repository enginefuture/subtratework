#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
        dispatch::DispatchResultWithPostInfo, 
        pallet_prelude::*
    };
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// pub type Something<T> = StorageValue<_, u32>;
    #[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
        _, 
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
    >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	// #[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	// pub enum Event<T: Config> {
	// 	/// Event documentation should end with an array that provides descriptive names for event
	// 	/// parameters. [something, who]
	// 	SomethingStored(u32, T::AccountId),
	// }

    
    #[pallet::event]
    //#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimMoved(T::AccountId, T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	// #[pallet::error]
	// pub enum Error<T> {
	// 	/// Error names should be descriptive.
	// 	NoneValue,
	// 	/// Errors should have helpful documentation associated with them.
	// 	StorageOverflow,
	// }

    #[pallet::error]
	pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        DestinationIsClaimOwner
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        
    }

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
            Proofs::<T>::insert(
                &claim,
                (sender.clone(), <frame_system::Pallet::<T>>::block_number()),
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            destination: T::AccountId,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            ensure!(owner != destination, Error::<T>::DestinationIsClaimOwner);
            Proofs::<T>::remove(&claim);
            Proofs::<T>::insert(&claim,(destination.clone(), <frame_system::Pallet::<T>>::block_number()),);
            Self::deposit_event(Event::ClaimMoved(sender, destination, claim));
            Ok(().into())
        }
    }
}
