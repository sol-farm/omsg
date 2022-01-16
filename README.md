# omsg

A set of macros for optimized usage of `msg!` involving string formatting, attempting to use stack backed formatting instead of heap based formatting when possible. Saves on average ~200 compute units per logged message