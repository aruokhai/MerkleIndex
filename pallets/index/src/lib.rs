#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::traits::Get;
use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
	pallet_prelude::BlockNumberFor,
};
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	traits::Zero,
	transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	RuntimeDebug,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::{*, OptionQuery};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_std::vec::*;
	use frame_support::sp_std::collections::btree_map::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn providers)]
	pub type Provider<T: Config> = StorageMap<_,Blake2_128Concat,T::AccountId, BTreeMap<u32, Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn patients)]
	pub type Patient<T: Config> = StorageMap<_,Blake2_128Concat,T::AccountId, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn active_link_provider)]
	pub type ActiveLinkProvider<T: Config> = StorageValue<_, T::AccountId, OptionQuery>; 

	#[pallet::storage]
	#[pallet::getter(fn active_link_patient)]
	pub type ActiveLinkPatient<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProviderAdded { who: T::AccountId },
		PatientAdded { who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Patient Linking
		PatientLinking,
		NotPatientLinking
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			let node_address = StorageValueRef::persistent(b"index::node-address");
 			if let Ok(Some(address)) = node_address.get::<T::AccountId>() {
				let result = Self::add_provider_oauth_client(address);
 			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn register_provider(origin: OriginFor<T>, public_key: Vec<u8>, ip_address: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if <Provider<T>>::contains_key(who.clone()) {
				Self::deposit_event(Event::ProviderAdded{ who: who.clone() });
				return Ok(())
			}
			<Provider<T>>::set(who.clone(), BTreeMap::from([
				(0, public_key),
				(1, ip_address),]));
			Self::deposit_event(Event::ProviderAdded { who });
			Ok(())
			
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn register_patient(origin: OriginFor<T>, public_key: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if <Patient<T>>::contains_key(who.clone()) {
				Self::deposit_event(Event::PatientAdded{ who: who.clone() });
				return Ok(())
			}
			<Patient<T>>::set(who.clone(), public_key);
			Self::deposit_event(Event::PatientAdded { who });
			Ok(())
	
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn link_provider_patient(origin: OriginFor<T>, provider_address: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if  !<Patient<T>>::contains_key(who.clone()) {
				return Err(Error::<T>::NotPatientLinking.into())
			}
			match <ActiveLinkPatient<T>>::get() {
				Some(_) => return Err(Error::<T>::PatientLinking.into()),
				None => {
					<ActiveLinkPatient<T>>::put(who);
					<ActiveLinkProvider<T>>::put(provider_address);
					Ok(())
				}
			}
		}
		
	}

	impl<T: Config> Pallet<T> {

		fn add_provider_oauth_client(node_address: T::AccountId) -> Result<(), &'static str> {
			let active_patient_link = <ActiveLinkPatient<T>>::get().ok_or("No patient")?;
		}
	}

}
