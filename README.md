# Point
1. `local` position is ALWAYS in Direction::Odd,
   - `displacement > 0` means point is moving in Direction::Odd
   - `displacement < 0` means point is moving in Direction::Even

# Section
1. `global` position is ALWAYS in the ''left'',
    - If a section `direction` is Direction::Odd milepost at `global` is lower than milepost at the "odd end" of the section
    - If a section `direction` is Direction::Even milepost at `global` is higher than milepost at the "odd end" of the section