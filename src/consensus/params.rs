// Rust Bitcoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Consensus parameters
//!
//! This module provides predefined set of parameters for different chains.
//!

use network::constants::Network;
use util::uint::Uint256;

/// Lowest possible difficulty for Mainnet.
const MAX_BITS_BITCOIN: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000000ffffffffu64,
]);
/// Lowest possible difficulty for Testnet.
const MAX_BITS_TESTNET: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000000ffffffffu64,
]);
/// Lowest possible difficulty for Regtest.
const MAX_BITS_REGTEST: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x7fffffffffffffffu64,
]);
/// Lowest possible difficulty for Mainnet.
const MAX_BITS_DOGECOIN: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000fffffffffffu64,
]);
/// Lowest possible difficulty for Mainnet.
const MAX_BITS_DOGETEST: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000fffffffffffu64,
]);

#[derive(Debug, Clone)]
/// Parameters that influence chain consensus.
pub struct Params {
    /// Network for which parameters are valid.
    pub network: Network,
    /// Time when BIP16 becomes active.
    pub bip16_time: u32,
    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,
    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,
    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,
    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,
    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,
    /// Proof of work limit value. It contains the lowest possible difficulty.
    pub pow_limit: Uint256,
    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,
    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,
    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,
    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
}

impl Params {
    /// Creates parameters set for the given network.
    pub fn new(network: Network) -> Self {
        match network {
            Network::Bitcoin => Params {
                network: Network::Bitcoin,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 227931, // 000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8
                bip65_height: 388381, // 000000000000000004c2b624ed5d7756c508d90fd0da2c7c679febfa6c4735f0
                bip66_height: 363725, // 00000000000000000379eaa19dce8c9b722d46ae6a57c2f1a988119488b50931
                rule_change_activation_threshold: 1916, // 95%
                miner_confirmation_window: 2016,
                pow_limit: MAX_BITS_BITCOIN,
                pow_target_spacing: 10 * 60,            // 10 minutes.
                pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
                allow_min_difficulty_blocks: false,
                no_pow_retargeting: false,
            },
            Network::Testnet => Params {
                network: Network::Testnet,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 21111, // 0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8
                bip65_height: 581885, // 00000000007f6655f22f98e72ed80d8b06dc761d5da09df0fa1dc4be4f861eb6
                bip66_height: 330776, // 000000002104c8c45e99a8853285a3b592602a3ccde2b832481da85e9e4ba182
                rule_change_activation_threshold: 1512, // 75%
                miner_confirmation_window: 2016,
                pow_limit: MAX_BITS_TESTNET,
                pow_target_spacing: 10 * 60,            // 10 minutes.
                pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
            },
            Network::Regtest => Params {
                network: Network::Regtest,
                bip16_time: 1333238400,  // Apr 1 2012
                bip34_height: 100000000, // not activated on regtest
                bip65_height: 1351,
                bip66_height: 1251,                    // used only in rpc tests
                rule_change_activation_threshold: 108, // 75%
                miner_confirmation_window: 144,
                pow_limit: MAX_BITS_REGTEST,
                pow_target_spacing: 10 * 60,            // 10 minutes.
                pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: true,
            },
            Network::Dogecoin => Params {
                network: Network::Dogecoin,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 1034383, // 0x80d1364201e5df97e696c03bdd24dc885e8617b9de51e453c10a4f629b1e797a
                bip65_height: 3464751, // 34cd2cbba4ba366f47e5aa0db5f02c19eba2adf679ceb6653ac003bdc9a0ef1f
                bip66_height: 1034383, // 80d1364201e5df97e696c03bdd24dc885e8617b9de51e453c10a4f629b1e797a
                rule_change_activation_threshold: 9576, // 95%
                miner_confirmation_window: 10080,
                pow_limit: MAX_BITS_DOGECOIN,
                pow_target_spacing: 60,           // 1 minute.
                pow_target_timespan: 4 * 60 * 60, // pre-digishield: 4 hours
                allow_min_difficulty_blocks: false,
                no_pow_retargeting: false,
            },
            Network::Dogetest => Params {
                network: Network::Dogetest,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 708658, // 0x21b8b97dcdb94caa67c7f8f6dbf22e61e0cfe0e46e1fff3528b22864659e9b38
                bip65_height: 1854705, // 955bd496d23790aba1ecfacb722b089a6ae7ddabaedf7d8fb0878f48308a71f9
                bip66_height: 708658, // 21b8b97dcdb94caa67c7f8f6dbf22e61e0cfe0e46e1fff3528b22864659e9b38
                rule_change_activation_threshold: 2880, // 75%
                miner_confirmation_window: 10080,
                pow_limit: MAX_BITS_DOGETEST,
                pow_target_spacing: 60,           // 1 minute.
                pow_target_timespan: 4 * 60 * 60, // pre-digishield: 4 hours
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
            },
        }
    }

    /// Calculates the number of blocks between difficulty adjustments.
    pub fn difficulty_adjustment_interval(&self) -> u64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
