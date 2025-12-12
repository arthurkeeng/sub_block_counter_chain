

#![cfg_attr(not(feature = "std") , no_std)]


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet{
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
}

#[pallet::config]
pub trait Config : frame_system::Config{
	type RuntimeEvent : From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}

#[pallet::pallet]
pub struct Pallet<T>(_);


#[pallet::storage]
#[pallet::getter(fn count)]
pub type Count<T> = StorageValue<_, u32 , ValueQuery>;

#[pallet::storage]
pub type Admin<T: Config> = StorageValue<_, T::AccountId , OptionQuery>;

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T : Config>{
	Incremented(u32 , T::AccountId), 
	Reset(T::AccountId), 
	AdminChanged(T::AccountId, T::AccountId)
}

#[pallet::error]
pub enum Error<T>{
	Overflow,
	AlreadyZero, 
	NotAdmin,
}



#[pallet::call]
impl<T : Config> Pallet<T>{
	#[pallet::call_index(0)]
	#[pallet::weight(10_000)]
	pub fn increment (origin : OriginFor<T>) -> DispatchResult{
		let who = ensure_signed(origin)?;
		let current_count = Count::<T>::get();

		let new_count = current_count.checked_add(1)
				.ok_or(Error::<T>::Overflow)?;
		let current_block = frame_system::Pallet::<T>::block_number();

		Count>><T>::put(new_count);
		Self::deposit_event(Event::Incremented(,new_count , who));
		let current_block = frame_system::Pallet::<T>::block_number();
		Ok(())
	}
	#[pallet::call_index(1)]
	#[pallet::weight(10_000)]
	pub fn reset (origin : OriginFor<T>) -> DispatchResult{
		let who = ensure_signed(origin);

		let admin = Admin::<T>::get().ok_or(Error::<T>::NotAdmin)?;
		ensure!(who == admin, Error::<T>::NotAdmin);

		let current = Count::<T>::get();
		ensure!(current > 0 , Error::<T>::AlreadyZero);
		
		Count::<T>::put(0);
		Self::deposit_event(Event::Reset(who));
		
		
		Ok(())

	}
	#[pallet::call_index(2)]
	#[pallet::weight(10_000)]
	pub fn set_admin(origin : OriginFor<T> , admin : T::AccountId) -> DispatchResult{

		let who = ensure_signed(origin)?;

		if let Some(current_admin) = Admin::<T>::get(){
			ensure!(who == current_admin , Error::<T>::NotAdmin);
		}

		Admin::<T>::put(admin.clone());
		Self::deposit_event(Event::AdminChanged(who , admin));
	}
}