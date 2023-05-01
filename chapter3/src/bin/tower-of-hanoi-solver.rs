//! While this Tower of Hanoi solver here is modeled on the one from Chapter 3
//! of The Recursive Book of Recursion, it does not mimic the structure of the
//! book's sample code as closely as earlier exercises in this repository.
//! While a shorter version of this solver is certainly possible--for example,
//! by using numbers and stacks to represent disks and towers directly, rather
//! than wrapping them in bespoke Disk and Tower types--such an implementation
//! would not be idiomatic Rust.
//!
//! Tiny programs like those featured earlier in the book are so simple that
//! good structure and conventions are not critical to their clarity or
//! maintainability.  The Tower of Hanoi exercise, however, seems complex
//! enough to merit a slightly more disciplined approach, which anyway is a
//! better fit for Rust.  Even in Python and JavaScript, the reader may enjoy
//! replacing the book's use of global variables with function arguments and
//! returns values, and encapsulating the book's raw numbers, strings, and
//! arrays with object-oriented classes akin to the Rust types defined here.
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::{fmt, io};

#[derive(Debug, Eq, PartialEq)]
struct Disk {
    radius: usize,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Tower {
    disks: Vec<Disk>, // a stack, ordered from bottom to top
}

impl Tower {
    fn pop(&mut self) -> Option<Disk> {
        self.disks.pop()
    }

    fn push(&mut self, disk: Disk) {
        self.disks.push(disk);
    }
}

impl Tower {
    fn with_disks(n: usize) -> Tower {
        Tower {
            disks: (1..=n).rev().map(|radius| Disk { radius }).collect(),
        }
    }
}

const TOWER_LABELS: [&str; 3] = ["A", "B", "C"];

#[derive(Debug)]
struct ParseTowerSelectorError(String);

impl fmt::Display for ParseTowerSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: bad tower selector", self.0)
    }
}

impl Error for ParseTowerSelectorError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TowerSelector {
    A,
    B,
    C,
}

impl fmt::Display for TowerSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", TOWER_LABELS[*self as usize])
    }
}

impl FromStr for TowerSelector {
    type Err = ParseTowerSelectorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(TowerSelector::A),
            "B" => Ok(TowerSelector::B),
            "C" => Ok(TowerSelector::C),
            _ => Err(ParseTowerSelectorError(s.to_string())),
        }
    }
}

struct TowerSelectorSet {
    source: TowerSelector, // What the book calls `startTower`.
    target: TowerSelector, // What the book calls `endTower`.
    buffer: TowerSelector, // What the book calls `tempTower`.
}

impl TowerSelectorSet {
    fn from_parts(
        source: TowerSelector,
        target: TowerSelector,
        buffer: TowerSelector,
    ) -> TowerSelectorSet {
        (source == target || source == buffer || target == buffer)
            .then(|| panic!("tower selectors should be unique: {source}, {target}, {buffer}"));
        TowerSelectorSet {
            source,
            target,
            buffer,
        }
    }

    fn new() -> TowerSelectorSet {
        TowerSelectorSet {
            source: TowerSelector::A,
            target: TowerSelector::B,
            buffer: TowerSelector::C,
        }
    }
}

#[derive(Debug)]
struct EmptyTowerError(TowerSelector);

impl fmt::Display for EmptyTowerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tower {} is empty", self.0)
    }
}

impl Error for EmptyTowerError {}

#[derive(Debug, Eq, PartialEq)]
struct TowerSet {
    a: Tower,
    b: Tower,
    c: Tower,
}

impl TowerSet {
    fn with_disks(n: usize) -> TowerSet {
        TowerSet {
            a: Tower::with_disks(n),
            b: Tower::default(),
            c: Tower::default(),
        }
    }

    fn move_disk(
        &mut self,
        source: TowerSelector,
        target: TowerSelector,
    ) -> Result<(), EmptyTowerError> {
        let disk = self[source].pop().ok_or(EmptyTowerError(source))?;
        self[target].push(disk);
        Ok(())
    }
}

impl<'a> IntoIterator for &'a TowerSet {
    type Item = &'a Tower;
    type IntoIter = std::array::IntoIter<&'a Tower, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.a, &self.b, &self.c].into_iter()
    }
}

impl TowerSet {
    fn disks(&self) -> impl Iterator<Item = &Disk> {
        self.into_iter().flat_map(|tower| tower.disks.iter())
    }
}

/// Helper for implementing the Display trait.
struct TowerSetDisplay<'a> {
    towers: &'a TowerSet,
    radius: usize,
}

