def is_isogram(string):
    unique_chars = []

    for c in string.lower():
        if c in ' -':
            continue
        if c in unique_chars:
            return False
        unique_chars.append(c)

    return True

    # # Alternatively
    # trim_string = string.lower().replace(' ', '').replace('-', '')
    # return len(trim_string) == len(set(trim_string))
