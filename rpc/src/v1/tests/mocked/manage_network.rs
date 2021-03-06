// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use ethsync::{ManageNetwork, NetworkConfiguration};
use util;

pub struct TestManageNetwork;

// TODO: rob, gavin (originally introduced this functions) - proper tests and test state
impl ManageNetwork for TestManageNetwork {
	fn accept_unreserved_peers(&self) { }
	fn deny_unreserved_peers(&self) { }
	fn remove_reserved_peer(&self, _peer: String) -> Result<(), String> { Ok(()) }
	fn add_reserved_peer(&self, _peer: String) -> Result<(), String> { Ok(()) }
	fn start_network(&self) {}
	fn stop_network(&self) {}
	fn network_config(&self) -> NetworkConfiguration { NetworkConfiguration::from(util::NetworkConfiguration::new_local()) }
}
