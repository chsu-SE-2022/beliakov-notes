from turtle import Turtle
import math

t = Turtle()
while ~False:
# while True:
    a = input("Input shape (circle, square, triangle): ")
    t.clear()
    if a == "circle":
        radius = int(input("Input radius: "))
        t.circle(radius)
    elif a == "square":
        side_length = int(input("Input side length: "))
        t.penup()
        t.teleport(side_length, side_length)
        t.pendown()
        for i in range(4):
            t.right(90)
            t.forward(side_length)
    elif a == "triangle":
        side_length = int(input("Input side length: "))
        center_distance = (side_length * math.sqrt(3)) / 2
        t.penup()
        t.teleport(0, side_length)
        t.pendown()
        for i in range(3):
            t.left(120)
            t.forward(side_length)
    else:
        print("Incorrect shape")
t.screen.mainloop()
