// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use routing::{ImmutableData, MutableData, XorName};

/// Identifier for a data (immutable or mutable)
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum DataId {
    /// Identifier of immutable data.
    Immutable(XorName),
    /// Identifier of mutable data.
    Mutable(XorName, u64),
}

impl DataId {
    /// Create identifier of the given immutable data.
    pub fn immutable(data: &ImmutableData) -> Self {
        DataId::Immutable(*data.name())
    }

    /// Create identifier of the given mutable data.
    pub fn mutable(data: &MutableData) -> Self {
        DataId::Mutable(*data.name(), data.tag())
    }

    /// Get name of this identifier.
    pub fn name(&self) -> &XorName {
        match *self {
            DataId::Immutable(ref name) => name,
            DataId::Mutable(ref name, _) => name,
        }
    }
}

/// Type that can hold both immutable and mutable data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Data {
    /// Immutable data.
    Immutable(ImmutableData),
    /// Mutable data.
    Mutable(MutableData),
}

impl Data {
    /// Get `DataId` of this `Data`.
    pub fn id(&self) -> DataId {
        match *self {
            Data::Immutable(ref data) => DataId::immutable(data),
            Data::Mutable(ref data) => DataId::mutable(data),
        }
    }
}

impl From<ImmutableData> for Data {
    fn from(data: ImmutableData) -> Self {
        Data::Immutable(data)
    }
}

impl From<MutableData> for Data {
    fn from(data: MutableData) -> Self {
        Data::Mutable(data)
    }
}
