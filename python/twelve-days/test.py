from twelve_days import recite

expected = [
    "On the eighth day of Christmas my true love gave to me: "
    "eight Maids-a-Milking, "
    "seven Swans-a-Swimming, "
    "six Geese-a-Laying, "
    "five Gold Rings, "
    "four Calling Birds, "
    "three French Hens, "
    "two Turtle Doves, "
    "and a Partridge in a Pear Tree."
]

print([recite(n, n)[0] for n in range(1, 4)])
print()
print(recite(1, 4))
