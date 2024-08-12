use crate::{
	configs::collective_council::CouncilCollective, currency::UNIT, weights, AccountId, Balance,
	Balances, BlockNumber, EnsureRoot, Permill, Runtime, RuntimeEvent, Treasury,
};
use frame_support::{parameter_types, traits::EitherOfDiverse, PalletId};
use parachains_common::DAYS;

#[cfg(feature = "fast-mode")]
use parachains_common::MINUTES;
#[cfg(feature = "fast-mode")]
const TREASURY_SPENDING_PRERIOD: BlockNumber = 5 * MINUTES;
#[cfg(not(feature = "fast-mode"))]
const TREASURY_SPENDING_PRERIOD: BlockNumber = 7 * DAYS;

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 100 * UNIT;
	pub const SpendPeriod: BlockNumber = TREASURY_SPENDING_PRERIOD;
	pub const MaxApprovals: u32 = 100;
	pub const TreasuryId: PalletId = PalletId(*b"py/trsry");
}

type EnsureRootOrMajorityCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
>;

type EnsureRootOrAllCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>,
>;

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EnsureRootOrAllCouncil;
	type Burn = ();
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
	type PalletId = TreasuryId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = EnsureRootOrMajorityCouncil;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_trasury_address() {
		assert_eq!(
			pallet_treasury::Pallet::<Runtime>::account_id().to_string(),
			"0x6d6f646C70792f74727372790000000000000000"
		);
	}
}
