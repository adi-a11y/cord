// CORD Chain Node – https://cord.network
// Copyright (C) 2019-2022 Dhiway
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use cord_primitives::{IdentifierOf, StatusOf};
use frame_support::{ensure, storage::types::StorageMap};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_std::{fmt::Debug, prelude::Clone, str, vec::Vec};

pub mod streams;
pub mod weights;

pub use crate::streams::*;

use crate::weights::WeightInfo;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Hash of the Stream.
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	/// Type of the controller.
	pub type CordAccountOf<T> = <T as frame_system::Config>::AccountId;
	// stream identifier prefix.
	pub const STREAM_IDENTIFIER_PREFIX: u16 = 43;
	/// Type for a block number.
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	/// Type for a block time.
	pub type SignatureOf<T> = <T as Config>::Signature;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_schema::Config {
		type EnsureOrigin: EnsureOrigin<
			Success = CordAccountOf<Self>,
			<Self as frame_system::Config>::Origin,
		>;
		type ForceOrigin: EnsureOrigin<Self::Origin>;
		type Signature: Verify<Signer = <Self as pallet::Config>::Signer> + Parameter;
		type Signer: IdentifyAccount<AccountId = CordAccountOf<Self>> + Parameter;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// streams stored on chain.
	/// It maps from stream Id to its details.
	#[pallet::storage]
	#[pallet::getter(fn streams)]
	pub type Streams<T> =
		StorageMap<_, Blake2_128Concat, IdentifierOf, StreamDetails<T>, OptionQuery>;

	/// stream commit details stored on chain.
	/// It maps from a stream Id to a vector of commit details.
	#[pallet::storage]
	#[pallet::getter(fn commits)]
	pub type Commits<T> =
		StorageMap<_, Blake2_128Concat, IdentifierOf, Vec<StreamCommit<T>>, OptionQuery>;

	/// stream hashes stored on chain.
	/// It maps from a stream hash to Id (resolve from hash).
	#[pallet::storage]
	#[pallet::getter(fn hashes_of)]
	pub type HashesOf<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, IdentifierOf, OptionQuery>;

	/// stream digest stored on chain.
	/// It maps from a stream digest hash to Id (resolve from hash).
	#[pallet::storage]
	#[pallet::getter(fn digest_of)]
	pub type DigestOf<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, IdentifierOf, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new stream has been created.
		/// \[stream hash, identifier, controller\]
		Anchor(HashOf<T>, IdentifierOf, CordAccountOf<T>),
		/// A stream has been updated.
		/// \[stream identifier, hash, controller\]
		Update(IdentifierOf, HashOf<T>, CordAccountOf<T>),
		/// A stream digest has been added.
		/// \[stream identifier, hash, controller\]
		Digest(IdentifierOf, HashOf<T>, CordAccountOf<T>),
		/// A stream status has been changed.
		/// \[stream identifier, controller\]
		Status(IdentifierOf, CordAccountOf<T>),
		/// A stream has been removed.
		/// \[stream identifier\]
		Remove(IdentifierOf),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Stream idenfier is not unique
		StreamAlreadyAnchored,
		/// Stream idenfier not found
		StreamNotFound,
		/// Stream idenfier marked inactive
		StreamRevoked,
		/// No stream status change required
		StatusChangeNotRequired,
		/// Only when the author is not the controller/delegate.
		UnauthorizedOperation,
		/// Stream link does not exist
		StreamLinkNotFound,
		/// Stream Link is revoked
		StreamLinkRevoked,
		// Invalid creator signature
		InvalidSignature,
		//Stream has is not unique
		HashAlreadyAnchored,
		// Expired Tx Signature
		ExpiredSignature,
		// Invalid Stream Identifier
		InvalidIdentifier,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new stream and associates it with its controller.
		///
		/// * origin: the identity of the Tx Author.
		/// * creator: creator (controller) of the stream.
		/// * stream_hash: hash of the incoming stream.
		/// * holder: \[OPTIONAL\] holder (recipient) of the stream.
		/// * schema: \[OPTIONAL\] stream schema identifier.
		/// * link: \[OPTIONAL\] stream link identifier.
		/// * tx_signature: creator signature.
		#[pallet::weight(52_000 + T::DbWeight::get().reads_writes(4, 3))]
		pub fn create(
			origin: OriginFor<T>,
			creator: CordAccountOf<T>,
			stream_hash: HashOf<T>,
			holder: Option<CordAccountOf<T>>,
			schema: Option<IdentifierOf>,
			link: Option<IdentifierOf>,
			tx_signature: SignatureOf<T>,
		) -> DispatchResult {
			<T as Config>::EnsureOrigin::ensure_origin(origin)?;
			ensure!(
				tx_signature.verify(&(&stream_hash).encode()[..], &creator),
				Error::<T>::InvalidSignature
			);

			let identifier: IdentifierOf = pallet_schema::schemas::create_identifier(
				&(&stream_hash).encode()[..],
				STREAM_IDENTIFIER_PREFIX,
			)
			.into_bytes();

			ensure!(!<Streams<T>>::contains_key(&identifier), Error::<T>::StreamAlreadyAnchored);

			if let Some(ref schema) = schema {
				pallet_schema::SchemaDetails::<T>::schema_status(schema, creator.clone())
					.map_err(<pallet_schema::Error<T>>::from)?;
			}

			if let Some(ref link) = link {
				let link_details =
					<Streams<T>>::get(&link).ok_or(Error::<T>::StreamLinkNotFound)?;
				ensure!(!link_details.revoked, Error::<T>::StreamLinkRevoked);
			}

			let now_block_number = frame_system::Pallet::<T>::block_number();
			<HashesOf<T>>::insert(&stream_hash, &identifier);

			StreamCommit::<T>::store_tx(
				&identifier,
				StreamCommit { block: now_block_number, commit: StreamCommitOf::Genesis },
			)?;

			<Streams<T>>::insert(
				&identifier,
				StreamDetails {
					stream_hash: stream_hash.clone(),
					controller: creator.clone(),
					holder,
					schema,
					link,
					revoked: false,
				},
			);

			Self::deposit_event(Event::Anchor(stream_hash, identifier, creator));

			Ok(())
		}
		/// Updates the stream information.
		///
		/// * origin: the identity of the Tx Author.
		/// * identifier: unique identifier of the incoming stream.
		/// * updater: controller or delegate of the stream.
		/// * stream_hash: hash of the incoming stream.
		/// * tx_signature: signature of the controller.
		#[pallet::weight(50_000 + T::DbWeight::get().reads_writes(2, 4))]
		pub fn update(
			origin: OriginFor<T>,
			identifier: IdentifierOf,
			updater: CordAccountOf<T>,
			stream_hash: HashOf<T>,
			tx_signature: SignatureOf<T>,
		) -> DispatchResult {
			<T as Config>::EnsureOrigin::ensure_origin(origin)?;
			ensure!(!<HashesOf<T>>::contains_key(&stream_hash), Error::<T>::HashAlreadyAnchored);
			pallet_schema::SchemaDetails::<T>::is_valid_identifier(
				&identifier,
				STREAM_IDENTIFIER_PREFIX,
			)
			.map_err(|_| Error::<T>::InvalidIdentifier)?;

			let tx_prev_details =
				<Streams<T>>::get(&identifier).ok_or(Error::<T>::StreamNotFound)?;
			ensure!(!tx_prev_details.revoked, Error::<T>::StreamRevoked);

			ensure!(
				tx_signature.verify(&(&stream_hash).encode()[..], &updater),
				Error::<T>::InvalidSignature
			);

			if let Some(ref schema) = tx_prev_details.schema {
				pallet_schema::SchemaDetails::<T>::schema_status(schema, updater.clone())
					.map_err(<pallet_schema::Error<T>>::from)?;
			}

			let now_block_number = frame_system::Pallet::<T>::block_number();

			StreamCommit::<T>::store_tx(
				&identifier,
				StreamCommit { block: now_block_number, commit: StreamCommitOf::Update },
			)?;
			<HashesOf<T>>::insert(&stream_hash, &identifier);

			<Streams<T>>::insert(
				&identifier,
				StreamDetails {
					controller: updater.clone(),
					stream_hash: stream_hash.clone(),
					..tx_prev_details
				},
			);
			Self::deposit_event(Event::Update(identifier, stream_hash, updater));

			Ok(())
		}
		/// Update the status of the stream
		///
		/// * origin: the identity of the Tx Author.
		/// * identifier: unique identifier of the stream.
		/// * updater: controller or delegate of the stream.
		/// * status: stream revocation status (bool).
		/// * tx_hash: transaction hash.
		/// * tx_signature: signature of the contoller.
		#[pallet::weight(30_000 + T::DbWeight::get().reads_writes(2, 3))]
		pub fn status(
			origin: OriginFor<T>,
			identifier: IdentifierOf,
			updater: CordAccountOf<T>,
			status: StatusOf,
			tx_hash: HashOf<T>,
			tx_signature: SignatureOf<T>,
		) -> DispatchResult {
			<T as Config>::EnsureOrigin::ensure_origin(origin)?;
			pallet_schema::SchemaDetails::<T>::is_valid_identifier(
				&identifier,
				STREAM_IDENTIFIER_PREFIX,
			)
			.map_err(|_| Error::<T>::InvalidIdentifier)?;
			let tx_status = <Streams<T>>::get(&identifier).ok_or(Error::<T>::StreamNotFound)?;
			ensure!(tx_status.revoked != status, Error::<T>::StatusChangeNotRequired);
			ensure!(
				tx_signature.verify(&(&tx_hash).encode()[..], &updater),
				Error::<T>::InvalidSignature
			);
			if let Some(ref schema) = tx_status.schema {
				pallet_schema::SchemaDetails::<T>::schema_status(schema, updater.clone())
					.map_err(<pallet_schema::Error<T>>::from)?;
			}
			let now_block_number = frame_system::Pallet::<T>::block_number();

			StreamCommit::<T>::store_tx(
				&identifier,
				StreamCommit { block: now_block_number, commit: StreamCommitOf::Status },
			)?;
			<Streams<T>>::insert(
				&identifier,
				StreamDetails { controller: updater.clone(), revoked: status, ..tx_status },
			);
			Self::deposit_event(Event::Status(identifier, updater));

			Ok(())
		}
		///  Remove a stream from the chain.
		///
		/// * origin: the identity of the council origin.
		/// * identifier: unique identifier of the incoming stream.
		#[pallet::weight(52_000 + T::DbWeight::get().reads_writes(3, 3))]
		pub fn remove(origin: OriginFor<T>, identifier: IdentifierOf) -> DispatchResult {
			<T as Config>::ForceOrigin::ensure_origin(origin)?;
			pallet_schema::SchemaDetails::<T>::is_valid_identifier(
				&identifier,
				STREAM_IDENTIFIER_PREFIX,
			)
			.map_err(|_| Error::<T>::InvalidIdentifier)?;
			<Streams<T>>::get(&identifier).ok_or(Error::<T>::StreamNotFound)?;

			let now_block_number = frame_system::Pallet::<T>::block_number();

			<Streams<T>>::remove(&identifier);
			StreamCommit::<T>::store_tx(
				&identifier,
				StreamCommit { block: now_block_number, commit: StreamCommitOf::Remove },
			)?;
			Self::deposit_event(Event::Remove(identifier));

			Ok(())
		}
		/// Adds stream digest information.
		///
		/// * origin: the identity of the Tx Author.
		/// * identifier: unique identifier of the incoming stream.
		/// * creator: controller or delegate of the stream.
		/// * digest_hash: hash of the incoming stream.
		/// * tx_signature: signature of the controller.
		#[pallet::weight(30_000 + T::DbWeight::get().reads_writes(2, 1))]
		pub fn digest(
			origin: OriginFor<T>,
			identifier: IdentifierOf,
			creator: CordAccountOf<T>,
			digest_hash: HashOf<T>,
			tx_signature: SignatureOf<T>,
		) -> DispatchResult {
			<T as Config>::EnsureOrigin::ensure_origin(origin)?;
			ensure!(!<DigestOf<T>>::contains_key(&digest_hash), Error::<T>::HashAlreadyAnchored);
			pallet_schema::SchemaDetails::<T>::is_valid_identifier(
				&identifier,
				STREAM_IDENTIFIER_PREFIX,
			)
			.map_err(|_| Error::<T>::InvalidIdentifier)?;

			let tx_prev_details =
				<Streams<T>>::get(&identifier).ok_or(Error::<T>::StreamNotFound)?;
			ensure!(!tx_prev_details.revoked, Error::<T>::StreamRevoked);

			ensure!(
				tx_signature.verify(&(&digest_hash).encode()[..], &creator),
				Error::<T>::InvalidSignature
			);

			if let Some(ref schema) = tx_prev_details.schema {
				pallet_schema::SchemaDetails::<T>::schema_status(schema, creator.clone())
					.map_err(<pallet_schema::Error<T>>::from)?;
			}

			<DigestOf<T>>::insert(&digest_hash, &identifier);

			Self::deposit_event(Event::Digest(identifier, digest_hash, creator));

			Ok(())
		}
	}
}
