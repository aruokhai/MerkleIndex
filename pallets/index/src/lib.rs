#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
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
	pub type ActiveLinkProvider<T: Config> = StorageValue<_, StorageValue<_, T::AccountId, ValueQuery>; 

	#[pallet::storage]
	#[pallet::getter(fn active_link_patient)]
	pub type ActiveLinkPatient<T: Config> = StorageValue<_, StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn active_link)]
	pub type ActiveLink<T: Config> = StorageValue<_, StorageValue<_, bool, ValueQuery>;


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

		// An example dispatchable that may throw a custom error.
		// #[pallet::call_index(1)]
		// #[pallet::weight(T::WeightInfo::cause_error())]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}
