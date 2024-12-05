import matplotlib.pyplot as plt

i = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]
t = [1, 0, 0, 0, 0, 0, 0, 0, 2, 4, 5, 13, 25, 50, 101, 209, 446, 892, 1852, 3345, 6537, 14064]

plt.plot(i, t)
plt.xlabel("Number of qubits in superposition")
plt.ylabel("Simulation time (ms)")
plt.title("Scalability with number of qubits")
plt.show()