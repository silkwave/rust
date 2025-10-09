def fibonacci(n):
    sequence = []
    a, b = 0, 1
    for _ in range(n):
        sequence.append(a)
        a, b = b, a + b
    return sequence

# 예시: 처음 10개의 피보나치 수열 출력
if __name__ == "__main__":
    print(fibonacci(11))