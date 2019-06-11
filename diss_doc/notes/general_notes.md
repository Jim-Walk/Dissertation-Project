Rather than fighting the borrow checker, I seem to be spending a lot of time battling the extremely strict type system, often resulting in extremely verbose code. I wonder if this is due to my inexperience, and if I will learn simpler ways to write things. 

The compiler's hints are very useful in these battles

Code in question which has annoyed me is below, from stream.rs
```
 let errA = self.a.iter().fold(T::from(0).unwrap(), |sum, val| sum + num::abs(*val - goldA));
```

Changing babel stream's add to rusty `a.iter().zip(b.iter())` made the function faster. average speed in the C++ way was about 0.09501 seconds, and was 0.09079 the rusty way. I think this might have something to do with `vec.iter_mut` 

Triad also showed a similar decrease in time when rewritten this way.

It seems like rust stream's implementation suffers as we are unable to initalise float variables in parrallel. I will attempt a version of babel stream which uses ints, and see if parallel initialisation increases speed.

Rust's bitwise shift operator panics rather than overflowing, which seems safer. Came to this realisation when working on sparse.c
