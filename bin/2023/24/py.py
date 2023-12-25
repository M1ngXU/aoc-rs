import numpy as np
import re
import z3

input = open("input.txt", "r")
lines = [[int(r) for r in re.findall("([-]?\d+)", line)] for line in input.readlines()]

# s = z3.Solver()
# x = z3.Real("x")
# y = z3.Real("y")
# z = z3.Real("z")
# dx = z3.Real("dx")
# dy = z3.Real("dy")
# dz = z3.Real("dz")
# for h1i, (x1, y1, z1, dx1, dy1, dz1) in enumerate(lines[:3]):
#     t = z3.Real("t" + str(h1i))
#     clx = z3.Real("clx" + str(h1i))
#     cly = z3.Real("cly" + str(h1i))
#     clz = z3.Real("clz" + str(h1i))
#     s.add(t > 0)
#     s.add(clx == x + dx * t)
#     s.add(clx == x1 + dx1 * t)
#     s.add(cly == y + dy * t)
#     s.add(cly == y1 + dy1 * t)
#     s.add(clz == z + dz * t)
#     s.add(clz == z1 + dz1 * t)
# if s.check() != z3.sat:
#     exit(0)
# solution_vector = []
# new_conditions = []
# for var in [x, y, z, dx, dy, dz]:
#     solution = s.model()[var].as_long()  # type: ignore pylint: disable=no-member
#     solution_vector.append(solution)
#     new_conditions.append(var != solution)
# print(solution, solution_vector, sum(solution_vector[:3]))
# exit(0)

total = 0
fails = 0
a = []
b = []
for i, line in enumerate(lines):
    x, y, z, dx, dy, dz = line
    a.append([1, 2] + [0] * i + [-dz] + [0] * (len(lines) - i - 1))
a = np.array(a)
b = np.array(b)
for i in range(len(lines)):
    for j in range(i):
        x1, y1, z1, dx1, dy1, dz1 = lines[i]
        x2, y2, z2, dx2, dy2, dz2 = lines[j]
        # x1 + dx1 * t1 = x2 + dx2 * t2
        # y1 + dy1 * t1 = y2 + dy2 * t2
        # use numpy to solve for t1/t2
        n = np.array([[dx1, -dx2], [dy1, -dy2]])
        b = np.array([x2 - x1, y2 - y1])
        try:
            t1, t2 = np.linalg.solve(n, b)
        except:
            print(x1, y1, dx1, dy1)
            print(x2, y2, dx2, dy2)
            fails += 1
            continue
        x_collide = x1 + dx1 * t1
        y_collide = y1 + dy1 * t1
        if (
            t1 > 0
            and t2 > 0
            and (x_collide >= 200000000000000 and x_collide <= 400000000000000)
            and (y_collide >= 200000000000000 and y_collide <= 400000000000000)
        ):
            total += 1
print(total, fails)
