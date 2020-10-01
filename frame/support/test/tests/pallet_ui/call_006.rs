#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{ModuleInterface, DispatchResultWithPostInfo};
	use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};

	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Module<T>(core::marker::PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Module<T> {
		fn foo(origin: OriginFor<T>) -> DispatchResultWithPostInfo {}
	}
}

fn main() {
}
