#![allow(dead_code, unused_variables)]

use std::fmt;

const TOTAL_DISKS: usize = 6;

#[derive(Debug)]
struct Disk {
    radius: usize,
}

#[derive(Debug)]
struct Tower {
    disks: Vec<Disk>, // a stack, ordered from bottom to top
}

#[derive(Debug)]
struct TowerSet {
    a: Tower,
    b: Tower,
    c: Tower,
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
    fn new(
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

/// Whereas the book keeps the towers in a global variable, we pass them around
/// as a function argument.  We also group the tower selectors into a single
/// struct, rather than passing them all as separate function arguments as the
/// book does.  What the book calls `numberOfDisks`, we call `height`.  These
/// are purely stylistic choices.
fn solve(towers: TowerSet, height: usize, selectors: TowerSelectorSet) {
    if height == 0 {
        return; // BASE CASE
    }
    //let Some(disk) = towers.
    println!("\n{towers}");
}

fn main() {
    let disks = (1..=TOTAL_DISKS)
        .rev()
        .map(|radius| Disk { radius })
        .collect();
    let towers = TowerSet {
        a: Tower { disks },
        b: Tower { disks: vec![] },
        c: Tower { disks: vec![] },
    };
    solve(
        towers,
        TOTAL_DISKS,
        TowerSelectorSet::new(TowerSelector::A, TowerSelector::B, TowerSelector::C),
    );
}
