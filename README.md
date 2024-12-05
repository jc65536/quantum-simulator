# CS 238 Homework: Implement a quantum circuit simulator
*Jason Cheng*

## Design

My design supports up to 64 qubits, since `BitVec` is a `u64`. I made sure my
amplitudes were also using `u64`, so that they are long enough to represent
amplitudes up to and including $ 2^{64 / 2} $.

The actual number of qubits $ n $ is determined from the program. I realized
that cirq doesn't actually care about the `qreg` statement, but counts the
number of actual qubits used instead. Similarly, my program counts the number of
qubits used across instructions and uses that as $ n $.

Of course, the user can always override the detected number of qubits by setting `n` in `State`.

## Evaluation

