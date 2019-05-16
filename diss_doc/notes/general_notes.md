Rather than fighting the borrow checker, I seem to be spending a lot of time battling the extremely strict type system, often resulting in extremely verbose code. I wonder if this is due to my inexperience, and if I will learn simpler ways to write things. 

The compiler's hints are very useful in these battles

Code in question which has annoyed me is below, from stream.rs
```
 let errA = self.a.iter().fold(T::from(0).unwrap(), |sum, val| sum + num::abs(*val - goldA));
```
