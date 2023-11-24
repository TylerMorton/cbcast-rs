## Implementation of Causal Broadcast
After reading the paper "Lightweight Causal and Atomic Group Multicast" by Kenneth Birman, et. al. I decided to try and implement the protocol myself. The goal was to create a network interface library that would use TCP to have some sort of FIFO guarantee and then have the causal broadcast protocol on top of that. I am currently not implementing atomic broadcast because I am already having enough trouble with causal, plus I would need some sort of leader election feature.

## Thoughts on causal broadcast.
Great, when you need it. The protocol is great when you need to have causal ordering but increases overhead. This relates to the ideals of CAP where we are sacrificing availability to increase consistency. In this case increasing consistency with causal time and decreasing availability to processing messages that arrive from FIFO channels.

## Todos:
- Use over TCP
- Implement vector clock ordering (there is currently a bug!)