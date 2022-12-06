# Parse the input and create a list of Calories for each Elf
calories = []
current_elf_calories = []
with open('input.txt') as f:
    for line in f:
        line = line.strip()
        if line == '':
            # New Elf
            calories.append(current_elf_calories)
            current_elf_calories = []
        else:
            # Add Calories for current Elf
            current_elf_calories.append(int(line))

# Add the last Elf's Calories
calories.append(current_elf_calories)

# Find the Elf with the most Calories
max_calories = 0
for elf_calories in calories:
    total_calories = sum(elf_calories)
    if total_calories > max_calories:
        max_calories = total_calories

# Print the result
print('Elf carrying the most Calories:', max_calories)
