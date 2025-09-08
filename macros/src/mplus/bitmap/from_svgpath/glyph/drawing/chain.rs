use std::{iter, option, slice};

use swash::zeno::{Command, PathData, Point, Vector};

type Rest<'a> = iter::Map<iter::Copied<slice::Iter<'a, Vector>>, fn(Vector) -> Command>;
type Map<'a> = fn(&'a Chain) -> iter::Chain<option::IntoIter<Command>, Rest<'a>>;

pub struct Chain(pub Vec<Point>);
pub struct ChainList(pub Vec<Chain>);

impl<'a> PathData for &'a Chain {
    type Commands = iter::Chain<option::IntoIter<Command>, Rest<'a>>;

    fn commands(&self) -> Self::Commands {
        let Chain(points) = self;
        let mut points = points.iter().copied();
        let first = points.next().map(Command::MoveTo);
        let rest: Rest = points.map(Command::LineTo);

        first.into_iter().chain(rest)
    }
}

impl<'a> PathData for &'a ChainList {
    type Commands =
        iter::FlatMap<slice::Iter<'a, Chain>, <&'a Chain as PathData>::Commands, Map<'a>>;

    fn commands(&self) -> Self::Commands {
        let ChainList(chains) = self;
        let map: Map = |chain| chain.commands();

        chains.iter().flat_map(map)
    }
}

impl<T: Into<Vec<Point>>> From<T> for Chain {
    fn from(points: T) -> Self {
        Self(points.into())
    }
}

impl<T: Into<Vec<Chain>>> From<T> for ChainList {
    fn from(chains: T) -> Self {
        Self(chains.into())
    }
}
