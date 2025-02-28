// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::common::test_dir_path;
use aptos_types::account_address::AccountAddress;
use e2e_move_tests::MoveHarness;
use framework::{BuildOptions, BuiltPackage, ReleasePackage};
use move_deps::move_package::compilation::package_layout::CompiledPackageLayout;
use package_builder::PackageBuilder;

mod common;

#[test]
fn generate_upgrade_script() {
    let mut h = MoveHarness::new();
    let acc = h.new_account_at(AccountAddress::from_hex_literal("0xcafe").unwrap());

    // Construct two packages: one for which a proposal is created, the other for
    // holding the proposal script.

    let mut upgrade = PackageBuilder::new("Pack");
    upgrade.add_source(
        "test",
        &format!(
            "\
module 0x{}::test {{
    public entry fun hi(_s: &signer){{
    }}
}}",
            acc.address()
        ),
    );
    let upgrade_dir = upgrade.write_to_temp().unwrap();

    let mut proposal = PackageBuilder::new("Proposal");
    proposal.add_local_dep(
        "AptosFramework",
        &test_dir_path("../../framework/aptos-framework").to_string_lossy(),
    );
    let proposal_dir = proposal.write_to_temp().unwrap();

    let upgrade_release = ReleasePackage::new(
        BuiltPackage::build(upgrade_dir.path().to_path_buf(), BuildOptions::default()).unwrap(),
    )
    .unwrap();

    // Generate the proposal and compile it.
    upgrade_release
        .generate_script_proposal(
            AccountAddress::ONE,
            proposal_dir
                .path()
                .to_path_buf()
                .join(CompiledPackageLayout::Sources.path())
                .join("proposal.move"),
        )
        .unwrap();
    let _ =
        BuiltPackage::build(proposal_dir.path().to_path_buf(), BuildOptions::default()).unwrap();

    // TODO: maybe execute the proposal.
}
