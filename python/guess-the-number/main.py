from art import logo
import random

print(logo)

print("Welcome to the Number Guessing Game!")
print("I'm thinking of a number between 1 and 100.")

difficulty = None

answer = random.randint(0, 100)
print(f"Pssst, the correct answer is {answer}.")
while difficulty == None:
    difficulty_input = input("Choose a difficulty. Type 'easy' or 'hard': ")

    if difficulty_input == "easy" or difficulty_input == "hard":
        difficulty = difficulty_input
    else:
        print("Input is invalid! Try again!")

if difficulty_input == "easy":
    remaining_attempt = 10
elif difficulty_input == "hard":
    remaining_attempt = 5

is_successful = False

while remaining_attempt != None and remaining_attempt > 0:
    print(
        f"You have {remaining_attempt} attempts remaining to guess the number.")
    user_guess = int(input("Make a guess: "))

    if user_guess > answer:
        print("Too High.")
    elif user_guess < answer:
        print("Too Low.")
    elif user_guess == answer:
        is_successful = True
        remaining_attempt = 0

if remaining_attempt == 0:
    if is_successful:
        print(f"You've got it! The answer is {answer}!")
    else:
        print(f"You've run out of guesses, you lose.")
        print(f"Here's the answer by the way! {answer}")
