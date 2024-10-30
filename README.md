# Rust important reference 


[Rust 程式語言](https://askeing.github.io/rust-book/README.html)

[[Rust] 程式設計教學](https://opensourcedoc.com/rust-programming/)

[rust-by-example](https://github.com/rust-lang/rust-by-example)

[Comprehensive Rust](https://google.github.io/comprehensive-rust/zh-TW/)

## Rust in Linux Kernel 
https://hackmd.io/@linD026/rust-in-linux-organize

# Gitpod Rust Tutorial 

ref : https://www.tutorialspoint.com/rust/index.htm

## mathlang , through this project , to demo how to use cargo 
./src    

## tutorial 
./tutorial 


### test

The Deep Q-Network (DQN) approximates the Q-value function, which in standard Q-learning is defined as:

\[
Q(s, a) = \mathbb{E}\left[ r + \gamma \max_{a'} Q(s', a') \mid s, a \right]
\]

where:

- \( s \): current state
- \( a \): action taken from state \( s \)
- \( r \): immediate reward
- \( s' \): next state after taking action \( a \)
- \( a' \): possible actions in the next state \( s' \)
- \( \gamma \): discount factor, representing the importance of future rewards

The **loss function** used to train the DQN, derived from the Bellman equation, is:

\[
L(\theta) = \mathbb{E}_{s, a, r, s'} \left[ \left( y - Q(s, a; \theta) \right)^2 \right]
\]

where:

- \( \theta \): parameters of the Q-network
- \( Q(s, a; \theta) \): Q-value predicted by the network for state \( s \) and action \( a \)
- \( y = r + \gamma \max_{a'} Q(s', a'; \theta^{-}) \): target value, computed using a separate target network with parameters \( \theta^{-} \)

## MathLang from Gitpod 
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/JesterOrNot/MathLang) 

Basic maths language

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/JesterOrNot/MathLang)
