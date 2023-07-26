import os

logo = """
 _____________________
|  _________________  |
| | Pythonista   0. | |  .----------------.  .----------------.  .----------------.  .----------------. 
| |_________________| | | .--------------. || .--------------. || .--------------. || .--------------. |
|  ___ ___ ___   ___  | | |     ______   | || |      __      | || |   _____      | || |     ______   | |
| | 7 | 8 | 9 | | + | | | |   .' ___  |  | || |     /  \     | || |  |_   _|     | || |   .' ___  |  | |
| |___|___|___| |___| | | |  / .'   \_|  | || |    / /\ \    | || |    | |       | || |  / .'   \_|  | |
| | 4 | 5 | 6 | | - | | | |  | |         | || |   / ____ \   | || |    | |   _   | || |  | |         | |
| |___|___|___| |___| | | |  \ `.___.'\  | || | _/ /    \ \_ | || |   _| |__/ |  | || |  \ `.___.'\  | |
| | 1 | 2 | 3 | | x | | | |   `._____.'  | || ||____|  |____|| || |  |________|  | || |   `._____.'  | |
| |___|___|___| |___| | | |              | || |              | || |              | || |              | |
| | . | 0 | = | | / | | | '--------------' || '--------------' || '--------------' || '--------------' |
| |___|___|___| |___| |  '----------------'  '----------------'  '----------------'  '----------------' 
|_____________________|
"""


# Add
def add(num1, num2):
    return num1 + num2


# Subtract
def subtract(num1, num2):
    return num1 - num2


# Multiply
def multiply(num1, num2):
    return num1 * num2


# Divide
def divide(num1, num2):
    return num1 / num2


operations = {
    "+": add,
    "-": subtract,
    "*": multiply,
    "/": divide
}


def calculator():
    print(logo)

    num1 = float(input("What's the first number? : "))
    should_continue = True

    while should_continue:

        for symbol in operations:
            print(symbol)
        operations_symbol = None
        while operations_symbol not in list(operations.keys()):
            operations_symbol = input(
                "Pick an operation from the line above: ")

        num2 = float(input("What's the second number? : "))

        calculation_function = operations[operations_symbol]
        answer = calculation_function(num1, num2)

        print(f"{num1} {operations_symbol} {num2} = {answer}")

        should_continue = input(
            f"Type 'y' to continue calculating with {answer}, or type 'n' to start a new calculation. : ")

        if should_continue == "n":
            num1 = 0
            should_reassign_num1 = False

            os.system('clear')
            calculator()

        else:
            num1 = answer


calculator()
