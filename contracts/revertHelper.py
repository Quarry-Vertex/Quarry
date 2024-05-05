def revert_message_to_characters(revert_message):
    # Remove the leading '0x' if present
    if revert_message.startswith("0x"):
        revert_message = revert_message[2:]

    # Split the string into chunks of 2 characters
    chunks = [revert_message[i:i+2] for i in range(0, len(revert_message), 2)]

    # Convert each chunk into its corresponding ASCII character
    characters = [chr(int(chunk, 16)) for chunk in chunks]

    # Join the characters to form the message
    return ''.join(characters)

# Example usage
revert_message = "0x08c379a0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000185375626d697474656420626c6f636b206973207374616c650000000000000000"
decoded_message = revert_message_to_characters(revert_message)
print(decoded_message)
