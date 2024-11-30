import os


# 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

def create_file_if_not_exists(filename: str) -> None:
    if not os.path.exists(filename):
        with open(filename, "w") as file:
            pass

def read_last_number_from_file(filename: str) -> int:
    try:
        with open(filename, "r") as file:
            lines = file.readlines()
            non_empty_lines = [line.strip() for line in lines if line.strip()]
            if non_empty_lines:
                return int(non_empty_lines[-1])
            else:
                print(f"File {filename} is empty.")
                return None
    except FileNotFoundError:
        print(f"File {filename} does not exist.")
        return None
    except ValueError:
        print(f"Last line in {filename} is not a valid number.")
        return None

def product(input: int) -> int:
    result: int = 1

    # get each digit by mod instead of string conversion
    while input > 0:
        result *= input % 10
        input //= 10

    return result

def get_multiplicative_persistence(user_input: int) -> int:
    steps: int = 0

    # 10 is the smallest double-digit number
    while user_input >= 10:
        user_input: int = product(user_input)
        steps += 1

    return steps

def append_to_file(filename: str, data: int):
    with open(filename, "a") as file:
        file.write(f"{data}\n")

def main():
    filename: str = "11.txt"
    create_file_if_not_exists(filename)
    number: int = read_last_number_from_file("11.txt") or 0

    while True:
        multiplicative_persistence: int = get_multiplicative_persistence(number)

        print(f"{number}", end="\r")

        if multiplicative_persistence == 11:
            append_to_file(filename, number)

        number += 1


if __name__ == "__main__":
    main()
