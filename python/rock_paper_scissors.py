import random
from enum import Enum


class Selection(Enum):
    ROCK = 0
    PAPER = 1
    SCISSORS = 2


rock = '''
    _______
---'   ____)
      (_____)
      (_____)
      (____)
---.__(___)
'''

paper = '''
    _______
---'   ____)____
          ______)
          _______)
         _______)
---.__________)
'''

scissors = '''
    _______
---'   ____)____
          ______)
       __________)
      (____)
---.__(___)
'''

ASCII_ARTS = [rock, paper, scissors]
RESULTS = [
    ["It's a draw", "You lose", "You win"],
    ["You win", "It's a draw", "You lose"],
    ["You lose", "You win", "It's a draw"]
]


def rock_paper_scissors():
    user_selection = int(input(
        "What do you choose? Type 0 for Rock, 1 for Paper or 2 for Scissors. "))
    comparing_value = random.randint(0, 2)

    if user_selection in [0, 1, 2]:
        print(ASCII_ARTS[user_selection])
        print(f"Your computer chose:\n{ASCII_ARTS[comparing_value]}")
        print(RESULTS[user_selection][comparing_value])
        if RESULTS[user_selection][comparing_value] == "It's a draw":
            rock_paper_scissors()
    else:
        print("ERROR: INVALID INPUT")
        rock_paper_scissors()


rock_paper_scissors()
