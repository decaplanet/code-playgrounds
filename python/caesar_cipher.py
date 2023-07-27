logo = """           
 ,adPPYba, ,adPPYYba,  ,adPPYba, ,adPPYba, ,adPPYYba, 8b,dPPYba,  
a8"     "" ""     `Y8 a8P_____88 I8[    "" ""     `Y8 88P'   "Y8  
8b         ,adPPPPP88 8PP"""""""  `"Y8ba,  ,adPPPPP88 88          
"8a,   ,aa 88,    ,88 "8b,   ,aa aa    ]8I 88,    ,88 88          
 `"Ybbd8"' `"8bbdP"Y8  `"Ybbd8"' `"YbbdP"' `"8bbdP"Y8 88   
            88             88                                 
           ""             88                                 
                          88                                 
 ,adPPYba, 88 8b,dPPYba,  88,dPPYba,   ,adPPYba, 8b,dPPYba,  
a8"     "" 88 88P'    "8a 88P'    "8a a8P_____88 88P'   "Y8  
8b         88 88       d8 88       88 8PP""""""" 88          
"8a,   ,aa 88 88b,   ,a8" 88       88 "8b,   ,aa 88          
 `"Ybbd8"' 88 88`YbbdP"'  88       88  `"Ybbd8"' 88          
              88                                             
              88           
"""

alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
            'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']


def caesar(start_text, shift_amount, cipher_direction):

    # py: int | str
    int_str_list = []

    if cipher_direction == "decode":
        shift_amount *= -1

    for char in start_text:
        if char not in alphabet:
            int_str_list.append(char)
        else:
            position = alphabet.index(char) + shift_amount
            int_str_list.append(position)

    end_text = ""

    for item in int_str_list:
        if type(item) == str:
            end_text += item
        else:
            if cipher_direction == "encode" and item > len(alphabet) - 1:
                final_index = item

                while final_index > len(alphabet) - 1:
                    final_index -= len(alphabet)

                end_text += alphabet[final_index]
            elif cipher_direction == "decode" and item < 0:
                final_index = item

                while final_index < 0:
                    final_index += len(alphabet)

                end_text += alphabet[final_index]
            elif not item < 0 and not item > len(alphabet) - 1:
                end_text += alphabet[item]

    print(f"The {cipher_direction}d text is \"{end_text}\".")


print(logo)

should_continue = True
while should_continue:
    direction = input("Type 'encode' to encrypt, type 'decode' to decrypt:\n")
    text = input("Type your message:\n").lower()
    shift = int(input("Type the shift number:\n"))

    caesar(start_text=text, shift_amount=shift, cipher_direction=direction)

    direction = input("Do you want to restart the cipher program? (y/n)\n")

    if direction != "y" or direction != "yes":
        should_continue = False
