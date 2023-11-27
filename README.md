## Implementation of Causal Broadcast
After checking out Kenneth Birman's paper, "Lightweight Causal and Atomic Group Multicast," I decided to take a shot at implementing the causal broadcast protocol. The idea was to build a network interface library using TCP for a reliable FIFO guarantee, topped with the causal broadcast (cbcast) protocol.

Right now, my main focus is on tackling the challenges of causal broadcast. I'm steering clear of diving into atomic broadcast implementation because it's already proving to be quite a handful, and there's the added consideration of possibly needing a leader election feature.

## Thoughts on CBCAST 
CBCAST is a handy tool when you need precise causal ordering, but it does come with some extra overhead. This lines up with CAP principles, where we're willing to sacrifice a bit of availability for better consistency. Here, the goal is to strengthen consistency through causal time, even if it means temporarily slowing down processing for messages coming in from FIFO channels.

## Todos:
- Implement vector clock ordering (there is currently a bug!)
- Use over TCP, using sockets to transfer data is not operational.
