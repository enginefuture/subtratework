```rust
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
```