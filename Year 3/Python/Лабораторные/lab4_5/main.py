from abc import ABC,abstractmethod
class ImpossibleShapeException(Exception):
    pass

class Shape(ABC):
    def __init__(self):
        self.name = "Shape"
    @abstractmethod
    def area(self):
        return 0
    @staticmethod
    def print():
        print("Hello!")
    def __add__(self, o):
        self.area() + o.area()

class Square(Shape):
    def __init__(self, side):
        self.name = "Square"
        self.side = side
    def area(self):
        return self.side ** 2

class Triangle(Shape):
    def __init__(self, is_equi=False, a=None, b=None, c=None):
        self.name = "Triangle"
        self.side = a
        self.a = a
        self.b = b
        self.c = c
        if not is_equi:
            self.type = "Not equilateral"
        else:
            self.type = "Equilateral"

    def area(self):
        if self.type == "Equilateral":
            try:
                return self.__equilateral_area()
            except AssertionError:
                print("Error: Side length is negative")
            finally:
                print("Finally block executed")
        elif self.type == "Not equilateral":
            try:
                 return self.__non_equilateral_area()
            except AssertionError:
                 print("Error: Area is <= zero")
            except ImpossibleShapeException:
                print("Error: This triangle is impossible")
            except Exception:
                print("Error: General error")
            finally:
                print("Finally block executed")
    def __equilateral_area(self):
        assert self.side >= 0
        return (3 ** 1/2) / 4 * (self.side ** 2)
    def __non_equilateral_area(self):
        if ((self.a + self.b < self.c) 
            or (self.a + self.c < self.b) 
            or (self.b + self.c < self.a)):
            raise ImpossibleShapeException
        s = (self.a + self.b + self.c) / 2
        sa = s - self.a
        sb = s - self.b
        sc = s - self.c
        area = (s * sa * sb * sc) ** (1/2)
        assert area > 0
        return area

class Circle(Shape):
    def __init__(self, radius):
        self.name = "Circle"
        self.radius = radius
    def area(self):
        return 3.14 * (self.radius ** 2)

class NestedCircles(object):
    def __init__(self, lhs_radius, rhs_radius):
        self.lhs = Circle(lhs_radius)
        self.rhs = Circle(lhs_radius)
    def area_sub(self):
        return self.lhs.area() - self.rhs.area()

def main():
    # shapes = Shape()
    square = Square(5)
    print(square.area())
    Shape.print()
    equi = Triangle(True, 30)
    nonequi = Triangle(False, 30, 40, 50)
    print(equi.area())
    print(nonequi.area())
    two_shapes = NestedCircles(30, 30)
    print(two_shapes.area_sub())

    exception_test()

def exception_test():
    triangle = Triangle(False, 1, 10, 12)
    print(triangle.area())
    tri = Triangle(False, 0, 0, 0)
    print(tri.area())
    neg = Triangle(True, -10)
    print(neg.area())
    
if __name__ == "__main__":
    main()
