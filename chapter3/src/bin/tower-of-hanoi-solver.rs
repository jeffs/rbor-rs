#![allow(dead_code, unused_variables)]

use std::fmt;

const TOTAL_DISKS: usize = 6;

#[derive(Debug)]
struct Disk {
    radius: usize,
}

#[derive(Debug, Default)]
struct Tower {
    disks: Vec<Disk>, // a stack, ordered from bottom to top
}

#[derive(Debug)]
struct TowerSet {
    a: Tower,
    b: Tower,
    c: Tower,
}

impl TowerSet {
    fn labels() -> impl Iterator<Item = &'static str> {
        ["A", "B", "C"].into_iter()
    }

    fn towers(&self) -> impl Iterator<Item = &Tower> {
        [&self.a, &self.b, &self.c].into_iter()
    }

    fn disks(&self) -> impl Iterator<Item = &Disk> {
        self.towers().flat_map(|tower| tower.disks.iter())
    }
}

impl fmt::Display for TowerSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //  print pole tops
        //  for each row
        //      print a
        //      print b
        //      print c
        //      println
        //  print labels

        let pole = "||";
        let max_radius = self
            .disks()
            .map(|disk| disk.radius)
            .max()
            .unwrap_or_default();

        // Print the tops of the poles.
        for _ in self.towers() {
            write!(f, " {:^2$}||{:^2$}", "", "", max_radius)?;
        }
        writeln!(f)?;

        // Print the disks.
        let disk_count = self.disks().count();
        for depth in 0..disk_count {
            for tower in self.towers() {
                let index = disk_count - depth - 1;
                if let Some(disk) = tower.disks.get(index) {
                    let radius = disk.radius;
                    write!(
                        f,
                        " {blank:gap$}{fill}_{radius}{fill}{blank:gap$}",
                        blank = "",
                        fill = "@".repeat(radius),
                        gap = max_radius - radius,
                    )?;
                } else {
                    write!(f, " {:^2$}||{:^2$}", "", "", max_radius)?;
                }
            }
            writeln!(f)?;
        }

        // Print a label for each tower.
        for label in Self::labels() {
            write!(f, " {:^2$} {label}{:^2$}", "", "", max_radius)?;
        }

        Ok(())
    }
}

enum TowerSelector {
    A,
    B,
    C,
}

/// Whereas the book keeps the towers in a global variable, we pass them around
/// as a function argument.
fn solve(
    towers: TowerSet,
    height: usize,         // How many disks to move.
    source: TowerSelector, // What the book calls `startTower`.
    target: TowerSelector, // What the book calls `endTower`.
    buffer: TowerSelector, // What the book calls `tempTower`.
) {
    println!("\n{towers}");
}

fn main() {
    let disks = (1..=TOTAL_DISKS)
        .rev()
        .map(|radius| Disk { radius })
        .collect();
    let towers = TowerSet {
        a: Tower { disks },
        b: Tower::default(),
        c: Tower::default(),
    };
    let steps = solve(
        towers,
        TOTAL_DISKS,
        TowerSelector::A,
        TowerSelector::B,
        TowerSelector::C,
    );
}
