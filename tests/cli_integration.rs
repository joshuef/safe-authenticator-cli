// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use assert_cmd::prelude::*;
use predicates::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::process::Command;

static UNAUTHED_REQ: &str = "bAAAAAADNVCMIGAQAAAACQAAAAAAAAAAANZSXILTNMFUWI43BMZSS4YLQNFPXA3DBPFTXE33VNZSC453FMJRWY2LFNZ2C4MJQAE";
static UNAUTHED_RESPONSE: &str = "bAEAAAADNVCMIGAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC\n"; // \n added to string with println!

static AUTHED_REQ: &str = "bAAAAAAFBMHKYWAAAAAABWAAAAAAAAAAANZSXILTNMFUWI43BMZSS45DFON2C453FMJQXA4BONFSAACYAAAAAAAAAABLWKYSBOBYCAVDFON2A2AAAAAAAAAAAJVQWSZCTMFTGKICMORSC4AACAAAAAAAAAAAAUAAAAAAAAAAAL5SG6Y3VNVSW45DTAEAAAAAAAAAAAAIAAAAAOAAAAAAAAAAAL5YHKYTMNFRQCAAAAAAAAAAAAAAAAAAB";
static AUTHED_RESPONSE_START: &str = "bAEAAAAFBMHKYW";

#[test]
#[ignore]
fn calling_safe_create_acc() {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mut cmd = Command::cargo_bin("safe_auth").unwrap();

    cmd.args(&vec!["--invite-token", &rand_string])
        .assert()
        .success();
}

#[test]
#[ignore]
fn calling_safe_auth_with_unregistered_req() {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mut cmd = Command::cargo_bin("safe_auth").unwrap();

    cmd.args(&vec!["--invite-token", &rand_string])
        .assert()
        .success();

    let mut auth_cmd = Command::cargo_bin("safe_auth").unwrap();

    auth_cmd
        .args(&vec!["-r", &UNAUTHED_REQ])
        .assert()
        .stdout(UNAUTHED_RESPONSE)
        .success();
}

#[test]
#[ignore]
fn calling_safe_auth_with_registered_req() {
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mut cmd = Command::cargo_bin("safe_auth").unwrap();

    cmd.args(&vec!["--invite-token", &rand_string])
        .assert()
        .success();

    let mut auth_cmd = Command::cargo_bin("safe_auth").unwrap();

    auth_cmd
        .args(&vec!["-r", &AUTHED_REQ])
        .assert()
        .stdout(predicate::str::starts_with(AUTHED_RESPONSE_START).from_utf8())
        .success();
}
