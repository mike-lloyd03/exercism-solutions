import re


def count_words(sentence):
    pattern = r"[a-zA-Z0-9]+('[a-zA-Z]+)?"

    words_dict = {}
    for word in re.finditer(pattern, sentence.lower()):
        words_dict.update({word[0]: (words_dict.get(word[0], 0) + 1)})

    return words_dict
