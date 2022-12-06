# Parse the input and create a list of Calories for each Elf
calories = []
current_elf_calories = []
with open('input.txt') as f:
    for line in f:
        line = line.strip()
        if line == '':
            # New Elf
            calories.append(sum(current_elf_calories))
            current_elf_calories = []
        else:
            # Add Calories for current Elf
            current_elf_calories.append(int(line))

# Add the last Elf's Calories
calories.append(sum(current_elf_calories))

# Sort the Elves' Calories in descending order
calories.sort(reverse=True)

# Calculate the total Calories carried by the top three Elves
top_three_calories = sum(calories[:3])

# Print the result
print('Total Calories carried by the top three Elves:', top_three_calories)
