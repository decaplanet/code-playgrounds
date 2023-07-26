import os

logo = '''
                         ___________
                         \         /
                          )_______(
                          |"""""""|_.-._,.---------.,_.-._
                          |       | | |               | | ''-.
                          |       |_| |_             _| |_..-'
                          |_______| '-' `'---------'` '-'
                          )"""""""(
                         /_________\\
                       .-------------.
                      /_______________\\
'''
bids = {}

is_continue = True
is_first_time = True
print(logo)

while is_continue:
    if not is_first_time:
        os.system('clear')

    name = input("What's your name? :\n")
    bid = input("What's your bid? :\n$")

    try:
        bids[name] = float(bid)
    except:
        should_continue = input(
            "Input was invalid. Enter any letters to try again. :\n")
        continue

    should_continue = input(
        "Are there any other bidders? Type 'yes' or 'no'. :\n")
    if should_continue == "no":
        is_continue = False

    is_first_time = False


def find_highest_bidder(bidding_record: dict[str, float]):
    winner = ""
    largest_bid = None

    for bidder in bidding_record:
        bid_amount = bids[bidder]

        if largest_bid == None:
            winner = bidder
            largest_bid = bid_amount
        else:
            if bid_amount > largest_bid:
                winner = bidder
                largest_bid = bids[bidder]

    return (winner, largest_bid)


largest = find_highest_bidder(bids)

if largest != None:
    print(f"\nThe winner is {largest[0]} with a bid of ${largest[1]}")
