// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use sha3::{Digest, Keccak256};

#[laos_precompile_utils_macro::generate_function_selector]
pub enum Action {
	Toto = "toto()",
	Tata = "tata()",
}

#[test]
fn test_keccak256() {
	assert_eq!(&laos_precompile_utils_macro::keccak256!(""), Keccak256::digest(b"").as_slice(),);
	assert_eq!(
		&laos_precompile_utils_macro::keccak256!("toto()"),
		Keccak256::digest(b"toto()").as_slice(),
	);
	assert_ne!(
		&laos_precompile_utils_macro::keccak256!("toto()"),
		Keccak256::digest(b"tata()").as_slice(),
	);
}

#[test]
fn test_generate_function_selector() {
	assert_eq!(&(Action::Toto as u32).to_be_bytes()[..], &Keccak256::digest(b"toto()")[0..4],);
	assert_eq!(&(Action::Tata as u32).to_be_bytes()[..], &Keccak256::digest(b"tata()")[0..4],);
	assert_ne!(Action::Toto as u32, Action::Tata as u32);
}
