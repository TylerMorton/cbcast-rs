[![codecov](https://codecov.io/gh/TylerMorton/cbcast-rs/graph/badge.svg?token=1UICLGJJMJ)](https://codecov.io/gh/TylerMorton/cbcast-rs)

## Implementation of Causal Broadcast
After reading the paper "Lightweight Causal and Atomic Group Multicast" by Kenneth Birman, et. al., I decided to try and implement the protocol myself. The goal was to create a network interface library that would use TCP to have some sort of FIFO guarantee and then have the causal broadcast (cbcast) protocol on top of that. I am currently not implementing atomic broadcast because I am already having enough trouble with causal, plus I would need some sort of leader election feature.

## Thoughts on CBCAST 
Great when you need it, the protocol excels in situations requiring causal ordering but comes with increased overhead. This aligns with the principles of CAP, where we willingly trade off availability for enhanced consistency. In this context, the emphasis is on bolstering consistency through causal time while accepting reduced availability to process messages arriving from FIFO channels.

## Todos:
- Implement vector clock ordering (there is currently a bug!)
- Use over TCP, using sockets to transfer data is not operational.