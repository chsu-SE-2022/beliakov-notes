from collections import *
import timeit

def main():
    with open("./input.txt") as f:
        numbers = f.readline().strip().split(' ')
        deq = deque(numbers)
        count = Counter(numbers)

        demo_deque(deq)
        demo_counter(count)
        # timer()
    
def demo_deque(deq: deque):
    print(f'Deque: {deq}')

    deq.append('10')
    print(f'Appended 10 to deque: {deq}')

    deq.appendleft('0')
    print(f'Prepend 0 to deque: {deq}')
    
    pop = deq.pop()
    print(f'Pop {pop}, deque: {deq}')
    
    leftpop = deq.popleft()
    print(f'Popleft {leftpop}, deque: {deq}')

    elem_to_search = '2'
    count = deq.count(elem_to_search)
    print(f'Found {count} of {elem_to_search}')

    index = deq.index(elem_to_search)
    print(f'Index of {elem_to_search} is {index}')

    inserted = '727'
    insertion_idx = 3
    deq.insert(insertion_idx, inserted)
    print(f'Inserted {inserted} at idx {insertion_idx}, deque: {deq}')

    deq.remove(inserted)
    print(f'removed {inserted}, deque: {deq}')

    deq.reverse()
    print(f'inverted deque: {deq}')

    deq.rotate(1)
    print(f'rotated deque by 1: {deq}')

    with open("./deque.txt", "w") as f:
        f.write(' '.join(deq))

def demo_counter(count: Counter):
    print(f'counter: {count}')

    print(f'Elements of count: {list(count.elements())}')

    print(f'three most common elements in counter: {count.most_common(3)}')

    print(f'Total count sum: {count.total()}')

    with open("./counter.txt", "w") as f:
        f.write(' '.join(count))

def timer():
    numbers = []
    for i in range(10000):
        numbers.append(i)
    count = Counter(numbers)
    deq = deque(numbers)
    print(f'Count clear: {timeit.timeit(lambda: count.clear(), number=10000) * 1000:.3f}ms')
    print(f'Deque clear: {timeit.timeit(lambda: deq.clear(), number=10000)}')

    print(f'Count extend: {timeit.timeit(lambda: count.update(numbers), number=10000)}')
    print(f'Deque extend: {timeit.timeit(lambda: deq.extend(numbers), number=10000)}')

    print(f'Count find: {timeit.timeit(lambda: count.get(10), number=10000)}')
    print(f'Deque find: {timeit.timeit(lambda: deq.index(10), number=10000)}')


if __name__ == "__main__":
    main()
