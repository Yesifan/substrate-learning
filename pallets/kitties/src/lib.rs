#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::{
    dispatch::{DispatchResult, DispatchResultWithPostInfo},
    pallet_prelude::*,
    sp_runtime::traits::{Hash, Zero},
    traits::{Currency, ExistenceRequirement, Randomness},
  };
  use frame_system::pallet_prelude::*;
  use sp_io::hashing::blake2_128;

  // TODO Part II: 用于保存 Kitty 信息的 struct

  // TODO Part II: 在 Kitty struct 使用 Enum and implementation 来处理 Gender type.

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  #[pallet::generate_storage_info]
  pub struct Pallet<T>(_);

  /// 通过指定 pallet 所依赖的参数和类型来配置 pallet
  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// The Currency handler for the Kitties pallet.
    type Currency: Currency<Self::AccountId>;

    // TODO Part II: Specify the custom types for our runtime.
  }

  // Errors.
  #[pallet::error]
  pub enum Error<T> {
    // TODO Part III
  }

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    // TODO Part III
  }

  // ACTION: Storage item to keep a count of all existing Kitties.

  // TODO Part II: Remaining storage items.

  // TODO Part III: Our pallet's genesis configuration.

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    // TODO Part III: create_kitty

    // TODO Part III: set_price

    // TODO Part III: transfer

    // TODO Part III: buy_kitty

    // TODO Part III: breed_kitty
  }

  // TODO Parts II: helper function for Kitty struct

  impl<T: Config> Pallet<T> {
    // TODO Part III: helper functions for dispatchable functions

    // TODO: increment_nonce, random_hash, mint, transfer_from
  }
}
