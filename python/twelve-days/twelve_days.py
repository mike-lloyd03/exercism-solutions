def recite(start_verse, end_verse):
    result = []

    for i in range(start_verse - 1, end_verse):
        verse_str = (
            f"On the {verses[i]['num']} day of Christmas my true love gave to me: "
        )

        for j in range(i, -1, -1):
            if i != 0 and j == 0:
                verse_str += 'and '
            verse_str += verses[j]['item']
            if j == 0:
                verse_str += '.'
            else:
                verse_str += ', '
        result.append(verse_str)

    return result


verses = [
    {'num': 'first', 'item': 'a Partridge in a Pear Tree'},
    {'num': 'second', 'item': 'two Turtle Doves'},
    {'num': 'third', 'item': 'three French Hens'},
    {'num': 'fourth', 'item': 'four Calling Birds'},
    {'num': 'fifth', 'item': 'five Gold Rings'},
    {'num': 'sixth', 'item': 'six Geese-a-Laying'},
    {'num': 'seventh', 'item': 'seven Swans-a-Swimming'},
    {'num': 'eighth', 'item': 'eight Maids-a-Milking'},
    {'num': 'ninth', 'item': 'nine Ladies Dancing'},
    {'num': 'tenth', 'item': 'ten Lords-a-Leaping'},
    {'num': 'eleventh', 'item': 'eleven Pipers Piping'},
    {'num': 'twelfth', 'item': 'twelve Drummers Drumming'},
]
