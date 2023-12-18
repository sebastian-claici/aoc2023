from dataclasses import dataclass


@dataclass
class Polygon:
    points: list[(int, int)]

    def add(self, p: (int, int)):
        self.points.append(p)

    def area(self):
        ar = 0
        for i in range(len(self.points) - 1):
            x1, y1 = self.points[i]
            x2, y2 = self.points[(i + 1) % len(self.points)]
            ar += x1 * y2 - x2 * y1

        return abs(ar // 2)

    def perimeter(self):
        perim = 0
        for i in range(len(self.points)):
            x1, y1 = self.points[i]
            x2, y2 = self.points[(i + 1) % len(self.points)]
            perim += abs(x1 - x2) + abs(y1 - y2)

        return perim


lines = [line.strip() for line in open("input/day18.in")]

polygon = Polygon(points=[])
vertex = (0, 0)
for line in lines:
    polygon.add(vertex)

    dir, steps, color = line.split()
    color = color[1:-1]
    steps = int('0x' + color[1:-1], base=16)
    dir = color[-1].translate(str.maketrans("0123", "RDLU"))

    if dir == 'L':
        vertex = (vertex[0] - steps, vertex[1])
    elif dir == 'R':
        vertex = (vertex[0] + steps, vertex[1])
    elif dir == 'U':
        vertex = (vertex[0], vertex[1] + steps)
    elif dir == 'D':
        vertex = (vertex[0], vertex[1] - steps)


area = polygon.area()
perim = polygon.perimeter()
ins = area - perim // 2 + 1
print(ins + perim)
