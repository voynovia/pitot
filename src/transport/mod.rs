// Pitot - a customizable aviation information receiver
// Copyright (C) 2017-2018  Datong Sun (dndx@idndx.com)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use pitot::handle::Handle;
use std::iter::Chain;
use std::slice::Iter;

type ChainedIter<'a> = Chain<Iter<'a, Payload>, Iter<'a, Payload>>;

pub mod udp;

use protocol::Payload;

pub trait Transport {
    fn run(&mut self, handle: &mut Handle, i: ChainedIter);
}
