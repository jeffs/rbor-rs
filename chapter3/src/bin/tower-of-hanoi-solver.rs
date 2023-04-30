use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq)]
struct Disk {
    radius: usize,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Tower {
    disks: Vec<Disk>, // a stack, ordered from bottom to top
}

impl Tower {
    fn must_pop(&mut self) -> Disk {
        self.disks
            .pop()
            .expect("A tower being popped from should have at least one disk")
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

const TOWER_LABELS: [&'static str; 3] = ["A", "B", "C"];

#[derive(Clone, Copy, Eq, PartialEq)]
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

struct TowerSelectorSet {
    source: TowerSelector, // What the book calls `startTower`.
    target: TowerSelector, // What the book calls `endTower`.
    buffer: TowerSelector, // What the book calls `tempTower`.
}

impl TowerSelectorSet {
    #[allow(dead_code)]
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

    fn move_disk(&mut self, source: TowerSelector, target: TowerSelector) {
        let disk = self[source].must_pop();
        self[target].push(disk);
    }
}

impl TowerSet {
    fn towers(&self) -> impl Iterator<Item = &Tower> {
        [&self.a, &self.b, &self.c].into_iter()
    }

    fn disks(&self) -> impl Iterator<Item = &Disk> {
        self.towers().flat_map(|tower| tower.disks.iter())
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
        for _ in self.towers.towers() {
            self.write_pole(f)?;
        }
        writeln!(f)
    }

    fn print_disks(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disk_count = self.towers.disks().count();
        for depth in 0..disk_count {
            for tower in self.towers.towers() {
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
fn solve(mut towers: TowerSet, height: usize, selectors: TowerSelectorSet) -> TowerSet {
    let TowerSelectorSet {
        source,
        target,
        buffer,
    } = selectors;
    match height {
        0 => {
            // BASE CASE: No disks to move.
            towers
        }
        1 => {
            // BASE CASE: Only one disk to move.
            towers.move_disk(source, target);
            println!("\n{towers}");
            towers
        }
        _ => {
            // RECURSIVE CASE
            let mut towers = solve(
                towers,
                height - 1,
                TowerSelectorSet {
                    source,
                    target: buffer,
                    buffer: target,
                },
            );
            towers.move_disk(source, target);
            solve(
                towers,
                height - 1,
                TowerSelectorSet {
                    source: buffer,
                    target,
                    buffer: source,
                },
            )
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

fn main() {
    const TOTAL_DISKS: usize = 6;
    solve(
        TowerSet::with_disks(TOTAL_DISKS),
        TOTAL_DISKS,
        TowerSelectorSet::new(),
    );
}
