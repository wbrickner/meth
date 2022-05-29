#![doc = include_str!("../README.md")]

use std::{thread::{spawn, sleep}, time::Duration, sync::mpsc::Receiver};
use mouse_rs::{Mouse, types::Point};
mod wiggler;

/// Wiggles immediately. If mouse does not move for `30s`, wiggle again, keeping the desktop awake.
/// 
/// ### Things to know:
/// 
/// - __Sleep prevention stops when `Meth` is dropped.__
/// - Manages its own background thread, idle sleeping most of the time.
/// - Background thread may persist for up to `30s` after `Meth` is dropped.
/// 
/// ```rust
/// // computer will not sleep until
/// // `important_computation()` completes.
/// {
///   let meth = Meth::new();
///   important_computation();
/// }
/// 
/// // `meth` dropped,
/// // computer might enter sleep now.
/// ```
pub struct Meth {
  /// tx handle to kill background thread (which occasionally wiggles the mouse)
  terminate: std::sync::mpsc::Sender<()>
}

impl Drop for Meth {
  fn drop(&mut self) {
    // can only fail when receiver is dropped first
    // (in which case our goal is already achieved)
    let _ = self.terminate.send(());
  }
}

impl Meth {
  /// Wait 30s between wiggle attempts
  const SLEEP_PERIOD: Duration = Duration::from_secs(30);

  /// Create a new Meth instance
  pub fn new() -> Self {
    let (terminate, rx) = std::sync::mpsc::channel();
    
    spawn(move || {
      loop {
        // check for termination signal
        if let Ok(_) = rx.try_recv() { break }
        
        // if some error occurs, continue to revive thread
        if Self::keep_awake(&rx).is_ok() { break }
        sleep(Self::SLEEP_PERIOD)
      }
    });

    Meth { terminate }
  }

  /// Are these two points identical?
  fn identical(a: &Point, b: &Point) -> bool {
    a.x == b.x && 
    a.y == b.y
  }

  /// Fallible inner control loop.
  /// - Will wiggle immediately, in case we are about to sleep.
  fn keep_awake(rx: &Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    let mouse = Mouse::new();
    let mut position = mouse.get_position().unwrap();
    let mut wiggler = wiggler::Wiggler::default();

    loop {
      // check for termination signal
      if let Ok(_) = rx.try_recv() { break }
      
      {
        let mut p = mouse.get_position()?;
      
        // if the mouse has not moved since last poll
        if Self::identical(&p, &position) {
          // wiggle it
          wiggler.transform(&mut p);
          mouse.move_to(p.x as i32, p.y as i32)?;
        }

        position = p;
      };

      // return to sleep
      sleep(Meth::SLEEP_PERIOD);
    }

    Ok(())
  }
}


#[cfg(test)]
mod test {
  use std::time::Duration;
  use crate::Meth;

  #[test]
  fn test_move() {
    {
      println!("Wiggling soon!");
      let _meth = Meth::new();
      std::thread::sleep(Duration::from_secs(60));
    }

    println!("No more wiggles!");
    std::thread::sleep(Duration::from_secs(90));
  }

  #[test]
  fn two() {
    {
      println!("Wiggling soon!");
      let _meth_a = Meth::new();
      let _meth_b = Meth::new();
      std::thread::sleep(Duration::from_secs(60));
    }

    println!("No more wiggles!");
    std::thread::sleep(Duration::from_secs(90));
  }
}