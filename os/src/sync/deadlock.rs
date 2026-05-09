use alloc::vec::Vec;
use alloc::vec;

/// DEAD
pub const DEAD: isize = -0xDEAD;

/// deadlock detector
pub struct DeadlockDetector {
    /// available
    pub available: Vec<usize>,
    /// allocation
    pub allocation: Vec<Vec<usize>>,
    /// need
    pub need: Vec<Vec<usize>>,
}

impl DeadlockDetector {
    /// new
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }

    /// initialize when thread is created for allocation and need
    /// m: numbers of resources
    pub fn initialize(&mut self, tid: usize, m: usize) {
        assert_eq!(self.available.len(), m);

        while tid >= self.allocation.len() {
            self.allocation.push(vec![0; m]);
        }
        self.allocation[tid] = vec![0; m];

        while tid >= self.need.len() {
            self.need.push(vec![0; m]);
        }
        self.need[tid] = vec![0; m];
    }

    /// adjust allocation and need size when create mutex or semaphore
    /// m: the length of mutex_list or semaphore_list
    pub fn adjust_resource_size(&mut self, m: usize) {
        for thread in self.allocation.iter_mut() {
            while m > thread.len() {
                thread.push(0);
            }
        }

        for thread in self.need.iter_mut() {
            while m > thread.len() {
                thread.push(0);
            }
        }
    }

    /// is_safe
    pub fn is_safe(&self) -> bool {
        let m = self.available.len(); // numbers of resources
        let n = self.allocation.len(); // numbers of threads

        if m == 0 || n == 0 { return true }

        assert_eq!(self.allocation[0].len(), self.need[0].len());
        assert_eq!(m, self.need[0].len());
        assert_eq!(n, self.need.len());

        let mut work = self.available.clone();
        let mut finish = vec![false; n];

        loop {
            let mut found = false;
            for i in 0..n {
                if !finish[i] {
                    let can_allocate = (0..m).all(|j| self.need[i][j] <= work[j] );
                    if can_allocate {
                        for j in 0..m {
                            work[j] += self.allocation[i][j];
                        }

                        found = true;
                        finish[i] = true;
                    }
                }
            }

            if !found {
                break;
            }
        }

        finish.iter().all(|&f| f)
    }
}