// This file is part of CORD – https://cord.network

// Copyright (C) 2019-2022 Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		Weight::from_ref_time(24_107_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(184_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		Weight::from_ref_time(36_869_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(165_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		Weight::from_ref_time(36_445_000 as u64)
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(199_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		Weight::from_ref_time(33_035_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		Weight::from_ref_time(25_946_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(101_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		Weight::from_ref_time(22_945_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(69_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		Weight::from_ref_time(41_502_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		Weight::from_ref_time(30_791_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		Weight::from_ref_time(2_000_000_000_000 as u64)
	}
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		Weight::from_ref_time(57_184_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: PhragmenElection Voting (r:5001 w:5000)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[1, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 85_000
			.saturating_add(Weight::from_ref_time(61_507_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: PhragmenElection Voting (r:10001 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:3 w:3)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 1_864_000
			.saturating_add(Weight::from_ref_time(167_385_000 as u64).saturating_mul(v as u64))
			// Standard Error: 124_000
			.saturating_add(Weight::from_ref_time(9_721_000 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(265 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