impl TowerSetDisplay<'_> {
    fn new(towers: &TowerSet) -> TowerSetDisplay {
        TowerSetDisplay {
            towers,
            radius: towers
                .disks()
                .map(|disk| disk.radius)
                .max()
                .unwrap_or_default(),
        }
    }

    fn write_pole(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {:^2$}||{:^2$}", "", "", self.radius)
    }

    fn print_pole_tops(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in self.towers {
            self.write_pole(f)?;
        }
        writeln!(f)
    }

    fn print_disks(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disk_count = self.towers.disks().count();
        for depth in 0..disk_count {
            for tower in self.towers {
                if let Some(disk) = tower.disks.get(disk_count - depth - 1) {
                    // There's a disk at this depth on this tower.  Print it,
                    // surrounded by space.
                    let radius = disk.radius;
                    write!(
                        f,
                        " {blank:gap$}{fill}_{radius}{fill}{blank:gap$}",
                        blank = "",
                        fill = "@".repeat(radius),
                        gap = self.radius - radius,
                    )?;
                } else {
                    // There's no disk at this depth on this tower.  Print the
                    // pole, surrounded by space.
                    self.write_pole(f)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

    fn print_labels(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for label in TOWER_LABELS {
            write!(f, " {:^2$} {label}{:^2$}", "", "", self.radius)?;
        }
        Ok(())
    }

    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print_pole_tops(f)?;
        self.print_disks(f)?;
        self.print_labels(f)
    }
}

impl fmt::Display for TowerSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        TowerSetDisplay::new(self).print(f)
    }
}

impl Index<TowerSelector> for TowerSet {
    type Output = Tower;

    fn index(&self, index: TowerSelector) -> &Self::Output {
        match index {
            TowerSelector::A => &self.a,
            TowerSelector::B => &self.b,
            TowerSelector::C => &self.c,
        }
    }
}

impl IndexMut<TowerSelector> for TowerSet {
    fn index_mut(&mut self, index: TowerSelector) -> &mut Self::Output {
        match index {
            TowerSelector::A => &mut self.a,
            TowerSelector::B => &mut self.b,
            TowerSelector::C => &mut self.c,
        }
    }
}

/// Whereas the book keeps the towers in a global variable, we pass them
/// around. We also group the tower selectors into a single struct, rather than
/// passing them all as separate function arguments as the book does.  What the
/// book calls `numberOfDisks`, we call `height`.  These are purely stylistic
/// choices.
fn solve(
    towers: TowerSet,
    height: usize,
    selectors: TowerSelectorSet,
) -> Result<TowerSet, EmptyTowerError> {
    if height == 0 {
        // BASE CASE: No disks to move.
        println!("\n{towers}");
        return Ok(towers);
    }
    // RECURSIVE CASE
    let TowerSelectorSet {
        source,
        target,
        buffer,
    } = selectors;
    let mut towers = solve(
        towers,
        height - 1,
        TowerSelectorSet::from_parts(source, buffer, target),
    )?;
    towers.move_disk(source, target)?;
    solve(
        towers,
        height - 1,
        TowerSelectorSet::from_parts(buffer, target, source),
    )
}

fn try_move_command(
    towers: &mut TowerSet,
    source: &str,
    target: &str,
) -> Result<(), Box<dyn Error>> {
    Ok(towers.move_disk(source.parse()?, target.parse()?)?)
}

fn interact(mut towers: TowerSet) -> Result<TowerSet, Box<dyn Error>> {
    let mut lines = io::stdin().lines();
    loop {
        println!("\n{towers}");
        println!("\nEnter letter of start tower and the end tower. (A, B, C) or Q to quit.");
        let Some(line) = lines.next() else {
            return Ok(towers);  // End of input.
        };
        let line = line?.to_uppercase();
        let line = line.trim();
        if line.starts_with('Q') {
            return Ok(towers);
        }
        match line.len() {
            0 => {
                // Ignore empty line.
            }
            1 if line.starts_with('Q') => {
                return Ok(towers);
            }
            2 => {
                if let Err(err) = try_move_command(&mut towers, &line[..1], &line[1..2]) {
                    eprintln!("warning: {err}");
                }
            }
            _ => {
                eprintln!("warning: bad command");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        for n in 0..10 {
            let want = TowerSet {
                a: Tower::default(),
                b: Tower::with_disks(n),
                c: Tower::default(),
            };
            let got = solve(TowerSet::with_disks(n), n, TowerSelectorSet::new());
            assert_eq!(got, want);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const TOTAL_DISKS: usize = 6;
    let towers = TowerSet::with_disks(TOTAL_DISKS);
    if std::env::args().any(|arg| &arg == "-i" || &arg == "--interactive") {
        interact(towers)?;
    } else {
        solve(towers, TOTAL_DISKS, TowerSelectorSet::new())?;
    }
    Ok(())
}
