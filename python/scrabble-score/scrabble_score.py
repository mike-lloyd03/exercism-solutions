def score(word):
    score = 0

    for letter in word.lower():
        if letter in 'aeioulnrst':
            score += 1
        elif letter in 'dg':
            score += 2
        elif letter in 'bcmp':
            score += 3
        elif letter in 'fhvwy':
            score += 4
        elif letter == 'k':
            score += 5
        elif letter in 'jx':
            score += 8
        elif letter in 'qz':
            score += 10

    return score
