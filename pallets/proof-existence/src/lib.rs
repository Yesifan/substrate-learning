#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

  #[pallet::config] // <-- Step 2.
  pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  }
  #[pallet::event] // <-- Step 3.
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Event emitted when a proof has been claimed. [who, claim]
    ClaimCreated(T::AccountId, Vec<u8>),
    /// Event emitted when a claim is revoked by the owner. [who, claim]
    ClaimRevoked(T::AccountId, Vec<u8>),
  }
  #[pallet::error] // <-- Step 4.
  pub enum Error<T> {
    /// The proof has already been claimed.
    ProofAlreadyClaimed,
    /// The proof does not exist, so it cannot be revoked.
    NoSuchProof,
    /// The proof is claimed by another account, so caller can't revoke it.
    NotProofOwner,
  }
  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  // #[pallet::generate_storage_info]
  // https://github.com/substrate-developer-hub/substrate-docs/pull/587
  pub struct Pallet<T>(_);

  #[pallet::storage] // <-- Step 5.
  pub(super) type Proofs<T: Config> =
    StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call] // <-- Step 6.
  // Dispatchable functions allow users to interact with the pallet and invoke state changes.
  // These functions materialize as "extrinsics", which are often compared to transactions.
  // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
  impl<T: Config> Pallet<T> {
    #[pallet::weight(1_000)]
    pub fn create_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult {
      // Check that the extrinsic was signed and get the signer.
      // This function will return an error if the extrinsic is not signed.
      // https://docs.substrate.io/v3/runtime/origins
      let sender = ensure_signed(origin)?;
      // Verify that the specified proof has not already been claimed.
      ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
      // Get the block number from the FRAME System pallet.
      let current_block = <frame_system::Pallet<T>>::block_number();
      // Store the proof with the sender and block number.
      Proofs::<T>::insert(&proof, (&sender, current_block));
      // Emit an event that the claim was created.
      Self::deposit_event(Event::ClaimCreated(sender, proof));
      Ok(())
    }
    #[pallet::weight(10_000)]
    pub fn revoke_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult {
      // Check that the extrinsic was signed and get the signer.
      // This function will return an error if the extrinsic is not signed.
      // https://docs.substrate.io/v3/runtime/origins
      let sender = ensure_signed(origin)?;
      // Verify that the specified proof has been claimed.
      ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
      // Get owner of the claim.
      let (owner, _) = Proofs::<T>::get(&proof);
      // Verify that sender of the current call is the claim owner.
      ensure!(sender == owner, Error::<T>::NotProofOwner);
      // Remove claim from storage.
      Proofs::<T>::remove(&proof);
      // Emit an event that the claim was erased.
      Self::deposit_event(Event::ClaimRevoked(sender, proof));
      Ok(())
    }
  }
}
